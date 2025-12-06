//! # A-lang Interpreter
//!
//! The main interpreter module that executes A-lang AST with support for all WOW factors.

pub mod value;

use crate::ast::*;
use crate::reactive::ReactiveContext;
use crate::stdlib::ffi::FFIContext;
use crate::time_travel::{TimeTravelConfig, TimeTravelDebugger};
use im::HashMap as PersistentHashMap;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use value::{FunctionValue, Value};

/// Interpreter error types
#[derive(Debug, Clone)]
pub enum InterpreterError {
    UndefinedVariable(String),
    TypeError(String),
    RuntimeError(String),
    ReturnValue(Value),
    BreakStatement,
    ContinueStatement,
    DivisionByZero,
    IndexOutOfBounds,
    InvalidOperation(String),
    Throw(Value),
}

impl std::fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InterpreterError::UndefinedVariable(name) => {
                write!(f, "Undefined variable: {}", name)
            }
            InterpreterError::TypeError(msg) => write!(f, "Type error: {}", msg),
            InterpreterError::RuntimeError(msg) => write!(f, "Runtime error: {}", msg),
            InterpreterError::ReturnValue(_) => write!(f, "Return statement outside function"),
            InterpreterError::BreakStatement => write!(f, "Break statement outside loop"),
            InterpreterError::ContinueStatement => write!(f, "Continue statement outside loop"),
            InterpreterError::DivisionByZero => write!(f, "Division by zero"),
            InterpreterError::IndexOutOfBounds => write!(f, "Index out of bounds"),
            InterpreterError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            InterpreterError::Throw(val) => write!(f, "Uncaught exception: {}", val),
        }
    }
}

impl std::error::Error for InterpreterError {}

/// Environment for variable storage with scoping
#[derive(Debug, Clone)]
pub struct Environment {
    scopes: Vec<HashMap<String, Value>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, value);
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value.clone());
            }
        }
        None
    }

    pub fn set(&mut self, name: &str, value: Value) -> Result<(), InterpreterError> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) {
                scope.insert(name.to_string(), value);
                return Ok(());
            }
        }
        Err(InterpreterError::UndefinedVariable(name.to_string()))
    }

    pub fn to_persistent(&self) -> PersistentHashMap<String, Value> {
        let mut result = PersistentHashMap::new();
        for scope in &self.scopes {
            for (k, v) in scope {
                result.insert(k.clone(), v.clone());
            }
        }
        result
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

/// The A-lang interpreter with all WOW features
pub struct Interpreter {
    env: Environment,
    reactive_ctx: Arc<ReactiveContext>,
    time_travel: Arc<RwLock<TimeTravelDebugger>>,
    ffi_context: Arc<Mutex<FFIContext>>,
    current_file: String,
    current_line: usize,
    auto_snapshot_counter: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Self {
            env: Environment::new(),
            reactive_ctx: Arc::new(ReactiveContext::new()),
            time_travel: Arc::new(RwLock::new(TimeTravelDebugger::default())),
            ffi_context: Arc::new(Mutex::new(FFIContext::new())),
            current_file: "main.al".to_string(),
            current_line: 0,
            auto_snapshot_counter: 0,
        };

        interpreter.register_builtins();
        interpreter.register_ffi_builtins();
        interpreter
    }

    pub fn with_config(config: TimeTravelConfig) -> Self {
        let mut interpreter = Self {
            env: Environment::new(),
            reactive_ctx: Arc::new(ReactiveContext::new()),
            time_travel: Arc::new(RwLock::new(TimeTravelDebugger::new(config))),
            ffi_context: Arc::new(Mutex::new(FFIContext::new())),
            current_file: "main.al".to_string(),
            current_line: 0,
            auto_snapshot_counter: 0,
        };

        interpreter.register_builtins();
        interpreter.register_ffi_builtins();
        interpreter
    }

    /// Register built-in functions
    fn register_builtins(&mut self) {
        // print function
        self.env.define(
            "print".to_string(),
            Value::Native(Arc::new(|args| {
                for arg in args {
                    print!("{} ", arg);
                }
                println!();
                Ok(Value::Nil)
            })),
        );

        // input function (Python-style)
        self.env.define(
            "input".to_string(),
            Value::Native(Arc::new(|args| {
                use std::io::{self, Write};

                // Show prompt if provided
                if !args.is_empty() {
                    if let Value::String(prompt) = &args[0] {
                        print!("{}", prompt);
                        io::stdout().flush().unwrap();
                    } else {
                        print!("{} ", args[0]);
                        io::stdout().flush().unwrap();
                    }
                }

                // Read line from stdin
                let mut buffer = String::new();
                match io::stdin().read_line(&mut buffer) {
                    Ok(_) => Ok(Value::String(buffer.trim_end().to_string())),
                    Err(e) => Err(format!("Failed to read input: {}", e)),
                }
            })),
        );

        // len function
        self.env.define(
            "len".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("len expects exactly 1 argument".to_string());
                }
                match &args[0] {
                    Value::String(s) => Ok(Value::Integer(s.len() as i64)),
                    Value::Array(arr) => Ok(Value::Integer(arr.len() as i64)),
                    Value::Object(obj) => Ok(Value::Integer(obj.len() as i64)),
                    _ => Err(format!("len not supported for {}", args[0].type_name())),
                }
            })),
        );

        // type_of function
        self.env.define(
            "type_of".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("type_of expects exactly 1 argument".to_string());
                }
                Ok(Value::String(args[0].type_name().to_string()))
            })),
        );

        // push function (for arrays)
        self.env.define(
            "push".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 2 {
                    return Err("push expects exactly 2 arguments".to_string());
                }
                match &args[0] {
                    Value::Array(arr) => {
                        let mut new_arr = arr.clone();
                        new_arr.push(args[1].clone());
                        Ok(Value::Array(new_arr))
                    }
                    _ => Err("push expects an array as first argument".to_string()),
                }
            })),
        );

        // pop function
        self.env.define(
            "pop".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("pop expects exactly 1 argument".to_string());
                }
                match &args[0] {
                    Value::Array(arr) => {
                        let mut new_arr = arr.clone();
                        new_arr.pop();
                        Ok(Value::Array(new_arr))
                    }
                    _ => Err("pop expects an array".to_string()),
                }
            })),
        );

        // join function
        self.env.define(
            "join".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 2 {
                    return Err("join expects 2 arguments (array, separator)".to_string());
                }
                match (&args[0], &args[1]) {
                    (Value::Array(arr), Value::String(sep)) => {
                        let strings: Vec<String> = arr.iter().map(|v| v.as_string()).collect();
                        Ok(Value::String(strings.join(sep)))
                    }
                    _ => Err("join expects (array, string)".to_string()),
                }
            })),
        );

        // split function
        self.env.define(
            "split".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 2 {
                    return Err("split expects 2 arguments (string, separator)".to_string());
                }
                match (&args[0], &args[1]) {
                    (Value::String(s), Value::String(sep)) => {
                        let parts: Vec<Value> = s
                            .split(sep.as_str())
                            .map(|p| Value::String(p.to_string()))
                            .collect();
                        Ok(Value::Array(parts))
                    }
                    _ => Err("split expects (string, string)".to_string()),
                }
            })),
        );

        // str function (convert to string)
        self.env.define(
            "str".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("str expects exactly 1 argument".to_string());
                }
                Ok(Value::String(args[0].as_string()))
            })),
        );

        // int function (convert to integer)
        self.env.define(
            "int".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("int expects exactly 1 argument".to_string());
                }
                match args[0].as_integer() {
                    Some(n) => Ok(Value::Integer(n)),
                    None => Err(format!("Cannot convert {} to integer", args[0].type_name())),
                }
            })),
        );

        // float function (convert to float)
        self.env.define(
            "float".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("float expects exactly 1 argument".to_string());
                }
                match args[0].as_float() {
                    Some(f) => Ok(Value::Float(f)),
                    None => Err(format!("Cannot convert {} to float", args[0].type_name())),
                }
            })),
        );

        // abs function
        self.env.define(
            "abs".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("abs expects exactly 1 argument".to_string());
                }
                match &args[0] {
                    Value::Integer(n) => Ok(Value::Integer(n.abs())),
                    Value::Float(f) => Ok(Value::Float(f.abs())),
                    _ => Err("abs expects a number".to_string()),
                }
            })),
        );

        // min function
        self.env.define(
            "min".to_string(),
            Value::Native(Arc::new(|args| {
                if args.is_empty() {
                    return Err("min expects at least 1 argument".to_string());
                }
                let mut min_val = args[0].clone();
                for arg in &args[1..] {
                    if let Ok(ordering) = arg.compare(&min_val) {
                        if ordering == std::cmp::Ordering::Less {
                            min_val = arg.clone();
                        }
                    }
                }
                Ok(min_val)
            })),
        );

        // max function
        self.env.define(
            "max".to_string(),
            Value::Native(Arc::new(|args| {
                if args.is_empty() {
                    return Err("max expects at least 1 argument".to_string());
                }
                let mut max_val = args[0].clone();
                for arg in &args[1..] {
                    if let Ok(ordering) = arg.compare(&max_val) {
                        if ordering == std::cmp::Ordering::Greater {
                            max_val = arg.clone();
                        }
                    }
                }
                Ok(max_val)
            })),
        );

        // floor function
        self.env.define(
            "floor".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("floor expects exactly 1 argument".to_string());
                }
                match &args[0] {
                    Value::Float(f) => Ok(Value::Integer(f.floor() as i64)),
                    Value::Integer(n) => Ok(Value::Integer(*n)),
                    _ => Err("floor expects a number".to_string()),
                }
            })),
        );

        // ceil function
        self.env.define(
            "ceil".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("ceil expects exactly 1 argument".to_string());
                }
                match &args[0] {
                    Value::Float(f) => Ok(Value::Integer(f.ceil() as i64)),
                    Value::Integer(n) => Ok(Value::Integer(*n)),
                    _ => Err("ceil expects a number".to_string()),
                }
            })),
        );

        // round function
        self.env.define(
            "round".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("round expects exactly 1 argument".to_string());
                }
                match &args[0] {
                    Value::Float(f) => Ok(Value::Integer(f.round() as i64)),
                    Value::Integer(n) => Ok(Value::Integer(*n)),
                    _ => Err("round expects a number".to_string()),
                }
            })),
        );

        // keys function (for objects)
        self.env.define(
            "keys".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("keys expects exactly 1 argument".to_string());
                }
                match &args[0] {
                    Value::Object(obj) => {
                        let keys: Vec<Value> =
                            obj.keys().map(|k| Value::String(k.clone())).collect();
                        Ok(Value::Array(keys))
                    }
                    _ => Err("keys expects an object".to_string()),
                }
            })),
        );

        // values function (for objects)
        self.env.define(
            "values".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("values expects exactly 1 argument".to_string());
                }
                match &args[0] {
                    Value::Object(obj) => {
                        let values: Vec<Value> = obj.values().cloned().collect();
                        Ok(Value::Array(values))
                    }
                    _ => Err("values expects an object".to_string()),
                }
            })),
        );

        // range function
        self.env.define(
            "range".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() < 1 || args.len() > 2 {
                    return Err("range expects 1 or 2 arguments".to_string());
                }
                let (start, end) = if args.len() == 1 {
                    (0, args[0].as_integer().unwrap_or(0))
                } else {
                    (
                        args[0].as_integer().unwrap_or(0),
                        args[1].as_integer().unwrap_or(0),
                    )
                };
                let arr: Vec<Value> = (start..end).map(Value::Integer).collect();
                Ok(Value::Array(arr))
            })),
        );

        // map, filter, reduce are handled specially in call_function
        // They need interpreter context to call the lambda

        // reverse function
        self.env.define(
            "reverse".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("reverse expects exactly 1 argument".to_string());
                }
                match &args[0] {
                    Value::Array(arr) => {
                        let mut new_arr = arr.clone();
                        new_arr.reverse();
                        Ok(Value::Array(new_arr))
                    }
                    Value::String(s) => {
                        let reversed: String = s.chars().rev().collect();
                        Ok(Value::String(reversed))
                    }
                    _ => Err("reverse expects an array or string".to_string()),
                }
            })),
        );

        // slice function
        self.env.define(
            "slice".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() < 2 || args.len() > 3 {
                    return Err("slice expects 2 or 3 arguments (array, start, end?)".to_string());
                }
                match &args[0] {
                    Value::Array(arr) => {
                        let start = args[1].as_integer().unwrap_or(0) as usize;
                        let end = if args.len() == 3 {
                            args[2].as_integer().unwrap_or(arr.len() as i64) as usize
                        } else {
                            arr.len()
                        };
                        let sliced = arr[start.min(arr.len())..end.min(arr.len())].to_vec();
                        Ok(Value::Array(sliced))
                    }
                    _ => Err("slice expects an array".to_string()),
                }
            })),
        );

        // indexOf function
        self.env.define(
            "indexOf".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 2 {
                    return Err("indexOf expects 2 arguments (array/string, value)".to_string());
                }
                match (&args[0], &args[1]) {
                    (Value::Array(arr), val) => {
                        for (i, item) in arr.iter().enumerate() {
                            if item == val {
                                return Ok(Value::Integer(i as i64));
                            }
                        }
                        Ok(Value::Integer(-1))
                    }
                    (Value::String(s), Value::String(search)) => match s.find(search.as_str()) {
                        Some(idx) => Ok(Value::Integer(idx as i64)),
                        None => Ok(Value::Integer(-1)),
                    },
                    _ => Err("indexOf expects (array, value) or (string, string)".to_string()),
                }
            })),
        );

        // includes function
        self.env.define(
            "includes".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 2 {
                    return Err("includes expects 2 arguments (array/string, value)".to_string());
                }
                match (&args[0], &args[1]) {
                    (Value::Array(arr), val) => Ok(Value::Boolean(arr.contains(val))),
                    (Value::String(s), Value::String(search)) => {
                        Ok(Value::Boolean(s.contains(search.as_str())))
                    }
                    _ => Err("includes expects (array, value) or (string, string)".to_string()),
                }
            })),
        );

        // toUpperCase function
        self.env.define(
            "toUpperCase".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("toUpperCase expects exactly 1 argument".to_string());
                }
                match &args[0] {
                    Value::String(s) => Ok(Value::String(s.to_uppercase())),
                    _ => Err("toUpperCase expects a string".to_string()),
                }
            })),
        );

        // toLowerCase function
        self.env.define(
            "toLowerCase".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("toLowerCase expects exactly 1 argument".to_string());
                }
                match &args[0] {
                    Value::String(s) => Ok(Value::String(s.to_lowercase())),
                    _ => Err("toLowerCase expects a string".to_string()),
                }
            })),
        );

        // trim function
        self.env.define(
            "trim".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("trim expects exactly 1 argument".to_string());
                }
                match &args[0] {
                    Value::String(s) => Ok(Value::String(s.trim().to_string())),
                    _ => Err("trim expects a string".to_string()),
                }
            })),
        );

        // replace function
        self.env.define(
            "replace".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 3 {
                    return Err(
                        "replace expects 3 arguments (string, search, replacement)".to_string()
                    );
                }
                match (&args[0], &args[1], &args[2]) {
                    (Value::String(s), Value::String(search), Value::String(replacement)) => Ok(
                        Value::String(s.replace(search.as_str(), replacement.as_str())),
                    ),
                    _ => Err("replace expects (string, string, string)".to_string()),
                }
            })),
        );

        // ===== FILE I/O =====

        // readFile function
        self.env.define(
            "readFile".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("readFile expects exactly 1 argument (path)".to_string());
                }
                match &args[0] {
                    Value::String(path) => match std::fs::read_to_string(path) {
                        Ok(content) => Ok(Value::String(content)),
                        Err(e) => Err(format!("Failed to read file: {}", e)),
                    },
                    _ => Err("readFile expects a string path".to_string()),
                }
            })),
        );

        // writeFile function
        self.env.define(
            "writeFile".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 2 {
                    return Err("writeFile expects 2 arguments (path, content)".to_string());
                }
                match (&args[0], &args[1]) {
                    (Value::String(path), Value::String(content)) => {
                        match std::fs::write(path, content) {
                            Ok(_) => Ok(Value::Nil),
                            Err(e) => Err(format!("Failed to write file: {}", e)),
                        }
                    }
                    _ => Err("writeFile expects (string, string)".to_string()),
                }
            })),
        );

        // readLines function
        self.env.define(
            "readLines".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("readLines expects exactly 1 argument (path)".to_string());
                }
                match &args[0] {
                    Value::String(path) => match std::fs::read_to_string(path) {
                        Ok(content) => {
                            let lines: Vec<Value> = content
                                .lines()
                                .map(|l| Value::String(l.to_string()))
                                .collect();
                            Ok(Value::Array(lines))
                        }
                        Err(e) => Err(format!("Failed to read file: {}", e)),
                    },
                    _ => Err("readLines expects a string path".to_string()),
                }
            })),
        );

        // appendFile function
        self.env.define(
            "appendFile".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 2 {
                    return Err("appendFile expects 2 arguments (path, content)".to_string());
                }
                match (&args[0], &args[1]) {
                    (Value::String(path), Value::String(content)) => {
                        use std::fs::OpenOptions;
                        use std::io::Write;
                        match OpenOptions::new().create(true).append(true).open(path) {
                            Ok(mut file) => match writeln!(file, "{}", content) {
                                Ok(_) => Ok(Value::Nil),
                                Err(e) => Err(format!("Failed to append to file: {}", e)),
                            },
                            Err(e) => Err(format!("Failed to open file: {}", e)),
                        }
                    }
                    _ => Err("appendFile expects (string, string)".to_string()),
                }
            })),
        );

        // fileExists function
        self.env.define(
            "fileExists".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("fileExists expects exactly 1 argument (path)".to_string());
                }
                match &args[0] {
                    Value::String(path) => Ok(Value::Boolean(std::path::Path::new(path).exists())),
                    _ => Err("fileExists expects a string path".to_string()),
                }
            })),
        );

        // ===== JSON =====

        // parseJSON function
        self.env.define(
            "parseJSON".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("parseJSON expects exactly 1 argument (json string)".to_string());
                }
                match &args[0] {
                    Value::String(json_str) => {
                        match serde_json::from_str::<serde_json::Value>(json_str) {
                            Ok(json) => Ok(Self::json_to_value(&json)),
                            Err(e) => Err(format!("Failed to parse JSON: {}", e)),
                        }
                    }
                    _ => Err("parseJSON expects a string".to_string()),
                }
            })),
        );

        // stringifyJSON function
        self.env.define(
            "stringifyJSON".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("stringifyJSON expects exactly 1 argument".to_string());
                }
                match Self::value_to_json(&args[0]) {
                    Ok(json) => match serde_json::to_string(&json) {
                        Ok(s) => Ok(Value::String(s)),
                        Err(e) => Err(format!("Failed to stringify JSON: {}", e)),
                    },
                    Err(e) => Err(e),
                }
            })),
        );

        // ===== ADVANCED MATH =====

        // sqrt function
        self.env.define(
            "sqrt".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("sqrt expects exactly 1 argument".to_string());
                }
                match args[0].as_float() {
                    Some(n) => Ok(Value::Float(n.sqrt())),
                    None => Err("sqrt expects a number".to_string()),
                }
            })),
        );

        // pow function
        self.env.define(
            "pow".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 2 {
                    return Err("pow expects 2 arguments (base, exponent)".to_string());
                }
                match (args[0].as_float(), args[1].as_float()) {
                    (Some(base), Some(exp)) => Ok(Value::Float(base.powf(exp))),
                    _ => Err("pow expects numbers".to_string()),
                }
            })),
        );

        // random function (0.0 to 1.0)
        self.env.define(
            "random".to_string(),
            Value::Native(Arc::new(|_args| {
                use std::collections::hash_map::RandomState;
                use std::hash::{BuildHasher, Hash, Hasher};
                let state = RandomState::new();
                let mut hasher = state.build_hasher();
                std::time::SystemTime::now().hash(&mut hasher);
                let hash = hasher.finish();
                let random = (hash as f64) / (u64::MAX as f64);
                Ok(Value::Float(random))
            })),
        );

        // randomInt function
        self.env.define(
            "randomInt".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 2 {
                    return Err("randomInt expects 2 arguments (min, max)".to_string());
                }
                match (args[0].as_integer(), args[1].as_integer()) {
                    (Some(min), Some(max)) => {
                        use std::collections::hash_map::RandomState;
                        use std::hash::{BuildHasher, Hash, Hasher};
                        let state = RandomState::new();
                        let mut hasher = state.build_hasher();
                        std::time::SystemTime::now().hash(&mut hasher);
                        let hash = hasher.finish();
                        let range = (max - min) as u64;
                        let result = min + (hash % range) as i64;
                        Ok(Value::Integer(result))
                    }
                    _ => Err("randomInt expects integers".to_string()),
                }
            })),
        );

        // sin function
        self.env.define(
            "sin".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("sin expects exactly 1 argument".to_string());
                }
                match args[0].as_float() {
                    Some(n) => Ok(Value::Float(n.sin())),
                    None => Err("sin expects a number".to_string()),
                }
            })),
        );

        // cos function
        self.env.define(
            "cos".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("cos expects exactly 1 argument".to_string());
                }
                match args[0].as_float() {
                    Some(n) => Ok(Value::Float(n.cos())),
                    None => Err("cos expects a number".to_string()),
                }
            })),
        );

        // tan function
        self.env.define(
            "tan".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("tan expects exactly 1 argument".to_string());
                }
                match args[0].as_float() {
                    Some(n) => Ok(Value::Float(n.tan())),
                    None => Err("tan expects a number".to_string()),
                }
            })),
        );

        // PI constant
        self.env
            .define("PI".to_string(), Value::Float(std::f64::consts::PI));

        // E constant
        self.env
            .define("E".to_string(), Value::Float(std::f64::consts::E));

        // ===== SYSTEM UTILITIES (IoT/Embedded) =====

        // exec function - Execute system commands
        self.env.define(
            "exec".to_string(),
            Value::Native(Arc::new(|args| {
                if args.is_empty() {
                    return Err("exec expects at least 1 argument (command)".to_string());
                }
                match &args[0] {
                    Value::String(cmd) => {
                        use std::process::Command;
                        let parts: Vec<&str> = cmd.split_whitespace().collect();
                        if parts.is_empty() {
                            return Err("Empty command".to_string());
                        }

                        let output = Command::new(parts[0]).args(&parts[1..]).output();

                        match output {
                            Ok(out) => {
                                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                                Ok(Value::String(stdout))
                            }
                            Err(e) => Err(format!("Failed to execute: {}", e)),
                        }
                    }
                    _ => Err("exec expects a string command".to_string()),
                }
            })),
        );

        // getEnv function - Get environment variable
        self.env.define(
            "getEnv".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("getEnv expects exactly 1 argument (var name)".to_string());
                }
                match &args[0] {
                    Value::String(name) => match std::env::var(name) {
                        Ok(val) => Ok(Value::String(val)),
                        Err(_) => Ok(Value::Nil),
                    },
                    _ => Err("getEnv expects a string".to_string()),
                }
            })),
        );

        // setEnv function - Set environment variable
        self.env.define(
            "setEnv".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 2 {
                    return Err("setEnv expects 2 arguments (name, value)".to_string());
                }
                match (&args[0], &args[1]) {
                    (Value::String(name), Value::String(val)) => {
                        std::env::set_var(name, val);
                        Ok(Value::Nil)
                    }
                    _ => Err("setEnv expects (string, string)".to_string()),
                }
            })),
        );

        // sleep function - Sleep for milliseconds
        self.env.define(
            "sleep".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("sleep expects exactly 1 argument (milliseconds)".to_string());
                }
                match args[0].as_integer() {
                    Some(ms) => {
                        std::thread::sleep(std::time::Duration::from_millis(ms as u64));
                        Ok(Value::Nil)
                    }
                    None => Err("sleep expects an integer (milliseconds)".to_string()),
                }
            })),
        );

        // timestamp function - Get current Unix timestamp
        self.env.define(
            "timestamp".to_string(),
            Value::Native(Arc::new(|_args| {
                match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
                    Ok(duration) => Ok(Value::Integer(duration.as_secs() as i64)),
                    Err(_) => Err("Failed to get timestamp".to_string()),
                }
            })),
        );

        // ===== BINARY/BYTES HANDLING =====

        // bytes function - Convert string to byte array
        self.env.define(
            "bytes".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("bytes expects exactly 1 argument".to_string());
                }
                match &args[0] {
                    Value::String(s) => {
                        let bytes: Vec<Value> =
                            s.bytes().map(|b| Value::Integer(b as i64)).collect();
                        Ok(Value::Array(bytes))
                    }
                    _ => Err("bytes expects a string".to_string()),
                }
            })),
        );

        // fromBytes function - Convert byte array to string
        self.env.define(
            "fromBytes".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("fromBytes expects exactly 1 argument (byte array)".to_string());
                }
                match &args[0] {
                    Value::Array(arr) => {
                        let mut bytes = Vec::new();
                        for val in arr {
                            match val.as_integer() {
                                Some(b) if b >= 0 && b <= 255 => bytes.push(b as u8),
                                _ => return Err("Invalid byte value".to_string()),
                            }
                        }
                        match String::from_utf8(bytes) {
                            Ok(s) => Ok(Value::String(s)),
                            Err(_) => Err("Invalid UTF-8 sequence".to_string()),
                        }
                    }
                    _ => Err("fromBytes expects an array".to_string()),
                }
            })),
        );

        // packInt function - Pack integer to bytes (little-endian)
        self.env.define(
            "packInt".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("packInt expects exactly 1 argument".to_string());
                }
                match args[0].as_integer() {
                    Some(n) => {
                        let bytes: Vec<Value> = n
                            .to_le_bytes()
                            .iter()
                            .map(|&b| Value::Integer(b as i64))
                            .collect();
                        Ok(Value::Array(bytes))
                    }
                    None => Err("packInt expects an integer".to_string()),
                }
            })),
        );

        // unpackInt function - Unpack bytes to integer (little-endian)
        self.env.define(
            "unpackInt".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("unpackInt expects exactly 1 argument (byte array)".to_string());
                }
                match &args[0] {
                    Value::Array(arr) => {
                        if arr.len() != 8 {
                            return Err("unpackInt expects 8 bytes".to_string());
                        }
                        let mut bytes = [0u8; 8];
                        for (i, val) in arr.iter().enumerate() {
                            match val.as_integer() {
                                Some(b) if b >= 0 && b <= 255 => bytes[i] = b as u8,
                                _ => return Err("Invalid byte value".to_string()),
                            }
                        }
                        Ok(Value::Integer(i64::from_le_bytes(bytes)))
                    }
                    _ => Err("unpackInt expects an array".to_string()),
                }
            })),
        );

        // hexEncode function - Encode bytes to hex string
        self.env.define(
            "hexEncode".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("hexEncode expects exactly 1 argument".to_string());
                }
                match &args[0] {
                    Value::Array(arr) => {
                        let mut hex = String::new();
                        for val in arr {
                            match val.as_integer() {
                                Some(b) if b >= 0 && b <= 255 => {
                                    hex.push_str(&format!("{:02x}", b as u8));
                                }
                                _ => return Err("Invalid byte value".to_string()),
                            }
                        }
                        Ok(Value::String(hex))
                    }
                    Value::String(s) => {
                        let hex: String = s.bytes().map(|b| format!("{:02x}", b)).collect();
                        Ok(Value::String(hex))
                    }
                    _ => Err("hexEncode expects an array or string".to_string()),
                }
            })),
        );

        // hexDecode function - Decode hex string to bytes
        self.env.define(
            "hexDecode".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("hexDecode expects exactly 1 argument (hex string)".to_string());
                }
                match &args[0] {
                    Value::String(hex) => {
                        let hex_clean = hex.replace(" ", "");
                        if hex_clean.len() % 2 != 0 {
                            return Err("Hex string must have even length".to_string());
                        }
                        let mut bytes = Vec::new();
                        for i in (0..hex_clean.len()).step_by(2) {
                            match u8::from_str_radix(&hex_clean[i..i + 2], 16) {
                                Ok(b) => bytes.push(Value::Integer(b as i64)),
                                Err(_) => return Err("Invalid hex string".to_string()),
                            }
                        }
                        Ok(Value::Array(bytes))
                    }
                    _ => Err("hexDecode expects a string".to_string()),
                }
            })),
        );

        // ===== NETWORK UTILITIES (Basic) =====

        // httpGet function - Simple HTTP GET (using std only, no external deps)
        self.env.define(
            "httpGet".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("httpGet expects exactly 1 argument (URL)".to_string());
                }
                // Note: This is a placeholder. In production, use reqwest or similar
                Err(
                    "httpGet not yet implemented - add reqwest dependency for HTTP support"
                        .to_string(),
                )
            })),
        );

        // ===== HARDWARE/IoT SIMULATION =====

        // GPIO simulation (for IoT/embedded)
        self.env.define(
            "gpioSetup".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 2 {
                    return Err("gpioSetup expects 2 arguments (pin, mode)".to_string());
                }
                // Simulated GPIO - in real implementation, would interface with hardware
                Ok(Value::Boolean(true))
            })),
        );

        self.env.define(
            "gpioWrite".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 2 {
                    return Err("gpioWrite expects 2 arguments (pin, value)".to_string());
                }
                // Simulated GPIO write
                Ok(Value::Nil)
            })),
        );

        self.env.define(
            "gpioRead".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("gpioRead expects 1 argument (pin)".to_string());
                }
                // Simulated GPIO read - returns random 0 or 1
                use std::collections::hash_map::RandomState;
                use std::hash::{BuildHasher, Hash, Hasher};
                let state = RandomState::new();
                let mut hasher = state.build_hasher();
                std::time::SystemTime::now().hash(&mut hasher);
                let val = (hasher.finish() % 2) as i64;
                Ok(Value::Integer(val))
            })),
        );

        // I2C simulation
        self.env.define(
            "i2cWrite".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 2 {
                    return Err("i2cWrite expects 2 arguments (address, data)".to_string());
                }
                // Simulated I2C write
                Ok(Value::Nil)
            })),
        );

        self.env.define(
            "i2cRead".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 2 {
                    return Err("i2cRead expects 2 arguments (address, length)".to_string());
                }
                // Simulated I2C read - returns dummy bytes
                match args[1].as_integer() {
                    Some(len) => {
                        let bytes: Vec<Value> = (0..len).map(|_| Value::Integer(0)).collect();
                        Ok(Value::Array(bytes))
                    }
                    None => Err("i2cRead expects length as integer".to_string()),
                }
            })),
        );

        // SPI simulation
        self.env.define(
            "spiTransfer".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("spiTransfer expects 1 argument (data array)".to_string());
                }
                // Simulated SPI transfer - echoes back
                match &args[0] {
                    Value::Array(arr) => Ok(Value::Array(arr.clone())),
                    _ => Err("spiTransfer expects an array".to_string()),
                }
            })),
        );

        // UART simulation
        self.env.define(
            "uartWrite".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("uartWrite expects 1 argument (data)".to_string());
                }
                // Simulated UART write
                Ok(Value::Nil)
            })),
        );

        self.env.define(
            "uartRead".to_string(),
            Value::Native(Arc::new(|_args| {
                // Simulated UART read - returns empty string
                Ok(Value::String(String::new()))
            })),
        );

        // ===== BITWISE UTILITIES =====

        // bitAnd function
        self.env.define(
            "bitAnd".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 2 {
                    return Err("bitAnd expects 2 arguments".to_string());
                }
                match (args[0].as_integer(), args[1].as_integer()) {
                    (Some(a), Some(b)) => Ok(Value::Integer(a & b)),
                    _ => Err("bitAnd expects integers".to_string()),
                }
            })),
        );

        // bitOr function
        self.env.define(
            "bitOr".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 2 {
                    return Err("bitOr expects 2 arguments".to_string());
                }
                match (args[0].as_integer(), args[1].as_integer()) {
                    (Some(a), Some(b)) => Ok(Value::Integer(a | b)),
                    _ => Err("bitOr expects integers".to_string()),
                }
            })),
        );

        // bitXor function
        self.env.define(
            "bitXor".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 2 {
                    return Err("bitXor expects 2 arguments".to_string());
                }
                match (args[0].as_integer(), args[1].as_integer()) {
                    (Some(a), Some(b)) => Ok(Value::Integer(a ^ b)),
                    _ => Err("bitXor expects integers".to_string()),
                }
            })),
        );

        // bitNot function
        self.env.define(
            "bitNot".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("bitNot expects 1 argument".to_string());
                }
                match args[0].as_integer() {
                    Some(a) => Ok(Value::Integer(!a)),
                    _ => Err("bitNot expects an integer".to_string()),
                }
            })),
        );

        // bitShiftLeft function
        self.env.define(
            "bitShiftLeft".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 2 {
                    return Err("bitShiftLeft expects 2 arguments".to_string());
                }
                match (args[0].as_integer(), args[1].as_integer()) {
                    (Some(a), Some(b)) => Ok(Value::Integer(a << b)),
                    _ => Err("bitShiftLeft expects integers".to_string()),
                }
            })),
        );

        // bitShiftRight function
        self.env.define(
            "bitShiftRight".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 2 {
                    return Err("bitShiftRight expects 2 arguments".to_string());
                }
                match (args[0].as_integer(), args[1].as_integer()) {
                    (Some(a), Some(b)) => Ok(Value::Integer(a >> b)),
                    _ => Err("bitShiftRight expects integers".to_string()),
                }
            })),
        );
    }

    /// Register FFI (Foreign Function Interface) built-in functions
    fn register_ffi_builtins(&mut self) {
        use crate::stdlib::ffi::FFIType;

        // ffiLoadLibrary(name: string, path: string) -> boolean
        let ffi_ctx = Arc::clone(&self.ffi_context);
        self.env.define(
            "ffiLoadLibrary".to_string(),
            Value::Native(Arc::new(move |args| {
                if args.len() != 2 {
                    return Err("ffiLoadLibrary expects 2 arguments (name, path)".to_string());
                }

                match (&args[0], &args[1]) {
                    (Value::String(name), Value::String(path)) => {
                        let mut ctx = ffi_ctx.lock().unwrap();
                        match ctx.load_library(name, path) {
                            Ok(_) => Ok(Value::Boolean(true)),
                            Err(e) => Err(format!("Failed to load library: {}", e)),
                        }
                    }
                    _ => Err("ffiLoadLibrary expects (string, string)".to_string()),
                }
            })),
        );

        // ffiRegisterFunction(name: string, return_type: string, param_types: array) -> boolean
        let ffi_ctx = Arc::clone(&self.ffi_context);
        self.env.define(
            "ffiRegisterFunction".to_string(),
            Value::Native(Arc::new(move |args| {
                if args.len() != 3 {
                    return Err(
                        "ffiRegisterFunction expects 3 arguments (name, return_type, param_types)"
                            .to_string(),
                    );
                }

                match (&args[0], &args[1], &args[2]) {
                    (Value::String(name), Value::String(ret_type), Value::Array(params)) => {
                        // Parse return type
                        let return_type = FFIType::from_string(ret_type)
                            .map_err(|e| format!("Invalid return type: {}", e))?;

                        // Parse parameter types
                        let mut param_types = Vec::new();
                        for param in params {
                            if let Value::String(type_str) = param {
                                let ffi_type = FFIType::from_string(type_str)
                                    .map_err(|e| format!("Invalid param type: {}", e))?;
                                param_types.push(ffi_type);
                            } else {
                                return Err("Parameter types must be strings".to_string());
                            }
                        }

                        let mut ctx = ffi_ctx.lock().unwrap();
                        ctx.register_signature(name, return_type, param_types);
                        Ok(Value::Boolean(true))
                    }
                    _ => Err("ffiRegisterFunction expects (string, string, array)".to_string()),
                }
            })),
        );

        // ffiCall(lib_name: string, func_name: string, args: array) -> any
        let ffi_ctx = Arc::clone(&self.ffi_context);
        self.env.define(
            "ffiCall".to_string(),
            Value::Native(Arc::new(move |args| {
                if args.len() != 3 {
                    return Err(
                        "ffiCall expects 3 arguments (lib_name, func_name, args)".to_string()
                    );
                }

                match (&args[0], &args[1], &args[2]) {
                    (
                        Value::String(lib_name),
                        Value::String(func_name),
                        Value::Array(func_args),
                    ) => {
                        let ctx = ffi_ctx.lock().unwrap();
                        ctx.call_function(lib_name, func_name, func_args.clone())
                    }
                    _ => Err("ffiCall expects (string, string, array)".to_string()),
                }
            })),
        );
    }

    // Helper to convert serde_json::Value to our Value
    fn json_to_value(json: &serde_json::Value) -> Value {
        match json {
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
                let values: Vec<Value> = arr.iter().map(Self::json_to_value).collect();
                Value::Array(values)
            }
            serde_json::Value::Object(obj) => {
                let mut map = HashMap::new();
                for (k, v) in obj {
                    map.insert(k.clone(), Self::json_to_value(v));
                }
                Value::Object(map)
            }
        }
    }

    // Helper to convert our Value to serde_json::Value
    fn value_to_json(value: &Value) -> Result<serde_json::Value, String> {
        match value {
            Value::Nil => Ok(serde_json::Value::Null),
            Value::Boolean(b) => Ok(serde_json::Value::Bool(*b)),
            Value::Integer(i) => Ok(serde_json::Value::Number((*i).into())),
            Value::Float(f) => {
                if let Some(n) = serde_json::Number::from_f64(*f) {
                    Ok(serde_json::Value::Number(n))
                } else {
                    Err("Invalid float value for JSON".to_string())
                }
            }
            Value::String(s) => Ok(serde_json::Value::String(s.clone())),
            Value::Array(arr) => {
                let mut json_arr = Vec::new();
                for item in arr {
                    json_arr.push(Self::value_to_json(item)?);
                }
                Ok(serde_json::Value::Array(json_arr))
            }
            Value::Object(obj) => {
                let mut json_obj = serde_json::Map::new();
                for (k, v) in obj {
                    json_obj.insert(k.clone(), Self::value_to_json(v)?);
                }
                Ok(serde_json::Value::Object(json_obj))
            }
            _ => Err(format!("Cannot convert {} to JSON", value.type_name())),
        }
    }

    /// Execute a program
    pub fn execute(&mut self, program: &Program) -> Result<Value, InterpreterError> {
        let mut last_value = Value::Nil;

        for statement in &program.statements {
            last_value = self.execute_statement(statement)?;
        }

        Ok(last_value)
    }

    /// Execute a single statement
    fn execute_statement(&mut self, statement: &Statement) -> Result<Value, InterpreterError> {
        // Auto-snapshot for time-travel debugging
        self.auto_snapshot_counter += 1;
        if self.auto_snapshot_counter % 100 == 0 {
            let _ = self.take_snapshot(None);
        }

        match statement {
            Statement::Let {
                name,
                value,
                type_annotation: _,
                span: _,
            } => {
                let val = self.evaluate_expression(value)?;
                // Check if it's a reactive variable
                if let Some(Value::ReactiveRef(_)) = self.env.get(name) {
                    self.reactive_ctx
                        .set(name, val)
                        .map_err(|e| InterpreterError::RuntimeError(e.to_string()))?;
                } else if self.env.get(name).is_some() {
                    // Try to set existing variable
                    self.env.set(name, val)?;
                } else {
                    // Define new variable
                    self.env.define(name.clone(), val);
                }
                Ok(Value::Nil)
            }

            Statement::Const {
                name,
                value,
                type_annotation: _,
                span: _,
            } => {
                let val = self.evaluate_expression(value)?;
                self.env.define(name.clone(), val);
                Ok(Value::Nil)
            }

            Statement::Reactive {
                name,
                initial_value,
                type_annotation: _,
                span: _,
            } => {
                let val = self.evaluate_expression(initial_value)?;
                let node_id = self
                    .reactive_ctx
                    .register_signal(name.clone(), val)
                    .map_err(|e| InterpreterError::RuntimeError(e.to_string()))?;
                self.env.define(name.clone(), Value::ReactiveRef(node_id));
                Ok(Value::Nil)
            }

            Statement::Function {
                name,
                parameters,
                body,
                return_type: _,
                is_async,
                span: _,
            } => {
                let param_names: Vec<String> = parameters.iter().map(|p| p.name.clone()).collect();

                let func = Value::Function(Arc::new(FunctionValue {
                    name: Some(name.clone()),
                    parameters: param_names,
                    body: body.clone(),
                    closure: HashMap::new(),
                    is_async: *is_async,
                }));

                self.env.define(name.clone(), func);
                Ok(Value::Nil)
            }

            Statement::Return { value, span: _ } => {
                let val = if let Some(expr) = value {
                    self.evaluate_expression(expr)?
                } else {
                    Value::Nil
                };
                Err(InterpreterError::ReturnValue(val))
            }

            Statement::Expression { expr, span: _ } => self.evaluate_expression(expr),

            Statement::If {
                condition,
                then_branch,
                else_branch,
                span: _,
            } => {
                let cond_val = self.evaluate_expression(condition)?;
                if cond_val.is_truthy() {
                    self.execute_block(then_branch)
                } else if let Some(else_stmts) = else_branch {
                    self.execute_block(else_stmts)
                } else {
                    Ok(Value::Nil)
                }
            }

            Statement::While {
                condition,
                body,
                span: _,
            } => {
                while self.evaluate_expression(condition)?.is_truthy() {
                    match self.execute_block(body) {
                        Err(InterpreterError::BreakStatement) => break,
                        Err(InterpreterError::ContinueStatement) => continue,
                        Err(e) => return Err(e),
                        Ok(_) => {}
                    }
                }
                Ok(Value::Nil)
            }

            Statement::For {
                variable,
                iterable,
                body,
                span: _,
            } => {
                let iter_val = self.evaluate_expression(iterable)?;

                let items = match iter_val {
                    Value::Array(arr) => arr,
                    Value::Range {
                        start,
                        end,
                        inclusive,
                    } => {
                        let range = if inclusive {
                            start..=end
                        } else {
                            start..=end - 1
                        };
                        range.map(Value::Integer).collect()
                    }
                    _ => {
                        return Err(InterpreterError::TypeError(format!(
                            "Cannot iterate over {}",
                            iter_val.type_name()
                        )))
                    }
                };

                self.env.push_scope();
                for item in items {
                    self.env.define(variable.clone(), item);
                    match self.execute_block(body) {
                        Err(InterpreterError::BreakStatement) => break,
                        Err(InterpreterError::ContinueStatement) => continue,
                        Err(e) => {
                            self.env.pop_scope();
                            return Err(e);
                        }
                        Ok(_) => {}
                    }
                }
                self.env.pop_scope();

                Ok(Value::Nil)
            }

            Statement::Break { span: _ } => Err(InterpreterError::BreakStatement),

            Statement::Continue { span: _ } => Err(InterpreterError::ContinueStatement),

            Statement::Snapshot { label, span: _ } => {
                self.take_snapshot(label.clone())?;
                Ok(Value::Nil)
            }

            Statement::Rewind {
                steps,
                to_label,
                span: _,
            } => {
                if let Some(label) = to_label {
                    self.rewind_to_checkpoint(label)?;
                } else if let Some(steps_expr) = steps {
                    let steps_val = self.evaluate_expression(steps_expr)?;
                    if let Value::Integer(n) = steps_val {
                        self.rewind_steps(n as usize)?;
                    } else {
                        return Err(InterpreterError::TypeError(
                            "Rewind steps must be an integer".to_string(),
                        ));
                    }
                } else {
                    self.rewind_steps(1)?;
                }
                Ok(Value::Nil)
            }

            Statement::Checkpoint { label, span: _ } => {
                self.take_snapshot(Some(label.clone()))?;
                Ok(Value::Nil)
            }

            Statement::Watch {
                expression: _,
                handler: _,
                span: _,
            } => {
                // TODO: Implement watch
                Ok(Value::Nil)
            }

            Statement::Effect {
                dependencies: _,
                body: _,
                span: _,
            } => {
                // TODO: Implement effect
                Ok(Value::Nil)
            }

            Statement::Computed {
                name: _,
                dependencies: _,
                expression: _,
                span: _,
            } => {
                // TODO: Implement computed
                Ok(Value::Nil)
            }

            Statement::Try {
                try_block,
                catch_clause,
                finally_block,
                span: _,
            } => {
                // Execute try block
                let try_result = self.execute_block(try_block);

                // Handle result
                let result = match try_result {
                    Err(InterpreterError::Throw(exception)) => {
                        // Exception was thrown, execute catch if present
                        if let Some(catch) = catch_clause {
                            self.env.push_scope();
                            if let Some(param) = &catch.parameter {
                                self.env.define(param.clone(), exception);
                            }
                            let catch_result = self.execute_block(&catch.body);
                            self.env.pop_scope();
                            catch_result
                        } else {
                            // No catch clause, re-throw
                            Err(InterpreterError::Throw(exception))
                        }
                    }
                    other => other,
                };

                // Execute finally block if present
                if let Some(finally) = finally_block {
                    let _ = self.execute_block(finally);
                }

                result
            }

            Statement::Throw { value, span: _ } => {
                let val = self.evaluate_expression(value)?;
                Err(InterpreterError::Throw(val))
            }

            _ => Ok(Value::Nil),
        }
    }

    /// Execute a block of statements
    fn execute_block(&mut self, statements: &[Statement]) -> Result<Value, InterpreterError> {
        let mut last_value = Value::Nil;
        for statement in statements {
            last_value = self.execute_statement(statement)?;
        }
        Ok(last_value)
    }

    /// Evaluate an expression
    fn evaluate_expression(&mut self, expr: &Expression) -> Result<Value, InterpreterError> {
        match expr {
            Expression::Literal { value, span: _ } => Ok(self.literal_to_value(value)),

            Expression::Identifier { name, span: _ } => {
                // Check if it's a reactive reference
                if let Some(Value::ReactiveRef(_node_id)) = self.env.get(name) {
                    self.reactive_ctx
                        .get(name)
                        .map_err(|e| InterpreterError::RuntimeError(e.to_string()))
                } else {
                    self.env
                        .get(name)
                        .ok_or_else(|| InterpreterError::UndefinedVariable(name.clone()))
                }
            }

            Expression::Binary {
                left,
                operator,
                right,
                span: _,
            } => {
                let left_val = self.evaluate_expression(left)?;
                let right_val = self.evaluate_expression(right)?;
                self.apply_binary_op(&left_val, operator, &right_val)
            }

            Expression::Unary {
                operator,
                operand,
                span: _,
            } => {
                let val = self.evaluate_expression(operand)?;
                self.apply_unary_op(operator, &val)
            }

            Expression::Call {
                callee,
                arguments,
                span: _,
            } => {
                // Check if this is a special builtin (map, filter, reduce)
                if let Expression::Identifier { name, .. } = callee.as_ref() {
                    if name == "map" && arguments.len() == 2 {
                        let array = self.evaluate_expression(&arguments[0])?;
                        let func = self.evaluate_expression(&arguments[1])?;
                        return self.builtin_map(array, func);
                    } else if name == "filter" && arguments.len() == 2 {
                        let array = self.evaluate_expression(&arguments[0])?;
                        let func = self.evaluate_expression(&arguments[1])?;
                        return self.builtin_filter(array, func);
                    } else if name == "reduce" && (arguments.len() == 2 || arguments.len() == 3) {
                        let array = self.evaluate_expression(&arguments[0])?;
                        let func = self.evaluate_expression(&arguments[1])?;
                        let initial = if arguments.len() == 3 {
                            Some(self.evaluate_expression(&arguments[2])?)
                        } else {
                            None
                        };
                        return self.builtin_reduce(array, func, initial);
                    }
                }

                let func = self.evaluate_expression(callee)?;
                let args: Result<Vec<_>, _> = arguments
                    .iter()
                    .map(|arg| self.evaluate_expression(arg))
                    .collect();
                let args = args?;

                self.call_function(func, args)
            }

            Expression::Array { elements, span: _ } => {
                let values: Result<Vec<_>, _> = elements
                    .iter()
                    .map(|elem| self.evaluate_expression(elem))
                    .collect();
                Ok(Value::Array(values?))
            }

            Expression::Object { fields, span: _ } => {
                let mut map = HashMap::new();
                for (key, value_expr) in fields {
                    let value = self.evaluate_expression(value_expr)?;
                    map.insert(key.clone(), value);
                }
                Ok(Value::Object(map))
            }

            Expression::PropertyAccess {
                object,
                property,
                span: _,
            } => {
                let obj = self.evaluate_expression(object)?;
                match obj {
                    Value::Object(map) => map.get(property).cloned().ok_or_else(|| {
                        InterpreterError::RuntimeError(format!("Property '{}' not found", property))
                    }),
                    _ => Err(InterpreterError::TypeError(format!(
                        "Cannot access property on {}",
                        obj.type_name()
                    ))),
                }
            }

            Expression::IndexAccess {
                object,
                index,
                span: _,
            } => {
                let obj = self.evaluate_expression(object)?;
                let idx = self.evaluate_expression(index)?;

                match (obj, idx) {
                    (Value::Array(arr), Value::Integer(i)) => {
                        if i < 0 || i >= arr.len() as i64 {
                            return Err(InterpreterError::IndexOutOfBounds);
                        }
                        Ok(arr[i as usize].clone())
                    }
                    (Value::String(s), Value::Integer(i)) => {
                        if i < 0 || i >= s.len() as i64 {
                            return Err(InterpreterError::IndexOutOfBounds);
                        }
                        Ok(Value::String(
                            s.chars().nth(i as usize).unwrap().to_string(),
                        ))
                    }
                    _ => Err(InterpreterError::TypeError(
                        "Invalid index operation".to_string(),
                    )),
                }
            }

            Expression::Lambda {
                parameters,
                body,
                return_type: _,
                span: _,
            } => {
                let param_names: Vec<String> = parameters.iter().map(|p| p.name.clone()).collect();

                Ok(Value::Function(Arc::new(FunctionValue {
                    name: None,
                    parameters: param_names,
                    body: body.clone(),
                    closure: HashMap::new(),
                    is_async: false,
                })))
            }

            Expression::Range {
                start,
                end,
                inclusive,
                span: _,
            } => {
                let start_val = self.evaluate_expression(start)?;
                let end_val = self.evaluate_expression(end)?;

                match (start_val, end_val) {
                    (Value::Integer(s), Value::Integer(e)) => Ok(Value::Range {
                        start: s,
                        end: e,
                        inclusive: *inclusive,
                    }),
                    _ => Err(InterpreterError::TypeError(
                        "Range bounds must be integers".to_string(),
                    )),
                }
            }

            _ => Ok(Value::Nil),
        }
    }

    /// Convert AST literal to runtime value
    fn literal_to_value(&self, literal: &Literal) -> Value {
        match literal {
            Literal::Integer(n) => Value::Integer(*n),
            Literal::Float(f) => Value::Float(*f),
            Literal::String(s) => Value::String(s.clone()),
            Literal::Boolean(b) => Value::Boolean(*b),
            Literal::Nil => Value::Nil,
        }
    }

    /// Apply binary operation
    fn apply_binary_op(
        &self,
        left: &Value,
        op: &BinaryOp,
        right: &Value,
    ) -> Result<Value, InterpreterError> {
        use std::cmp::Ordering;

        match op {
            BinaryOp::Add => left.add(right).map_err(InterpreterError::RuntimeError),
            BinaryOp::Subtract => left.subtract(right).map_err(InterpreterError::RuntimeError),
            BinaryOp::Multiply => left.multiply(right).map_err(InterpreterError::RuntimeError),
            BinaryOp::Divide => left.divide(right).map_err(InterpreterError::RuntimeError),
            BinaryOp::Modulo => left.modulo(right).map_err(InterpreterError::RuntimeError),
            BinaryOp::Power => left.power(right).map_err(InterpreterError::RuntimeError),
            BinaryOp::Equal => Ok(Value::Boolean(left == right)),
            BinaryOp::NotEqual => Ok(Value::Boolean(left != right)),
            BinaryOp::Less => Ok(Value::Boolean(
                left.compare(right)
                    .map_err(InterpreterError::RuntimeError)?
                    == Ordering::Less,
            )),
            BinaryOp::LessEqual => Ok(Value::Boolean(
                left.compare(right)
                    .map_err(InterpreterError::RuntimeError)?
                    != Ordering::Greater,
            )),
            BinaryOp::Greater => Ok(Value::Boolean(
                left.compare(right)
                    .map_err(InterpreterError::RuntimeError)?
                    == Ordering::Greater,
            )),
            BinaryOp::GreaterEqual => Ok(Value::Boolean(
                left.compare(right)
                    .map_err(InterpreterError::RuntimeError)?
                    != Ordering::Less,
            )),
            BinaryOp::And => Ok(Value::Boolean(left.is_truthy() && right.is_truthy())),
            BinaryOp::Or => Ok(Value::Boolean(left.is_truthy() || right.is_truthy())),
            _ => Err(InterpreterError::InvalidOperation(format!(
                "Binary operation {} not implemented",
                op
            ))),
        }
    }

    /// Apply unary operation
    fn apply_unary_op(&self, op: &UnaryOp, val: &Value) -> Result<Value, InterpreterError> {
        match op {
            UnaryOp::Negate => match val {
                Value::Integer(n) => Ok(Value::Integer(-n)),
                Value::Float(f) => Ok(Value::Float(-f)),
                _ => Err(InterpreterError::TypeError(format!(
                    "Cannot negate {}",
                    val.type_name()
                ))),
            },
            UnaryOp::Not => Ok(Value::Boolean(!val.is_truthy())),
            UnaryOp::BitwiseNot => match val {
                Value::Integer(n) => Ok(Value::Integer(!n)),
                _ => Err(InterpreterError::TypeError(format!(
                    "Cannot apply bitwise not to {}",
                    val.type_name()
                ))),
            },
        }
    }

    /// Call a function
    fn call_function(&mut self, func: Value, args: Vec<Value>) -> Result<Value, InterpreterError> {
        match func {
            Value::Function(func_val) => {
                if args.len() != func_val.parameters.len() {
                    return Err(InterpreterError::RuntimeError(format!(
                        "Expected {} arguments, got {}",
                        func_val.parameters.len(),
                        args.len()
                    )));
                }

                self.env.push_scope();

                // Bind parameters
                for (param, arg) in func_val.parameters.iter().zip(args.iter()) {
                    self.env.define(param.clone(), arg.clone());
                }

                // Execute function body
                let result = match self.execute_block(&func_val.body) {
                    Err(InterpreterError::ReturnValue(val)) => Ok(val),
                    Err(e) => Err(e),
                    Ok(_) => Ok(Value::Nil),
                };

                self.env.pop_scope();
                result
            }
            Value::Native(native_fn) => native_fn(args).map_err(InterpreterError::RuntimeError),
            _ => Err(InterpreterError::TypeError(format!(
                "Cannot call {}",
                func.type_name()
            ))),
        }
    }

    /// Helper for map operation
    fn builtin_map(&mut self, array: Value, func: Value) -> Result<Value, InterpreterError> {
        match array {
            Value::Array(arr) => {
                let mut result = Vec::new();
                for item in arr {
                    let mapped = self.call_function(func.clone(), vec![item])?;
                    result.push(mapped);
                }
                Ok(Value::Array(result))
            }
            _ => Err(InterpreterError::TypeError(
                "map expects an array".to_string(),
            )),
        }
    }

    /// Helper for filter operation
    fn builtin_filter(&mut self, array: Value, func: Value) -> Result<Value, InterpreterError> {
        match array {
            Value::Array(arr) => {
                let mut result = Vec::new();
                for item in arr {
                    let keep = self.call_function(func.clone(), vec![item.clone()])?;
                    if keep.is_truthy() {
                        result.push(item);
                    }
                }
                Ok(Value::Array(result))
            }
            _ => Err(InterpreterError::TypeError(
                "filter expects an array".to_string(),
            )),
        }
    }

    /// Helper for reduce operation
    fn builtin_reduce(
        &mut self,
        array: Value,
        func: Value,
        initial: Option<Value>,
    ) -> Result<Value, InterpreterError> {
        match array {
            Value::Array(arr) => {
                if arr.is_empty() {
                    return initial.ok_or_else(|| {
                        InterpreterError::RuntimeError(
                            "reduce of empty array with no initial value".to_string(),
                        )
                    });
                }

                let has_initial = initial.is_some();
                let mut accumulator = if let Some(init) = initial {
                    init
                } else {
                    arr[0].clone()
                };

                let start_index = if has_initial { 0 } else { 1 };

                for item in arr.iter().skip(start_index) {
                    accumulator =
                        self.call_function(func.clone(), vec![accumulator, item.clone()])?;
                }

                Ok(accumulator)
            }
            _ => Err(InterpreterError::TypeError(
                "reduce expects an array".to_string(),
            )),
        }
    }

    /// Take a snapshot for time-travel debugging
    fn take_snapshot(&mut self, label: Option<String>) -> Result<usize, InterpreterError> {
        let state = self.env.to_persistent();
        let call_stack = vec![]; // TODO: Track actual call stack

        self.time_travel
            .write()
            .unwrap()
            .snapshot(
                state,
                call_stack,
                self.current_line,
                self.current_file.clone(),
                label,
            )
            .map_err(|e| InterpreterError::RuntimeError(e.to_string()))
    }

    /// Rewind to a previous state
    fn rewind_steps(&mut self, steps: usize) -> Result<(), InterpreterError> {
        let _snapshot = self
            .time_travel
            .write()
            .unwrap()
            .rewind(steps)
            .map_err(|e| InterpreterError::RuntimeError(e.to_string()))?;

        // Restore state
        // TODO: Properly restore environment from snapshot
        Ok(())
    }

    /// Rewind to a checkpoint
    fn rewind_to_checkpoint(&mut self, label: &str) -> Result<(), InterpreterError> {
        let _snapshot = self
            .time_travel
            .write()
            .unwrap()
            .jump_to_checkpoint(label)
            .map_err(|e| InterpreterError::RuntimeError(e.to_string()))?;

        // Restore state
        // TODO: Properly restore environment from snapshot
        Ok(())
    }

    /// Get the reactive context
    pub fn reactive_context(&self) -> &Arc<ReactiveContext> {
        &self.reactive_ctx
    }

    /// Get the time-travel debugger
    pub fn time_travel_debugger(&self) -> &Arc<RwLock<TimeTravelDebugger>> {
        &self.time_travel
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        let mut interp = Interpreter::new();
        let expr = Expression::Binary {
            left: Box::new(Expression::Literal {
                value: Literal::Integer(5),
                span: Span::dummy(),
            }),
            operator: BinaryOp::Add,
            right: Box::new(Expression::Literal {
                value: Literal::Integer(3),
                span: Span::dummy(),
            }),
            span: Span::dummy(),
        };

        let result = interp.evaluate_expression(&expr).unwrap();
        assert_eq!(result, Value::Integer(8));
    }

    #[test]
    fn test_variable_definition() {
        let mut interp = Interpreter::new();
        let stmt = Statement::Let {
            name: "x".to_string(),
            value: Expression::Literal {
                value: Literal::Integer(42),
                span: Span::dummy(),
            },
            type_annotation: None,
            span: Span::dummy(),
        };

        interp.execute_statement(&stmt).unwrap();
        let value = interp.env.get("x").unwrap();
        assert_eq!(value, Value::Integer(42));
    }

    #[test]
    fn test_function_call() {
        let mut interp = Interpreter::new();

        // Define function: fn add(a, b) { return a + b; }
        let func_stmt = Statement::Function {
            name: "add".to_string(),
            parameters: vec![
                Parameter {
                    name: "a".to_string(),
                    type_annotation: None,
                    default_value: None,
                    span: Span::dummy(),
                },
                Parameter {
                    name: "b".to_string(),
                    type_annotation: None,
                    default_value: None,
                    span: Span::dummy(),
                },
            ],
            body: vec![Statement::Return {
                value: Some(Expression::Binary {
                    left: Box::new(Expression::Identifier {
                        name: "a".to_string(),
                        span: Span::dummy(),
                    }),
                    operator: BinaryOp::Add,
                    right: Box::new(Expression::Identifier {
                        name: "b".to_string(),
                        span: Span::dummy(),
                    }),
                    span: Span::dummy(),
                }),
                span: Span::dummy(),
            }],
            return_type: None,
            is_async: false,
            span: Span::dummy(),
        };

        interp.execute_statement(&func_stmt).unwrap();

        // Call function
        let call_expr = Expression::Call {
            callee: Box::new(Expression::Identifier {
                name: "add".to_string(),
                span: Span::dummy(),
            }),
            arguments: vec![
                Expression::Literal {
                    value: Literal::Integer(10),
                    span: Span::dummy(),
                },
                Expression::Literal {
                    value: Literal::Integer(20),
                    span: Span::dummy(),
                },
            ],
            span: Span::dummy(),
        };

        let result = interp.evaluate_expression(&call_expr).unwrap();
        assert_eq!(result, Value::Integer(30));
    }
}
