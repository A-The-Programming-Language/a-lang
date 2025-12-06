//! FFI (Foreign Function Interface) module for A-lang
//! Provides interoperability with C/C++ code

use crate::interpreter::value::Value;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_int, c_void};
use std::sync::{Arc, Mutex};

#[cfg(unix)]
use libloading::Library;

/// FFI function signature types
#[derive(Debug, Clone, PartialEq)]
pub enum FFIType {
    Void,
    Int,
    Long,
    Float,
    Double,
    String,
    Pointer,
    Bool,
}

impl FFIType {
    pub fn from_string(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "void" => Ok(FFIType::Void),
            "int" | "i32" => Ok(FFIType::Int),
            "long" | "i64" => Ok(FFIType::Long),
            "float" | "f32" => Ok(FFIType::Float),
            "double" | "f64" => Ok(FFIType::Double),
            "string" | "str" | "char*" => Ok(FFIType::String),
            "pointer" | "ptr" | "void*" => Ok(FFIType::Pointer),
            "bool" | "boolean" => Ok(FFIType::Bool),
            _ => Err(format!("Unknown FFI type: {}", s)),
        }
    }
}

/// FFI function signature
#[derive(Debug, Clone)]
pub struct FFISignature {
    pub return_type: FFIType,
    pub param_types: Vec<FFIType>,
}

/// FFI library handle
pub struct FFILibrary {
    #[cfg(unix)]
    lib: Library,
    name: String,
}

/// FFI context for managing loaded libraries and functions
pub struct FFIContext {
    libraries: HashMap<String, Arc<Mutex<FFILibrary>>>,
    signatures: HashMap<String, FFISignature>,
}

impl FFIContext {
    pub fn new() -> Self {
        Self {
            libraries: HashMap::new(),
            signatures: HashMap::new(),
        }
    }

    /// Load a dynamic library
    pub fn load_library(&mut self, name: &str, path: &str) -> Result<(), String> {
        #[cfg(unix)]
        {
            match unsafe { Library::new(path) } {
                Ok(lib) => {
                    self.libraries.insert(
                        name.to_string(),
                        Arc::new(Mutex::new(FFILibrary {
                            lib,
                            name: name.to_string(),
                        })),
                    );
                    Ok(())
                }
                Err(e) => Err(format!("Failed to load library '{}': {}", path, e)),
            }
        }

        #[cfg(not(unix))]
        {
            Err("FFI is only supported on Unix-like systems currently".to_string())
        }
    }

    /// Register a function signature
    pub fn register_signature(
        &mut self,
        name: &str,
        return_type: FFIType,
        param_types: Vec<FFIType>,
    ) {
        self.signatures.insert(
            name.to_string(),
            FFISignature {
                return_type,
                param_types,
            },
        );
    }

    /// Call a foreign function
    pub fn call_function(
        &self,
        lib_name: &str,
        func_name: &str,
        args: Vec<Value>,
    ) -> Result<Value, String> {
        // Get the signature
        let signature = self
            .signatures
            .get(func_name)
            .ok_or_else(|| format!("Function '{}' not registered", func_name))?;

        // Validate argument count
        if args.len() != signature.param_types.len() {
            return Err(format!(
                "Function '{}' expects {} arguments, got {}",
                func_name,
                signature.param_types.len(),
                args.len()
            ));
        }

        // Get the library
        let lib = self
            .libraries
            .get(lib_name)
            .ok_or_else(|| format!("Library '{}' not loaded", lib_name))?;

        #[cfg(unix)]
        {
            let lib = lib.lock().unwrap();

            // Match on return type and parameter types
            match (&signature.return_type, signature.param_types.as_slice()) {
                // int func(int)
                (FFIType::Int, [FFIType::Int]) => {
                    type FuncType = unsafe extern "C" fn(c_int) -> c_int;
                    unsafe {
                        let func: libloading::Symbol<FuncType> = lib
                            .lib
                            .get(func_name.as_bytes())
                            .map_err(|e| format!("Function '{}' not found: {}", func_name, e))?;
                        let arg = value_to_c_int(&args[0])?;
                        Ok(c_int_to_value(func(arg)))
                    }
                }

                // int func(int, int)
                (FFIType::Int, [FFIType::Int, FFIType::Int]) => {
                    type FuncType = unsafe extern "C" fn(c_int, c_int) -> c_int;
                    unsafe {
                        let func: libloading::Symbol<FuncType> = lib
                            .lib
                            .get(func_name.as_bytes())
                            .map_err(|e| format!("Function '{}' not found: {}", func_name, e))?;
                        let arg1 = value_to_c_int(&args[0])?;
                        let arg2 = value_to_c_int(&args[1])?;
                        Ok(c_int_to_value(func(arg1, arg2)))
                    }
                }

                // int func(string)
                (FFIType::Int, [FFIType::String]) => {
                    type FuncType = unsafe extern "C" fn(*const c_char) -> c_int;
                    unsafe {
                        let func: libloading::Symbol<FuncType> = lib
                            .lib
                            .get(func_name.as_bytes())
                            .map_err(|e| format!("Function '{}' not found: {}", func_name, e))?;
                        let c_str = value_to_c_string(&args[0])?;
                        Ok(c_int_to_value(func(c_str.as_ptr())))
                    }
                }

                // double func(double)
                (FFIType::Double, [FFIType::Double]) => {
                    type FuncType = unsafe extern "C" fn(c_double) -> c_double;
                    unsafe {
                        let func: libloading::Symbol<FuncType> = lib
                            .lib
                            .get(func_name.as_bytes())
                            .map_err(|e| format!("Function '{}' not found: {}", func_name, e))?;
                        let arg = value_to_c_double(&args[0])?;
                        Ok(c_double_to_value(func(arg)))
                    }
                }

                // double func(double, double)
                (FFIType::Double, [FFIType::Double, FFIType::Double]) => {
                    type FuncType = unsafe extern "C" fn(c_double, c_double) -> c_double;
                    unsafe {
                        let func: libloading::Symbol<FuncType> = lib
                            .lib
                            .get(func_name.as_bytes())
                            .map_err(|e| format!("Function '{}' not found: {}", func_name, e))?;
                        let arg1 = value_to_c_double(&args[0])?;
                        let arg2 = value_to_c_double(&args[1])?;
                        Ok(c_double_to_value(func(arg1, arg2)))
                    }
                }

                // float func(float)
                (FFIType::Float, [FFIType::Float]) => {
                    type FuncType = unsafe extern "C" fn(f32) -> f32;
                    unsafe {
                        let func: libloading::Symbol<FuncType> = lib
                            .lib
                            .get(func_name.as_bytes())
                            .map_err(|e| format!("Function '{}' not found: {}", func_name, e))?;
                        let arg = args[0].as_float().ok_or("Expected float argument")? as f32;
                        Ok(Value::Float(func(arg) as f64))
                    }
                }

                // void func()
                (FFIType::Void, []) => {
                    type FuncType = unsafe extern "C" fn();
                    unsafe {
                        let func: libloading::Symbol<FuncType> = lib
                            .lib
                            .get(func_name.as_bytes())
                            .map_err(|e| format!("Function '{}' not found: {}", func_name, e))?;
                        func();
                        Ok(Value::Nil)
                    }
                }

                // void func(int)
                (FFIType::Void, [FFIType::Int]) => {
                    type FuncType = unsafe extern "C" fn(c_int);
                    unsafe {
                        let func: libloading::Symbol<FuncType> = lib
                            .lib
                            .get(func_name.as_bytes())
                            .map_err(|e| format!("Function '{}' not found: {}", func_name, e))?;
                        let arg = value_to_c_int(&args[0])?;
                        func(arg);
                        Ok(Value::Nil)
                    }
                }

                // void func(string)
                (FFIType::Void, [FFIType::String]) => {
                    type FuncType = unsafe extern "C" fn(*const c_char);
                    unsafe {
                        let func: libloading::Symbol<FuncType> = lib
                            .lib
                            .get(func_name.as_bytes())
                            .map_err(|e| format!("Function '{}' not found: {}", func_name, e))?;
                        let c_str = value_to_c_string(&args[0])?;
                        func(c_str.as_ptr());
                        Ok(Value::Nil)
                    }
                }

                // void func(int, int)
                (FFIType::Void, [FFIType::Int, FFIType::Int]) => {
                    type FuncType = unsafe extern "C" fn(c_int, c_int);
                    unsafe {
                        let func: libloading::Symbol<FuncType> = lib
                            .lib
                            .get(func_name.as_bytes())
                            .map_err(|e| format!("Function '{}' not found: {}", func_name, e))?;
                        let arg1 = value_to_c_int(&args[0])?;
                        let arg2 = value_to_c_int(&args[1])?;
                        func(arg1, arg2);
                        Ok(Value::Nil)
                    }
                }

                // long func(long)
                (FFIType::Long, [FFIType::Long]) => {
                    type FuncType = unsafe extern "C" fn(i64) -> i64;
                    unsafe {
                        let func: libloading::Symbol<FuncType> = lib
                            .lib
                            .get(func_name.as_bytes())
                            .map_err(|e| format!("Function '{}' not found: {}", func_name, e))?;
                        let arg = args[0].as_integer().ok_or("Expected integer argument")?;
                        Ok(Value::Integer(func(arg)))
                    }
                }

                // bool func(bool)
                (FFIType::Bool, [FFIType::Bool]) => {
                    type FuncType = unsafe extern "C" fn(bool) -> bool;
                    unsafe {
                        let func: libloading::Symbol<FuncType> = lib
                            .lib
                            .get(func_name.as_bytes())
                            .map_err(|e| format!("Function '{}' not found: {}", func_name, e))?;
                        let arg = match &args[0] {
                            Value::Boolean(b) => *b,
                            _ => return Err("Expected boolean argument".to_string()),
                        };
                        Ok(Value::Boolean(func(arg)))
                    }
                }

                _ => Err(format!(
                    "Unsupported function signature: {:?} ({:?})",
                    signature.return_type, signature.param_types
                )),
            }
        }

        #[cfg(not(unix))]
        {
            Err("FFI is only supported on Unix-like systems currently".to_string())
        }
    }
}

impl Default for FFIContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert A-lang Value to C types
pub fn value_to_c_int(value: &Value) -> Result<c_int, String> {
    value
        .as_integer()
        .map(|i| i as c_int)
        .ok_or_else(|| "Expected integer value".to_string())
}

pub fn value_to_c_double(value: &Value) -> Result<c_double, String> {
    value
        .as_float()
        .map(|f| f as c_double)
        .ok_or_else(|| "Expected float value".to_string())
}

pub fn value_to_c_string(value: &Value) -> Result<CString, String> {
    match value {
        Value::String(s) => {
            CString::new(s.as_str()).map_err(|e| format!("Invalid C string: {}", e))
        }
        _ => Err("Expected string value".to_string()),
    }
}

/// Convert C types to A-lang Value
pub fn c_int_to_value(val: c_int) -> Value {
    Value::Integer(val as i64)
}

pub fn c_double_to_value(val: c_double) -> Value {
    Value::Float(val as f64)
}

pub unsafe fn c_string_to_value(ptr: *const c_char) -> Result<Value, String> {
    if ptr.is_null() {
        return Ok(Value::Nil);
    }

    let c_str = CStr::from_ptr(ptr);
    match c_str.to_str() {
        Ok(s) => Ok(Value::String(s.to_string())),
        Err(e) => Err(format!("Invalid UTF-8 string: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffi_type_parsing() {
        assert_eq!(FFIType::from_string("int").unwrap(), FFIType::Int);
        assert_eq!(FFIType::from_string("double").unwrap(), FFIType::Double);
        assert_eq!(FFIType::from_string("string").unwrap(), FFIType::String);
        assert_eq!(FFIType::from_string("void").unwrap(), FFIType::Void);
    }

    #[test]
    fn test_ffi_context_creation() {
        let ctx = FFIContext::new();
        assert_eq!(ctx.libraries.len(), 0);
        assert_eq!(ctx.signatures.len(), 0);
    }

    #[test]
    fn test_register_signature() {
        let mut ctx = FFIContext::new();
        ctx.register_signature(
            "test_func",
            FFIType::Int,
            vec![FFIType::Int, FFIType::Double],
        );

        assert!(ctx.signatures.contains_key("test_func"));
        let sig = ctx.signatures.get("test_func").unwrap();
        assert_eq!(sig.return_type, FFIType::Int);
        assert_eq!(sig.param_types.len(), 2);
    }

    #[test]
    fn test_value_conversions() {
        let int_val = Value::Integer(42);
        assert_eq!(value_to_c_int(&int_val).unwrap(), 42);

        let float_val = Value::Float(3.14);
        assert!((value_to_c_double(&float_val).unwrap() - 3.14).abs() < 0.001);

        let str_val = Value::String("hello".to_string());
        let c_str = value_to_c_string(&str_val).unwrap();
        assert_eq!(c_str.to_str().unwrap(), "hello");
    }
}
