//! Integration module for registering stdlib functions with the A-lang interpreter
//! This module bridges the stdlib modules with the interpreter's native function system

use crate::interpreter::value::Value;
use crate::stdlib::{
    bytes::{BinaryEncoder, BitOps, ByteBuffer, ByteOrder, StructPacker},
    ffi::FFIContext,
    hardware::{HardwareManager, PinMode, PinState, SpiMode, UartConfig, UartParity},
    network::{HttpClient, HttpMethod, HttpRequest, NetUtils, TcpConnection, UdpSocketWrapper},
    system::{PathUtils, SystemUtils, Timer},
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Global context for stateful stdlib components
pub struct StdlibContext {
    pub ffi: Arc<Mutex<FFIContext>>,
    pub hardware: Arc<HardwareManager>,
    pub tcp_connections: Arc<Mutex<HashMap<String, TcpConnection>>>,
    pub udp_sockets: Arc<Mutex<HashMap<String, UdpSocketWrapper>>>,
    pub byte_buffers: Arc<Mutex<HashMap<String, ByteBuffer>>>,
    pub timers: Arc<Mutex<HashMap<String, Timer>>>,
}

impl StdlibContext {
    pub fn new() -> Self {
        Self {
            ffi: Arc::new(Mutex::new(FFIContext::new())),
            hardware: Arc::new(HardwareManager::new()),
            tcp_connections: Arc::new(Mutex::new(HashMap::new())),
            udp_sockets: Arc::new(Mutex::new(HashMap::new())),
            byte_buffers: Arc::new(Mutex::new(HashMap::new())),
            timers: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Default for StdlibContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Register all stdlib native functions
pub fn register_stdlib_functions() -> Vec<(String, Value)> {
    let mut functions = Vec::new();

    // Network functions
    functions.extend(register_network_functions());

    // System functions
    functions.extend(register_system_functions());

    // Bytes functions
    functions.extend(register_bytes_functions());

    // Hardware functions
    functions.extend(register_hardware_functions());

    // FFI functions (basic)
    functions.extend(register_ffi_functions());

    functions
}

/// Register network-related functions
fn register_network_functions() -> Vec<(String, Value)> {
    vec![
        // HTTP GET
        (
            "httpGet".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("httpGet expects 1 argument (url)".to_string());
                }

                match &args[0] {
                    Value::String(url) => {
                        let client = HttpClient::new();
                        match client.get(url) {
                            Ok(response) => Ok(response.to_value()),
                            Err(e) => Err(format!("HTTP GET failed: {}", e)),
                        }
                    }
                    _ => Err("httpGet expects a string URL".to_string()),
                }
            })),
        ),
        // HTTP POST
        (
            "httpPost".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 2 {
                    return Err("httpPost expects 2 arguments (url, body)".to_string());
                }

                match (&args[0], &args[1]) {
                    (Value::String(url), Value::String(body)) => {
                        let client = HttpClient::new();
                        match client.post(url, body.as_bytes().to_vec()) {
                            Ok(response) => Ok(response.to_value()),
                            Err(e) => Err(format!("HTTP POST failed: {}", e)),
                        }
                    }
                    _ => Err("httpPost expects (string, string)".to_string()),
                }
            })),
        ),
        // Parse URL
        (
            "parseUrl".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("parseUrl expects 1 argument (url)".to_string());
                }

                match &args[0] {
                    Value::String(url) => match NetUtils::parse_url(url) {
                        Ok(parts) => {
                            let mut obj = HashMap::new();
                            for (k, v) in parts {
                                obj.insert(k, Value::String(v));
                            }
                            Ok(Value::Object(obj))
                        }
                        Err(e) => Err(format!("Failed to parse URL: {}", e)),
                    },
                    _ => Err("parseUrl expects a string URL".to_string()),
                }
            })),
        ),
        // Get local IP
        (
            "getLocalIp".to_string(),
            Value::Native(Arc::new(|args| {
                if !args.is_empty() {
                    return Err("getLocalIp expects no arguments".to_string());
                }

                match NetUtils::get_local_ip() {
                    Ok(ip) => Ok(Value::String(ip)),
                    Err(e) => Err(format!("Failed to get local IP: {}", e)),
                }
            })),
        ),
        // Check if port is available
        (
            "isPortAvailable".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("isPortAvailable expects 1 argument (port)".to_string());
                }

                match args[0].as_integer() {
                    Some(port) => {
                        if port < 0 || port > 65535 {
                            return Err("Port must be between 0 and 65535".to_string());
                        }
                        Ok(Value::Boolean(NetUtils::is_port_available(port as u16)))
                    }
                    None => Err("isPortAvailable expects an integer port".to_string()),
                }
            })),
        ),
    ]
}

/// Register system-related functions
fn register_system_functions() -> Vec<(String, Value)> {
    vec![
        // Execute command
        (
            "exec".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("exec expects 1 argument (command)".to_string());
                }

                match &args[0] {
                    Value::String(cmd) => match SystemUtils::exec(cmd) {
                        Ok(result) => Ok(result.to_value()),
                        Err(e) => Err(format!("Exec failed: {}", e)),
                    },
                    _ => Err("exec expects a string command".to_string()),
                }
            })),
        ),
        // Get environment variable
        (
            "getEnv".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("getEnv expects 1 argument (key)".to_string());
                }

                match &args[0] {
                    Value::String(key) => Ok(SystemUtils::get_env(key)
                        .map(Value::String)
                        .unwrap_or(Value::Nil)),
                    _ => Err("getEnv expects a string key".to_string()),
                }
            })),
        ),
        // Set environment variable
        (
            "setEnv".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 2 {
                    return Err("setEnv expects 2 arguments (key, value)".to_string());
                }

                match (&args[0], &args[1]) {
                    (Value::String(key), Value::String(value)) => {
                        SystemUtils::set_env(key, value);
                        Ok(Value::Nil)
                    }
                    _ => Err("setEnv expects (string, string)".to_string()),
                }
            })),
        ),
        // Get system info
        (
            "getSystemInfo".to_string(),
            Value::Native(Arc::new(|args| {
                if !args.is_empty() {
                    return Err("getSystemInfo expects no arguments".to_string());
                }

                let info = SystemUtils::get_system_info();
                let mut obj = HashMap::new();
                for (k, v) in info {
                    obj.insert(k, Value::String(v));
                }
                Ok(Value::Object(obj))
            })),
        ),
        // Get current working directory
        (
            "getCwd".to_string(),
            Value::Native(Arc::new(|args| {
                if !args.is_empty() {
                    return Err("getCwd expects no arguments".to_string());
                }

                match SystemUtils::get_cwd() {
                    Ok(cwd) => Ok(Value::String(cwd)),
                    Err(e) => Err(format!("Failed to get CWD: {}", e)),
                }
            })),
        ),
        // Change working directory
        (
            "setCwd".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("setCwd expects 1 argument (path)".to_string());
                }

                match &args[0] {
                    Value::String(path) => match SystemUtils::set_cwd(path) {
                        Ok(_) => Ok(Value::Nil),
                        Err(e) => Err(format!("Failed to set CWD: {}", e)),
                    },
                    _ => Err("setCwd expects a string path".to_string()),
                }
            })),
        ),
        // Get timestamp
        (
            "timestamp".to_string(),
            Value::Native(Arc::new(|args| {
                if !args.is_empty() {
                    return Err("timestamp expects no arguments".to_string());
                }

                Ok(Value::Integer(SystemUtils::get_timestamp() as i64))
            })),
        ),
        // Sleep
        (
            "sleep".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("sleep expects 1 argument (milliseconds)".to_string());
                }

                match args[0].as_integer() {
                    Some(ms) => {
                        if ms < 0 {
                            return Err("Sleep duration must be non-negative".to_string());
                        }
                        SystemUtils::sleep(ms as u64);
                        Ok(Value::Nil)
                    }
                    None => Err("sleep expects an integer duration".to_string()),
                }
            })),
        ),
        // Path utilities
        (
            "pathJoin".to_string(),
            Value::Native(Arc::new(|args| {
                let components: Vec<String> = args
                    .iter()
                    .map(|v| match v {
                        Value::String(s) => Ok(s.clone()),
                        _ => Err("pathJoin expects string arguments".to_string()),
                    })
                    .collect::<Result<Vec<_>, String>>()?;

                Ok(Value::String(PathUtils::join(&components)))
            })),
        ),
        (
            "pathBasename".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("pathBasename expects 1 argument".to_string());
                }

                match &args[0] {
                    Value::String(path) => Ok(PathUtils::basename(path)
                        .map(Value::String)
                        .unwrap_or(Value::Nil)),
                    _ => Err("pathBasename expects a string path".to_string()),
                }
            })),
        ),
    ]
}

/// Register bytes/binary manipulation functions
fn register_bytes_functions() -> Vec<(String, Value)> {
    vec![
        // Hex encoding
        (
            "toHex".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("toHex expects 1 argument (bytes array)".to_string());
                }

                match &args[0] {
                    Value::Array(arr) => {
                        let bytes: Result<Vec<u8>, String> = arr
                            .iter()
                            .map(|v| {
                                v.as_integer()
                                    .and_then(|i| {
                                        if i >= 0 && i <= 255 {
                                            Some(i as u8)
                                        } else {
                                            None
                                        }
                                    })
                                    .ok_or_else(|| "Array must contain bytes (0-255)".to_string())
                            })
                            .collect();

                        match bytes {
                            Ok(bytes) => Ok(Value::String(BinaryEncoder::to_hex(&bytes))),
                            Err(e) => Err(e),
                        }
                    }
                    _ => Err("toHex expects an array of bytes".to_string()),
                }
            })),
        ),
        // Hex decoding
        (
            "fromHex".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("fromHex expects 1 argument (hex string)".to_string());
                }

                match &args[0] {
                    Value::String(hex) => match BinaryEncoder::from_hex(hex) {
                        Ok(bytes) => {
                            let array: Vec<Value> =
                                bytes.iter().map(|&b| Value::Integer(b as i64)).collect();
                            Ok(Value::Array(array))
                        }
                        Err(e) => Err(format!("Hex decode failed: {}", e)),
                    },
                    _ => Err("fromHex expects a string".to_string()),
                }
            })),
        ),
        // Base64 encoding
        (
            "toBase64".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("toBase64 expects 1 argument".to_string());
                }

                match &args[0] {
                    Value::String(s) => Ok(Value::String(BinaryEncoder::to_base64(s.as_bytes()))),
                    Value::Array(arr) => {
                        let bytes: Result<Vec<u8>, String> = arr
                            .iter()
                            .map(|v| {
                                v.as_integer()
                                    .and_then(|i| {
                                        if i >= 0 && i <= 255 {
                                            Some(i as u8)
                                        } else {
                                            None
                                        }
                                    })
                                    .ok_or_else(|| "Array must contain bytes (0-255)".to_string())
                            })
                            .collect();

                        match bytes {
                            Ok(bytes) => Ok(Value::String(BinaryEncoder::to_base64(&bytes))),
                            Err(e) => Err(e),
                        }
                    }
                    _ => Err("toBase64 expects a string or byte array".to_string()),
                }
            })),
        ),
        // Base64 decoding
        (
            "fromBase64".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("fromBase64 expects 1 argument (base64 string)".to_string());
                }

                match &args[0] {
                    Value::String(b64) => match BinaryEncoder::from_base64(b64) {
                        Ok(bytes) => {
                            let array: Vec<Value> =
                                bytes.iter().map(|&b| Value::Integer(b as i64)).collect();
                            Ok(Value::Array(array))
                        }
                        Err(e) => Err(format!("Base64 decode failed: {}", e)),
                    },
                    _ => Err("fromBase64 expects a string".to_string()),
                }
            })),
        ),
        // Bit operations
        (
            "bitGet".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 2 {
                    return Err("bitGet expects 2 arguments (value, bit)".to_string());
                }

                match (args[0].as_integer(), args[1].as_integer()) {
                    (Some(value), Some(bit)) => {
                        if bit < 0 || bit >= 64 {
                            return Err("Bit index must be 0-63".to_string());
                        }
                        Ok(Value::Boolean(BitOps::get_bit(value as u64, bit as u8)))
                    }
                    _ => Err("bitGet expects (integer, integer)".to_string()),
                }
            })),
        ),
        (
            "bitSet".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 2 {
                    return Err("bitSet expects 2 arguments (value, bit)".to_string());
                }

                match (args[0].as_integer(), args[1].as_integer()) {
                    (Some(value), Some(bit)) => {
                        if bit < 0 || bit >= 64 {
                            return Err("Bit index must be 0-63".to_string());
                        }
                        Ok(Value::Integer(
                            BitOps::set_bit(value as u64, bit as u8) as i64
                        ))
                    }
                    _ => Err("bitSet expects (integer, integer)".to_string()),
                }
            })),
        ),
    ]
}

/// Register hardware simulation functions (GPIO, I2C, SPI, UART)
fn register_hardware_functions() -> Vec<(String, Value)> {
    vec![
        // GPIO functions
        (
            "gpioInit".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 2 {
                    return Err("gpioInit expects 2 arguments (pin, mode)".to_string());
                }

                // This is a placeholder - in real implementation, we'd need access to HardwareManager
                Ok(Value::Nil)
            })),
        ),
        (
            "gpioWrite".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 2 {
                    return Err("gpioWrite expects 2 arguments (pin, state)".to_string());
                }

                // Placeholder
                Ok(Value::Nil)
            })),
        ),
        (
            "gpioRead".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 1 {
                    return Err("gpioRead expects 1 argument (pin)".to_string());
                }

                // Placeholder - returns LOW
                Ok(Value::Integer(0))
            })),
        ),
    ]
}

/// Register FFI functions
fn register_ffi_functions() -> Vec<(String, Value)> {
    vec![
        // Load library
        (
            "ffiLoadLibrary".to_string(),
            Value::Native(Arc::new(|args| {
                if args.len() != 2 {
                    return Err("ffiLoadLibrary expects 2 arguments (name, path)".to_string());
                }

                match (&args[0], &args[1]) {
                    (Value::String(_name), Value::String(_path)) => {
                        // Placeholder - FFI requires more complex integration
                        Ok(Value::Boolean(true))
                    }
                    _ => Err("ffiLoadLibrary expects (string, string)".to_string()),
                }
            })),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_functions() {
        let functions = register_stdlib_functions();
        assert!(!functions.is_empty());

        // Check that we have key functions
        let function_names: Vec<String> = functions.iter().map(|(name, _)| name.clone()).collect();
        assert!(function_names.contains(&"httpGet".to_string()));
        assert!(function_names.contains(&"exec".to_string()));
        assert!(function_names.contains(&"toHex".to_string()));
    }

    #[test]
    fn test_stdlib_context_creation() {
        let _ctx = StdlibContext::new();
        // Just ensure it can be created
    }
}
