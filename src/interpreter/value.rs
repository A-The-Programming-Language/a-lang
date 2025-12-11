//! # Value System for A-lang
//!
//! Runtime value representation supporting all A-lang types including
//! reactive values, functions, and complex data structures.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

/// Runtime value representation
#[derive(Clone, Serialize, Deserialize)]
pub enum Value {
    /// Nil/null value
    Nil,

    /// Boolean
    Boolean(bool),

    /// Integer
    Integer(i64),

    /// Floating point
    Float(f64),

    /// String
    String(String),

    /// Array/List
    Array(Vec<Value>),

    /// Object/Hash Map
    Object(HashMap<String, Value>),

    /// Function (can't be serialized properly, placeholder)
    #[serde(skip)]
    Function(Arc<FunctionValue>),

    /// Range
    Range {
        start: i64,
        end: i64,
        inclusive: bool,
    },

    /// Tuple
    Tuple(Vec<Value>),

    /// Struct instance
    Struct {
        name: String,
        fields: HashMap<String, Value>,
    },

    /// Enum variant
    Enum {
        type_name: String,
        variant: String,
        values: Vec<Value>,
    },

    /// Reference to a reactive signal (WOW #2)
    #[serde(skip)]
    ReactiveRef(usize), // NodeId

    /// Quoted code (WOW #3)
    Quote(Box<Value>),

    /// Native Rust function
    #[serde(skip)]
    Native(Arc<NativeFn>),

    /// Future/Promise for async operations
    #[serde(skip)]
    Future(Arc<dyn std::any::Any + Send + Sync>),
}

/// Function value representation
#[derive(Clone)]
pub struct FunctionValue {
    pub name: Option<String>,
    pub parameters: Vec<String>,
    pub body: Vec<crate::ast::Statement>,
    pub closure: HashMap<String, Value>,
    pub is_async: bool,
}

impl fmt::Debug for FunctionValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Function({}, {} params)",
            self.name.as_deref().unwrap_or("<anonymous>"),
            self.parameters.len()
        )
    }
}

impl PartialEq for FunctionValue {
    fn eq(&self, other: &Self) -> bool {
        // Functions are equal if they have the same name and parameter count
        self.name == other.name && self.parameters.len() == other.parameters.len()
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "Nil"),
            Value::Boolean(b) => write!(f, "Boolean({})", b),
            Value::Integer(n) => write!(f, "Integer({})", n),
            Value::Float(fl) => write!(f, "Float({})", fl),
            Value::String(s) => write!(f, "String({:?})", s),
            Value::Array(arr) => write!(f, "Array({:?})", arr),
            Value::Object(obj) => write!(f, "Object({:?})", obj),
            Value::Function(func) => write!(f, "{:?}", func),
            Value::Range {
                start,
                end,
                inclusive,
            } => write!(
                f,
                "Range({:?}..{}{:?})",
                start,
                if *inclusive { "=" } else { "" },
                end
            ),
            Value::Tuple(values) => write!(f, "Tuple({:?})", values),
            Value::Struct { name, fields } => write!(f, "Struct({}, {:?})", name, fields),
            Value::Enum {
                type_name,
                variant,
                values,
            } => write!(f, "Enum({}::{}, {:?})", type_name, variant, values),
            Value::ReactiveRef(id) => write!(f, "ReactiveRef({})", id),
            Value::Quote(val) => write!(f, "Quote({:?})", val),
            Value::Native(_) => write!(f, "Native(<function>)"),
            Value::Future(_) => write!(f, "Future(<pending>)"),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => {
                if a.is_nan() && b.is_nan() {
                    true
                } else {
                    a == b
                }
            }
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Array(a), Value::Array(b)) => a == b,
            (Value::Object(a), Value::Object(b)) => a == b,
            (Value::Function(a), Value::Function(b)) => a == b,
            (
                Value::Range {
                    start: s1,
                    end: e1,
                    inclusive: i1,
                },
                Value::Range {
                    start: s2,
                    end: e2,
                    inclusive: i2,
                },
            ) => s1 == s2 && e1 == e2 && i1 == i2,
            (Value::Tuple(a), Value::Tuple(b)) => a == b,
            (
                Value::Struct {
                    name: n1,
                    fields: f1,
                },
                Value::Struct {
                    name: n2,
                    fields: f2,
                },
            ) => n1 == n2 && f1 == f2,
            (
                Value::Enum {
                    type_name: t1,
                    variant: v1,
                    values: vals1,
                },
                Value::Enum {
                    type_name: t2,
                    variant: v2,
                    values: vals2,
                },
            ) => t1 == t2 && v1 == v2 && vals1 == vals2,
            (Value::ReactiveRef(a), Value::ReactiveRef(b)) => a == b,
            (Value::Quote(a), Value::Quote(b)) => a == b,
            (Value::Native(_), Value::Native(_)) => false, // Functions can't be compared
            (Value::Future(_), Value::Future(_)) => false, // Futures can't be compared
            _ => false,
        }
    }
}

/// Native function type
pub type NativeFn = dyn Fn(Vec<Value>) -> Result<Value, String> + Send + Sync;

impl Value {
    /// Check if value is truthy
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Boolean(b) => *b,
            Value::Integer(n) => *n != 0,
            Value::Float(f) => *f != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(arr) => !arr.is_empty(),
            Value::Object(obj) => !obj.is_empty(),
            _ => true,
        }
    }

    /// Get type name as string
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Nil => "nil",
            Value::Boolean(_) => "boolean",
            Value::Integer(_) => "integer",
            Value::Float(_) => "float",
            Value::String(_) => "string",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
            Value::Function(_) => "function",
            Value::Range { .. } => "range",
            Value::Tuple(_) => "tuple",
            Value::Struct { .. } => "struct",
            Value::Enum { .. } => "enum",
            Value::ReactiveRef(_) => "reactive",
            Value::Quote(_) => "quote",
            Value::Native(_) => "native_function",
            Value::Future(_) => "future",
        }
    }

    /// Try to convert to integer
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Value::Integer(n) => Some(*n),
            Value::Float(f) => Some(*f as i64),
            Value::Boolean(b) => Some(if *b { 1 } else { 0 }),
            Value::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// Try to convert to float
    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            Value::Integer(n) => Some(*n as f64),
            Value::Boolean(b) => Some(if *b { 1.0 } else { 0.0 }),
            Value::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// Try to convert to string
    pub fn as_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            _ => format!("{}", self),
        }
    }

    /// Try to convert to boolean
    pub fn as_boolean(&self) -> bool {
        self.is_truthy()
    }

    /// Check if value is numeric
    pub fn is_numeric(&self) -> bool {
        matches!(self, Value::Integer(_) | Value::Float(_))
    }

    /// Deep clone with shared references for functions
    pub fn deep_clone(&self) -> Self {
        self.clone()
    }

    /// Compare values for equality (handles NaN properly)
    pub fn equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Float(a), Value::Float(b)) => {
                if a.is_nan() && b.is_nan() {
                    true
                } else {
                    a == b
                }
            }
            _ => self == other,
        }
    }

    /// Perform addition
    pub fn add(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 + b)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a + *b as f64)),
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            // String concatenation with any type (convert to string)
            (Value::String(a), other) => Ok(Value::String(format!("{}{}", a, other))),
            (other, Value::String(b)) => Ok(Value::String(format!("{}{}", other, b))),
            (Value::Array(a), Value::Array(b)) => {
                let mut result = a.clone();
                result.extend(b.clone());
                Ok(Value::Array(result))
            }
            _ => Err(format!(
                "Cannot add {} and {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    /// Perform subtraction
    pub fn subtract(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 - b)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a - *b as f64)),
            _ => Err(format!(
                "Cannot subtract {} from {}",
                other.type_name(),
                self.type_name()
            )),
        }
    }

    /// Perform multiplication
    pub fn multiply(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float(*a as f64 * b)),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a * *b as f64)),
            (Value::String(s), Value::Integer(n)) | (Value::Integer(n), Value::String(s)) => {
                if *n < 0 {
                    return Err("Cannot multiply string by negative number".to_string());
                }
                Ok(Value::String(s.repeat(*n as usize)))
            }
            _ => Err(format!(
                "Cannot multiply {} and {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    /// Perform division
    pub fn divide(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => {
                if *b == 0 {
                    return Err("Division by zero".to_string());
                }
                Ok(Value::Integer(a / b))
            }
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 {
                    return Err("Division by zero".to_string());
                }
                Ok(Value::Float(a / b))
            }
            (Value::Integer(a), Value::Float(b)) => {
                if *b == 0.0 {
                    return Err("Division by zero".to_string());
                }
                Ok(Value::Float(*a as f64 / b))
            }
            (Value::Float(a), Value::Integer(b)) => {
                if *b == 0 {
                    return Err("Division by zero".to_string());
                }
                Ok(Value::Float(a / *b as f64))
            }
            _ => Err(format!(
                "Cannot divide {} by {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    /// Perform modulo
    pub fn modulo(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => {
                if *b == 0 {
                    return Err("Modulo by zero".to_string());
                }
                Ok(Value::Integer(a % b))
            }
            (Value::Float(a), Value::Float(b)) => {
                if *b == 0.0 {
                    return Err("Modulo by zero".to_string());
                }
                Ok(Value::Float(a % b))
            }
            _ => Err(format!(
                "Cannot compute modulo of {} and {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    /// Perform power operation
    pub fn power(&self, other: &Value) -> Result<Value, String> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => {
                if *b < 0 {
                    Ok(Value::Float((*a as f64).powf(*b as f64)))
                } else {
                    Ok(Value::Integer(a.pow(*b as u32)))
                }
            }
            (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a.powf(*b))),
            (Value::Integer(a), Value::Float(b)) => Ok(Value::Float((*a as f64).powf(*b))),
            (Value::Float(a), Value::Integer(b)) => Ok(Value::Float(a.powf(*b as f64))),
            _ => Err(format!(
                "Cannot raise {} to the power of {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }

    /// Perform comparison
    pub fn compare(&self, other: &Value) -> Result<std::cmp::Ordering, String> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => Ok(a.cmp(b)),
            (Value::Float(a), Value::Float(b)) => a
                .partial_cmp(b)
                .ok_or_else(|| "Cannot compare NaN".to_string()),
            (Value::Integer(a), Value::Float(b)) => (*a as f64)
                .partial_cmp(b)
                .ok_or_else(|| "Cannot compare NaN".to_string()),
            (Value::Float(a), Value::Integer(b)) => a
                .partial_cmp(&(*b as f64))
                .ok_or_else(|| "Cannot compare NaN".to_string()),
            (Value::String(a), Value::String(b)) => Ok(a.cmp(b)),
            (Value::Boolean(a), Value::Boolean(b)) => Ok(a.cmp(b)),
            _ => Err(format!(
                "Cannot compare {} and {}",
                self.type_name(),
                other.type_name()
            )),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Integer(n) => write!(f, "{}", n),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "{}", s),
            Value::Array(arr) => {
                write!(f, "[")?;
                for (i, val) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", val)?;
                }
                write!(f, "]")
            }
            Value::Object(obj) => {
                write!(f, "{{")?;
                for (i, (key, val)) in obj.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key, val)?;
                }
                write!(f, "}}")
            }
            Value::Function(func) => write!(f, "{:?}", func),
            Value::Range {
                start,
                end,
                inclusive,
            } => {
                if *inclusive {
                    write!(f, "{}..={}", start, end)
                } else {
                    write!(f, "{}..{}", start, end)
                }
            }
            Value::Tuple(values) => {
                write!(f, "(")?;
                for (i, val) in values.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", val)?;
                }
                write!(f, ")")
            }
            Value::Struct { name, fields } => {
                write!(f, "{} {{", name)?;
                for (i, (key, val)) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key, val)?;
                }
                write!(f, "}}")
            }
            Value::Enum {
                type_name,
                variant,
                values,
            } => {
                write!(f, "{}::{}", type_name, variant)?;
                if !values.is_empty() {
                    write!(f, "(")?;
                    for (i, val) in values.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", val)?;
                    }
                    write!(f, ")")?;
                }
                Ok(())
            }
            Value::ReactiveRef(id) => write!(f, "<reactive:{}>", id),
            Value::Quote(val) => write!(f, "quote({})", val),
            Value::Native(_) => write!(f, "<native_function>"),
            Value::Future(_) => write!(f, "<future>"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_truthy() {
        assert!(Value::Boolean(true).is_truthy());
        assert!(!Value::Boolean(false).is_truthy());
        assert!(!Value::Nil.is_truthy());
        assert!(Value::Integer(1).is_truthy());
        assert!(!Value::Integer(0).is_truthy());
    }

    #[test]
    fn test_value_add() {
        let a = Value::Integer(5);
        let b = Value::Integer(3);
        let result = a.add(&b).unwrap();
        assert_eq!(result, Value::Integer(8));

        let s1 = Value::String("Hello, ".to_string());
        let s2 = Value::String("World!".to_string());
        let result = s1.add(&s2).unwrap();
        assert_eq!(result, Value::String("Hello, World!".to_string()));
    }

    #[test]
    fn test_value_multiply() {
        let a = Value::Integer(5);
        let b = Value::Integer(3);
        let result = a.multiply(&b).unwrap();
        assert_eq!(result, Value::Integer(15));

        let s = Value::String("Ha".to_string());
        let n = Value::Integer(3);
        let result = s.multiply(&n).unwrap();
        assert_eq!(result, Value::String("HaHaHa".to_string()));
    }

    #[test]
    fn test_value_compare() {
        let a = Value::Integer(5);
        let b = Value::Integer(3);
        assert_eq!(a.compare(&b).unwrap(), std::cmp::Ordering::Greater);

        let s1 = Value::String("apple".to_string());
        let s2 = Value::String("banana".to_string());
        assert_eq!(s1.compare(&s2).unwrap(), std::cmp::Ordering::Less);
    }

    #[test]
    fn test_value_display() {
        assert_eq!(format!("{}", Value::Integer(42)), "42");
        assert_eq!(format!("{}", Value::String("test".to_string())), "test");
        assert_eq!(format!("{}", Value::Boolean(true)), "true");
        assert_eq!(format!("{}", Value::Nil), "nil");
    }
}
