//! Database module for A-lang
//! Provides MySQL database connectivity with connection pooling

use crate::interpreter::value::Value;
use mysql_async::prelude::*;
use mysql_async::{Pool, PoolConstraints, PoolOpts, Row, Value as MySqlValue};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::runtime::Runtime;

/// Database connection configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    pub pool_min: usize,
    pub pool_max: usize,
}

impl DatabaseConfig {
    pub fn new(host: String, port: u16, user: String, password: String, database: String) -> Self {
        Self {
            host,
            port,
            user,
            password,
            database,
            pool_min: 1,
            pool_max: 10,
        }
    }

    pub fn connection_string(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.database
        )
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self::new(
            "localhost".to_string(),
            3306,
            "root".to_string(),
            "".to_string(),
            "test".to_string(),
        )
    }
}

/// Query result
#[derive(Debug, Clone)]
pub struct QueryResult {
    pub rows: Vec<HashMap<String, Value>>,
    pub affected_rows: u64,
    pub last_insert_id: u64,
}

impl QueryResult {
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            affected_rows: 0,
            last_insert_id: 0,
        }
    }

    pub fn to_value(&self) -> Value {
        let mut result = HashMap::new();

        // Convert rows to array of objects
        let rows_array: Vec<Value> = self
            .rows
            .iter()
            .map(|row| Value::Object(row.clone()))
            .collect();

        result.insert("rows".to_string(), Value::Array(rows_array));
        result.insert(
            "affectedRows".to_string(),
            Value::Integer(self.affected_rows as i64),
        );
        result.insert(
            "insertId".to_string(),
            Value::Integer(self.last_insert_id as i64),
        );

        Value::Object(result)
    }
}

impl Default for QueryResult {
    fn default() -> Self {
        Self::new()
    }
}

/// Database connection pool
pub struct Database {
    pool: Pool,
    runtime: Arc<Runtime>,
    config: DatabaseConfig,
}

impl Database {
    /// Create a new database connection pool
    pub fn new(config: DatabaseConfig) -> Result<Self, String> {
        let runtime = Runtime::new().map_err(|e| format!("Failed to create runtime: {}", e))?;

        let pool_opts = PoolOpts::default().with_constraints(
            PoolConstraints::new(config.pool_min, config.pool_max)
                .ok_or_else(|| "Invalid pool constraints".to_string())?,
        );

        let pool = Pool::new(
            mysql_async::OptsBuilder::default()
                .ip_or_hostname(config.host.clone())
                .tcp_port(config.port)
                .user(Some(config.user.clone()))
                .pass(Some(config.password.clone()))
                .db_name(Some(config.database.clone()))
                .pool_opts(pool_opts),
        );

        Ok(Self {
            pool,
            runtime: Arc::new(runtime),
            config,
        })
    }

    /// Execute a query and return results
    pub fn query(&self, sql: &str) -> Result<QueryResult, String> {
        let pool = self.pool.clone();
        let sql = sql.to_string();

        self.runtime.block_on(async move {
            let mut conn = pool
                .get_conn()
                .await
                .map_err(|e| format!("Failed to get connection: {}", e))?;

            let result: Vec<Row> = conn
                .query(sql)
                .await
                .map_err(|e| format!("Query failed: {}", e))?;

            let mut query_result = QueryResult::new();
            query_result.affected_rows = conn.affected_rows();
            query_result.last_insert_id = conn.last_insert_id().unwrap_or(0);

            for row in result {
                let mut row_map = HashMap::new();
                let columns = row.columns();

                for (i, column) in columns.iter().enumerate() {
                    let column_name = column.name_str().to_string();
                    let value = Self::mysql_value_to_alang(&row[i]);
                    row_map.insert(column_name, value);
                }

                query_result.rows.push(row_map);
            }

            Ok(query_result)
        })
    }

    /// Execute a query with parameters (prepared statement)
    pub fn query_params(&self, sql: &str, params: Vec<Value>) -> Result<QueryResult, String> {
        let pool = self.pool.clone();
        let sql = sql.to_string();

        self.runtime.block_on(async move {
            let mut conn = pool
                .get_conn()
                .await
                .map_err(|e| format!("Failed to get connection: {}", e))?;

            // Convert A-lang values to MySQL values
            let mysql_params: Vec<MySqlValue> =
                params.iter().map(Self::alang_value_to_mysql).collect();

            let result: Vec<Row> = conn
                .exec(sql, mysql_params)
                .await
                .map_err(|e| format!("Query failed: {}", e))?;

            let mut query_result = QueryResult::new();
            query_result.affected_rows = conn.affected_rows();
            query_result.last_insert_id = conn.last_insert_id().unwrap_or(0);

            for row in result {
                let mut row_map = HashMap::new();
                let columns = row.columns();

                for (i, column) in columns.iter().enumerate() {
                    let column_name = column.name_str().to_string();
                    let value = Self::mysql_value_to_alang(&row[i]);
                    row_map.insert(column_name, value);
                }

                query_result.rows.push(row_map);
            }

            Ok::<QueryResult, String>(query_result)
        })
    }

    /// Execute an INSERT query
    pub fn insert(&self, table: &str, data: HashMap<String, Value>) -> Result<u64, String> {
        let columns: Vec<String> = data.keys().cloned().collect();
        let placeholders: Vec<String> = (0..columns.len()).map(|_| "?".to_string()).collect();

        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table,
            columns.join(", "),
            placeholders.join(", ")
        );

        let values: Vec<Value> = columns.iter().map(|col| data[col].clone()).collect();

        let result = self.query_params(&sql, values)?;
        Ok(result.last_insert_id)
    }

    /// Execute an UPDATE query
    pub fn update(
        &self,
        table: &str,
        data: HashMap<String, Value>,
        where_clause: &str,
    ) -> Result<u64, String> {
        let set_clauses: Vec<String> = data.keys().map(|col| format!("{} = ?", col)).collect();

        let sql = format!(
            "UPDATE {} SET {} WHERE {}",
            table,
            set_clauses.join(", "),
            where_clause
        );

        let values: Vec<Value> = data.values().cloned().collect();

        let result = self.query_params(&sql, values)?;
        Ok(result.affected_rows)
    }

    /// Execute a DELETE query
    pub fn delete(&self, table: &str, where_clause: &str) -> Result<u64, String> {
        let sql = format!("DELETE FROM {} WHERE {}", table, where_clause);
        let result = self.query(&sql)?;
        Ok(result.affected_rows)
    }

    /// Execute a SELECT query
    pub fn select(
        &self,
        table: &str,
        columns: Vec<String>,
        where_clause: Option<&str>,
    ) -> Result<QueryResult, String> {
        let cols = if columns.is_empty() {
            "*".to_string()
        } else {
            columns.join(", ")
        };

        let sql = if let Some(where_str) = where_clause {
            format!("SELECT {} FROM {} WHERE {}", cols, table, where_str)
        } else {
            format!("SELECT {} FROM {}", cols, table)
        };

        self.query(&sql)
    }

    /// Begin a transaction
    pub fn begin_transaction(&self) -> Result<Transaction, String> {
        Transaction::new(self.pool.clone(), self.runtime.clone())
    }

    /// Test database connection
    pub fn ping(&self) -> Result<bool, String> {
        let pool = self.pool.clone();

        self.runtime.block_on(async move {
            let mut conn = pool
                .get_conn()
                .await
                .map_err(|e| format!("Failed to get connection: {}", e))?;

            conn.ping()
                .await
                .map_err(|e| format!("Ping failed: {}", e))?;

            Ok::<bool, String>(true)
        })
    }

    /// Close all connections
    pub fn close(&self) -> Result<(), String> {
        let pool = self.pool.clone();

        self.runtime.block_on(async move {
            pool.disconnect()
                .await
                .map_err(|e| format!("Failed to close connections: {}", e))?;
            Ok::<(), String>(())
        })
    }

    /// Convert MySQL value to A-lang value
    fn mysql_value_to_alang(value: &MySqlValue) -> Value {
        match value {
            MySqlValue::NULL => Value::Nil,
            MySqlValue::Bytes(bytes) => {
                if let Ok(s) = String::from_utf8(bytes.clone()) {
                    Value::String(s)
                } else {
                    let array: Vec<Value> =
                        bytes.iter().map(|&b| Value::Integer(b as i64)).collect();
                    Value::Array(array)
                }
            }
            MySqlValue::Int(i) => Value::Integer(*i),
            MySqlValue::UInt(u) => Value::Integer(*u as i64),
            MySqlValue::Float(f) => Value::Float(*f as f64),
            MySqlValue::Double(d) => Value::Float(*d),
            MySqlValue::Date(year, month, day, hour, min, sec, _micro) => Value::String(format!(
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
                year, month, day, hour, min, sec
            )),
            MySqlValue::Time(neg, days, hours, minutes, seconds, _micros) => {
                let sign = if *neg { "-" } else { "" };
                Value::String(format!(
                    "{}{} {:02}:{:02}:{:02}",
                    sign, days, hours, minutes, seconds
                ))
            }
        }
    }

    /// Convert A-lang value to MySQL value
    fn alang_value_to_mysql(value: &Value) -> MySqlValue {
        match value {
            Value::Nil => MySqlValue::NULL,
            Value::Boolean(b) => MySqlValue::Int(if *b { 1 } else { 0 }),
            Value::Integer(i) => MySqlValue::Int(*i),
            Value::Float(f) => MySqlValue::Double(*f),
            Value::String(s) => MySqlValue::Bytes(s.as_bytes().to_vec()),
            Value::Array(arr) => {
                // Convert array to JSON string
                let json_str = serde_json::to_string(&arr).unwrap_or_default();
                MySqlValue::Bytes(json_str.as_bytes().to_vec())
            }
            Value::Object(obj) => {
                // Convert object to JSON string
                let json_str = serde_json::to_string(&obj).unwrap_or_default();
                MySqlValue::Bytes(json_str.as_bytes().to_vec())
            }
            _ => MySqlValue::NULL,
        }
    }
}

/// Database transaction
pub struct Transaction {
    pool: Pool,
    runtime: Arc<Runtime>,
    committed: bool,
}

impl Transaction {
    fn new(pool: Pool, runtime: Arc<Runtime>) -> Result<Self, String> {
        let pool_clone = pool.clone();
        let runtime_clone = runtime.clone();

        runtime.block_on(async move {
            let mut conn = pool_clone
                .get_conn()
                .await
                .map_err(|e| format!("Failed to get connection: {}", e))?;

            conn.query_drop("START TRANSACTION")
                .await
                .map_err(|e| format!("Failed to start transaction: {}", e))?;

            Ok::<(), String>(())
        })?;

        Ok(Self {
            pool,
            runtime: runtime_clone,
            committed: false,
        })
    }

    /// Execute a query within the transaction
    pub fn query(&self, sql: &str) -> Result<QueryResult, String> {
        let pool = self.pool.clone();
        let sql = sql.to_string();

        self.runtime.block_on(async move {
            let mut conn = pool
                .get_conn()
                .await
                .map_err(|e| format!("Failed to get connection: {}", e))?;

            let result: Vec<Row> = conn
                .query(sql)
                .await
                .map_err(|e| format!("Query failed: {}", e))?;

            let mut query_result = QueryResult::new();
            query_result.affected_rows = conn.affected_rows();
            query_result.last_insert_id = conn.last_insert_id().unwrap_or(0);

            for row in result {
                let mut row_map = HashMap::new();
                let columns = row.columns();

                for (i, column) in columns.iter().enumerate() {
                    let column_name = column.name_str().to_string();
                    let value = Database::mysql_value_to_alang(&row[i]);
                    row_map.insert(column_name, value);
                }

                query_result.rows.push(row_map);
            }

            Ok(query_result)
        })
    }

    /// Commit the transaction
    pub fn commit(&mut self) -> Result<(), String> {
        let pool = self.pool.clone();

        self.runtime.block_on(async move {
            let mut conn = pool
                .get_conn()
                .await
                .map_err(|e| format!("Failed to get connection: {}", e))?;

            conn.query_drop("COMMIT")
                .await
                .map_err(|e| format!("Failed to commit: {}", e))?;

            Ok::<(), String>(())
        })?;

        self.committed = true;
        Ok(())
    }

    /// Rollback the transaction
    pub fn rollback(&mut self) -> Result<(), String> {
        let pool = self.pool.clone();

        self.runtime.block_on(async move {
            let mut conn = pool
                .get_conn()
                .await
                .map_err(|e| format!("Failed to get connection: {}", e))?;

            conn.query_drop("ROLLBACK")
                .await
                .map_err(|e| format!("Failed to rollback: {}", e))?;

            Ok::<(), String>(())
        })
    }
}

impl Drop for Transaction {
    fn drop(&mut self) {
        if !self.committed {
            let _ = self.rollback();
        }
    }
}

/// Query builder for constructing SQL queries
pub struct QueryBuilder {
    table: String,
    columns: Vec<String>,
    where_clauses: Vec<String>,
    order_by: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
}

impl QueryBuilder {
    pub fn new(table: String) -> Self {
        Self {
            table,
            columns: Vec::new(),
            where_clauses: Vec::new(),
            order_by: None,
            limit: None,
            offset: None,
        }
    }

    pub fn select(mut self, columns: Vec<String>) -> Self {
        self.columns = columns;
        self
    }

    pub fn where_clause(mut self, clause: String) -> Self {
        self.where_clauses.push(clause);
        self
    }

    pub fn order_by(mut self, column: String, direction: &str) -> Self {
        self.order_by = Some(format!("{} {}", column, direction));
        self
    }

    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn build(&self) -> String {
        let cols = if self.columns.is_empty() {
            "*".to_string()
        } else {
            self.columns.join(", ")
        };

        let mut sql = format!("SELECT {} FROM {}", cols, self.table);

        if !self.where_clauses.is_empty() {
            sql.push_str(&format!(" WHERE {}", self.where_clauses.join(" AND ")));
        }

        if let Some(ref order) = self.order_by {
            sql.push_str(&format!(" ORDER BY {}", order));
        }

        if let Some(limit) = self.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = self.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        sql
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_config() {
        let config = DatabaseConfig::new(
            "localhost".to_string(),
            3306,
            "user".to_string(),
            "pass".to_string(),
            "testdb".to_string(),
        );

        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 3306);
        assert_eq!(config.database, "testdb");
    }

    #[test]
    fn test_query_result_to_value() {
        let mut result = QueryResult::new();
        result.affected_rows = 5;
        result.last_insert_id = 123;

        let value = result.to_value();
        match value {
            Value::Object(obj) => {
                assert_eq!(obj.get("affectedRows").unwrap(), &Value::Integer(5));
                assert_eq!(obj.get("insertId").unwrap(), &Value::Integer(123));
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_query_builder() {
        let builder = QueryBuilder::new("users".to_string())
            .select(vec!["id".to_string(), "name".to_string()])
            .where_clause("age > 18".to_string())
            .order_by("name".to_string(), "ASC")
            .limit(10)
            .offset(20);

        let sql = builder.build();
        assert!(sql.contains("SELECT id, name FROM users"));
        assert!(sql.contains("WHERE age > 18"));
        assert!(sql.contains("ORDER BY name ASC"));
        assert!(sql.contains("LIMIT 10"));
        assert!(sql.contains("OFFSET 20"));
    }

    #[test]
    fn test_value_conversions() {
        // Test A-lang to MySQL conversion
        let int_val = Value::Integer(42);
        let mysql_int = Database::alang_value_to_mysql(&int_val);
        assert!(matches!(mysql_int, MySqlValue::Int(42)));

        let string_val = Value::String("hello".to_string());
        let mysql_string = Database::alang_value_to_mysql(&string_val);
        assert!(matches!(mysql_string, MySqlValue::Bytes(_)));

        let nil_val = Value::Nil;
        let mysql_nil = Database::alang_value_to_mysql(&nil_val);
        assert!(matches!(mysql_nil, MySqlValue::NULL));
    }
}
