//! HTTP/HTTPS Server module for A-lang
//! Express.js-like web framework for building APIs and web applications

use crate::interpreter::value::Value;
use axum::{
    extract::{Json, Path, Query, State},
    http::{HeaderMap, Method, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{delete, get, post, put},
    Router,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

/// HTTP Request wrapper
#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub params: HashMap<String, String>,
}

impl HttpRequest {
    pub fn to_value(&self) -> Value {
        let mut req = HashMap::new();
        req.insert("method".to_string(), Value::String(self.method.clone()));
        req.insert("path".to_string(), Value::String(self.path.clone()));
        req.insert("body".to_string(), Value::String(self.body.clone()));

        // Query parameters
        let mut query_obj = HashMap::new();
        for (k, v) in &self.query {
            query_obj.insert(k.clone(), Value::String(v.clone()));
        }
        req.insert("query".to_string(), Value::Object(query_obj));

        // Headers
        let mut headers_obj = HashMap::new();
        for (k, v) in &self.headers {
            headers_obj.insert(k.clone(), Value::String(v.clone()));
        }
        req.insert("headers".to_string(), Value::Object(headers_obj));

        // Route parameters
        let mut params_obj = HashMap::new();
        for (k, v) in &self.params {
            params_obj.insert(k.clone(), Value::String(v.clone()));
        }
        req.insert("params".to_string(), Value::Object(params_obj));

        Value::Object(req)
    }
}

/// HTTP Response builder
#[derive(Debug, Clone)]
pub struct HttpResponseBuilder {
    status: u16,
    headers: HashMap<String, String>,
    body: String,
}

impl HttpResponseBuilder {
    pub fn new() -> Self {
        Self {
            status: 200,
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    pub fn status(mut self, code: u16) -> Self {
        self.status = code;
        self
    }

    pub fn header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    pub fn json(mut self, value: &Value) -> Self {
        self.body = value.as_string();
        self.headers
            .insert("Content-Type".to_string(), "application/json".to_string());
        self
    }

    pub fn text(mut self, text: String) -> Self {
        self.body = text;
        self.headers
            .insert("Content-Type".to_string(), "text/plain".to_string());
        self
    }

    pub fn html(mut self, html: String) -> Self {
        self.body = html;
        self.headers
            .insert("Content-Type".to_string(), "text/html".to_string());
        self
    }

    pub fn build(self) -> (u16, HashMap<String, String>, String) {
        (self.status, self.headers, self.body)
    }
}

impl Default for HttpResponseBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Route handler type
pub type RouteHandler = Arc<dyn Fn(HttpRequest) -> HttpResponseBuilder + Send + Sync>;

/// Middleware type
pub type MiddlewareHandler =
    Arc<dyn Fn(HttpRequest) -> Result<HttpRequest, HttpResponseBuilder> + Send + Sync>;

/// HTTP Server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub enable_cors: bool,
    pub enable_logging: bool,
    pub static_dir: Option<String>,
}

impl ServerConfig {
    pub fn new(host: String, port: u16) -> Self {
        Self {
            host,
            port,
            enable_cors: true,
            enable_logging: true,
            static_dir: None,
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self::new("127.0.0.1".to_string(), 3000)
    }
}

/// HTTP Server (Express-like)
pub struct HttpServer {
    config: ServerConfig,
    routes: Arc<Mutex<HashMap<(String, String), RouteHandler>>>, // (method, path) -> handler
    middlewares: Arc<Mutex<Vec<MiddlewareHandler>>>,
    runtime: Option<Runtime>,
}

impl HttpServer {
    pub fn new(config: ServerConfig) -> Self {
        Self {
            config,
            routes: Arc::new(Mutex::new(HashMap::new())),
            middlewares: Arc::new(Mutex::new(Vec::new())),
            runtime: None,
        }
    }

    /// Register a route handler
    pub fn route(&mut self, method: &str, path: &str, handler: RouteHandler) -> Result<(), String> {
        let key = (method.to_uppercase(), path.to_string());
        self.routes.lock().unwrap().insert(key, handler);
        Ok(())
    }

    /// GET route
    pub fn get(&mut self, path: &str, handler: RouteHandler) -> Result<(), String> {
        self.route("GET", path, handler)
    }

    /// POST route
    pub fn post(&mut self, path: &str, handler: RouteHandler) -> Result<(), String> {
        self.route("POST", path, handler)
    }

    /// PUT route
    pub fn put(&mut self, path: &str, handler: RouteHandler) -> Result<(), String> {
        self.route("PUT", path, handler)
    }

    /// DELETE route
    pub fn delete(&mut self, path: &str, handler: RouteHandler) -> Result<(), String> {
        self.route("DELETE", path, handler)
    }

    /// Add middleware
    pub fn use_middleware(&mut self, middleware: MiddlewareHandler) {
        self.middlewares.lock().unwrap().push(middleware);
    }

    /// Start the server
    pub fn listen(&mut self) -> Result<(), String> {
        let addr: SocketAddr = self
            .config
            .address()
            .parse()
            .map_err(|e| format!("Invalid address: {}", e))?;

        // Create Tokio runtime
        let runtime = Runtime::new().map_err(|e| format!("Failed to create runtime: {}", e))?;

        let routes = self.routes.clone();
        let middlewares = self.middlewares.clone();
        let config = self.config.clone();

        runtime.block_on(async move {
            let app = Self::build_router(routes, middlewares, config);

            let listener = tokio::net::TcpListener::bind(addr)
                .await
                .map_err(|e| format!("Failed to bind: {}", e))?;

            println!("🚀 Server running on http://{}", addr);

            axum::serve(listener, app)
                .await
                .map_err(|e| format!("Server error: {}", e))?;

            Ok::<(), String>(())
        })?;

        self.runtime = Some(runtime);
        Ok(())
    }

    fn build_router(
        _routes: Arc<Mutex<HashMap<(String, String), RouteHandler>>>,
        _middlewares: Arc<Mutex<Vec<MiddlewareHandler>>>,
        config: ServerConfig,
    ) -> Router {
        let mut app = Router::new();

        // Add CORS if enabled
        if config.enable_cors {
            let cors = CorsLayer::new()
                .allow_methods(Any)
                .allow_headers(Any)
                .allow_origin(Any);
            app = app.layer(cors);
        }

        // Static files
        if let Some(static_dir) = config.static_dir {
            app = app.nest_service("/static", ServeDir::new(static_dir));
        }

        // Fallback handler for all routes
        app = app.fallback(|req: axum::extract::Request| async move {
            let method = req.method().to_string();
            let path = req.uri().path().to_string();

            // Simple response for now
            let response = HttpResponseBuilder::new()
                .status(404)
                .text(format!("Route not found: {} {}", method, path))
                .build();

            (
                StatusCode::from_u16(response.0).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                response.2,
            )
                .into_response()
        });

        app
    }

    /// Stop the server
    pub fn stop(&mut self) {
        if let Some(runtime) = self.runtime.take() {
            runtime.shutdown_background();
        }
    }
}

impl Drop for HttpServer {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Express-like Application
pub struct ExpressApp {
    router: Router,
    config: ServerConfig,
}

impl ExpressApp {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            config: ServerConfig::default(),
        }
    }

    pub fn with_config(config: ServerConfig) -> Self {
        Self {
            router: Router::new(),
            config,
        }
    }

    /// Add a GET route
    pub fn get<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, ()>,
        T: 'static,
    {
        self.router = self.router.route(path, get(handler));
        self
    }

    /// Add a POST route
    pub fn post<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, ()>,
        T: 'static,
    {
        self.router = self.router.route(path, post(handler));
        self
    }

    /// Add a PUT route
    pub fn put<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, ()>,
        T: 'static,
    {
        self.router = self.router.route(path, put(handler));
        self
    }

    /// Add a DELETE route
    pub fn delete<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, ()>,
        T: 'static,
    {
        self.router = self.router.route(path, delete(handler));
        self
    }

    /// Enable CORS
    pub fn cors(mut self) -> Self {
        let cors = CorsLayer::new()
            .allow_methods(Any)
            .allow_headers(Any)
            .allow_origin(Any);
        self.router = self.router.layer(cors);
        self
    }

    /// Serve static files
    pub fn static_files(mut self, path: &str, dir: &str) -> Self {
        self.router = self.router.nest_service(path, ServeDir::new(dir));
        self
    }

    /// Start listening
    pub async fn listen(self) -> Result<(), String> {
        let addr: SocketAddr = self
            .config
            .address()
            .parse()
            .map_err(|e| format!("Invalid address: {}", e))?;

        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(|e| format!("Failed to bind: {}", e))?;

        println!("🚀 Server running on http://{}", addr);

        axum::serve(listener, self.router)
            .await
            .map_err(|e| format!("Server error: {}", e))?;

        Ok(())
    }
}

impl Default for ExpressApp {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple handler functions
pub async fn hello_handler() -> &'static str {
    "Hello, World!"
}

pub async fn json_handler() -> Json<HashMap<String, String>> {
    let mut map = HashMap::new();
    map.insert("message".to_string(), "Hello from A-lang!".to_string());
    Json(map)
}

/// Request/Response utilities
pub struct RequestUtils;

impl RequestUtils {
    /// Parse JSON body
    pub fn parse_json(body: &str) -> Result<Value, String> {
        serde_json::from_str(body)
            .map(|v: serde_json::Value| Self::json_value_to_alang(&v))
            .map_err(|e| format!("JSON parse error: {}", e))
    }

    /// Convert serde_json::Value to A-lang Value
    fn json_value_to_alang(value: &serde_json::Value) -> Value {
        match value {
            serde_json::Value::Null => Value::Nil,
            serde_json::Value::Bool(b) => Value::Boolean(*b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Value::Integer(i)
                } else if let Some(f) = n.as_f64() {
                    Value::Float(f)
                } else {
                    Value::Nil
                }
            }
            serde_json::Value::String(s) => Value::String(s.clone()),
            serde_json::Value::Array(arr) => {
                Value::Array(arr.iter().map(Self::json_value_to_alang).collect())
            }
            serde_json::Value::Object(obj) => {
                let mut map = HashMap::new();
                for (k, v) in obj {
                    map.insert(k.clone(), Self::json_value_to_alang(v));
                }
                Value::Object(map)
            }
        }
    }

    /// Convert A-lang Value to JSON string
    pub fn stringify_json(value: &Value) -> Result<String, String> {
        let json_value = Self::alang_value_to_json(value);
        serde_json::to_string(&json_value).map_err(|e| format!("JSON stringify error: {}", e))
    }

    fn alang_value_to_json(value: &Value) -> serde_json::Value {
        match value {
            Value::Nil => serde_json::Value::Null,
            Value::Boolean(b) => serde_json::Value::Bool(*b),
            Value::Integer(i) => serde_json::Value::Number((*i).into()),
            Value::Float(f) => serde_json::Value::Number(
                serde_json::Number::from_f64(*f).unwrap_or_else(|| 0.into()),
            ),
            Value::String(s) => serde_json::Value::String(s.clone()),
            Value::Array(arr) => {
                serde_json::Value::Array(arr.iter().map(Self::alang_value_to_json).collect())
            }
            Value::Object(obj) => {
                let mut map = serde_json::Map::new();
                for (k, v) in obj {
                    map.insert(k.clone(), Self::alang_value_to_json(v));
                }
                serde_json::Value::Object(map)
            }
            _ => serde_json::Value::Null,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_config() {
        let config = ServerConfig::new("0.0.0.0".to_string(), 8080);
        assert_eq!(config.address(), "0.0.0.0:8080");
    }

    #[test]
    fn test_response_builder() {
        let response = HttpResponseBuilder::new()
            .status(201)
            .header("X-Custom".to_string(), "value".to_string())
            .text("Hello".to_string())
            .build();

        assert_eq!(response.0, 201);
        assert_eq!(response.1.get("X-Custom").unwrap(), "value");
        assert_eq!(response.2, "Hello");
    }

    #[test]
    fn test_json_conversion() {
        let json = r#"{"name": "Alice", "age": 30}"#;
        let value = RequestUtils::parse_json(json).unwrap();

        match value {
            Value::Object(obj) => {
                assert_eq!(
                    obj.get("name").unwrap(),
                    &Value::String("Alice".to_string())
                );
                assert_eq!(obj.get("age").unwrap(), &Value::Integer(30));
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_http_request_to_value() {
        let req = HttpRequest {
            method: "GET".to_string(),
            path: "/api/users".to_string(),
            query: {
                let mut q = HashMap::new();
                q.insert("id".to_string(), "123".to_string());
                q
            },
            headers: HashMap::new(),
            body: String::new(),
            params: HashMap::new(),
        };

        let value = req.to_value();
        match value {
            Value::Object(obj) => {
                assert_eq!(
                    obj.get("method").unwrap(),
                    &Value::String("GET".to_string())
                );
                assert_eq!(
                    obj.get("path").unwrap(),
                    &Value::String("/api/users".to_string())
                );
            }
            _ => panic!("Expected object"),
        }
    }
}
