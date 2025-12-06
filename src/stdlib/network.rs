//! Network module for A-lang
//! Provides TCP/UDP sockets, HTTP client, and networking utilities

use crate::interpreter::value::Value;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs, UdpSocket};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Network socket types
#[derive(Debug, Clone, PartialEq)]
pub enum SocketType {
    Tcp,
    Udp,
}

/// TCP connection wrapper
pub struct TcpConnection {
    stream: TcpStream,
    address: String,
}

impl TcpConnection {
    pub fn new(stream: TcpStream, address: String) -> Self {
        Self { stream, address }
    }

    pub fn connect(addr: &str) -> Result<Self, String> {
        let stream = TcpStream::connect(addr)
            .map_err(|e| format!("Failed to connect to {}: {}", addr, e))?;

        Ok(Self {
            stream,
            address: addr.to_string(),
        })
    }

    pub fn send(&mut self, data: &[u8]) -> Result<usize, String> {
        self.stream
            .write(data)
            .map_err(|e| format!("Failed to send data: {}", e))
    }

    pub fn receive(&mut self, buffer_size: usize) -> Result<Vec<u8>, String> {
        let mut buffer = vec![0u8; buffer_size];
        match self.stream.read(&mut buffer) {
            Ok(n) => {
                buffer.truncate(n);
                Ok(buffer)
            }
            Err(e) => Err(format!("Failed to receive data: {}", e)),
        }
    }

    pub fn receive_string(&mut self, buffer_size: usize) -> Result<String, String> {
        let bytes = self.receive(buffer_size)?;
        String::from_utf8(bytes).map_err(|e| format!("Invalid UTF-8: {}", e))
    }

    pub fn set_timeout(&mut self, millis: u64) -> Result<(), String> {
        let timeout = Duration::from_millis(millis);
        self.stream
            .set_read_timeout(Some(timeout))
            .map_err(|e| format!("Failed to set read timeout: {}", e))?;
        self.stream
            .set_write_timeout(Some(timeout))
            .map_err(|e| format!("Failed to set write timeout: {}", e))?;
        Ok(())
    }

    pub fn close(&mut self) -> Result<(), String> {
        self.stream
            .shutdown(std::net::Shutdown::Both)
            .map_err(|e| format!("Failed to close connection: {}", e))
    }
}

/// TCP server wrapper
pub struct TcpServer {
    listener: TcpListener,
    address: String,
}

impl TcpServer {
    pub fn bind(addr: &str) -> Result<Self, String> {
        let listener =
            TcpListener::bind(addr).map_err(|e| format!("Failed to bind to {}: {}", addr, e))?;

        Ok(Self {
            listener,
            address: addr.to_string(),
        })
    }

    pub fn accept(&self) -> Result<TcpConnection, String> {
        match self.listener.accept() {
            Ok((stream, addr)) => Ok(TcpConnection::new(stream, addr.to_string())),
            Err(e) => Err(format!("Failed to accept connection: {}", e)),
        }
    }

    pub fn set_nonblocking(&self, nonblocking: bool) -> Result<(), String> {
        self.listener
            .set_nonblocking(nonblocking)
            .map_err(|e| format!("Failed to set nonblocking mode: {}", e))
    }
}

/// UDP socket wrapper
pub struct UdpSocketWrapper {
    socket: UdpSocket,
    address: String,
}

impl UdpSocketWrapper {
    pub fn bind(addr: &str) -> Result<Self, String> {
        let socket = UdpSocket::bind(addr)
            .map_err(|e| format!("Failed to bind UDP socket to {}: {}", addr, e))?;

        Ok(Self {
            socket,
            address: addr.to_string(),
        })
    }

    pub fn send_to(&self, data: &[u8], addr: &str) -> Result<usize, String> {
        self.socket
            .send_to(data, addr)
            .map_err(|e| format!("Failed to send UDP packet to {}: {}", addr, e))
    }

    pub fn receive_from(&self, buffer_size: usize) -> Result<(Vec<u8>, String), String> {
        let mut buffer = vec![0u8; buffer_size];
        match self.socket.recv_from(&mut buffer) {
            Ok((n, addr)) => {
                buffer.truncate(n);
                Ok((buffer, addr.to_string()))
            }
            Err(e) => Err(format!("Failed to receive UDP packet: {}", e)),
        }
    }

    pub fn set_timeout(&self, millis: u64) -> Result<(), String> {
        let timeout = Duration::from_millis(millis);
        self.socket
            .set_read_timeout(Some(timeout))
            .map_err(|e| format!("Failed to set read timeout: {}", e))?;
        self.socket
            .set_write_timeout(Some(timeout))
            .map_err(|e| format!("Failed to set write timeout: {}", e))?;
        Ok(())
    }

    pub fn set_broadcast(&self, broadcast: bool) -> Result<(), String> {
        self.socket
            .set_broadcast(broadcast)
            .map_err(|e| format!("Failed to set broadcast mode: {}", e))
    }

    pub fn connect(&self, addr: &str) -> Result<(), String> {
        self.socket
            .connect(addr)
            .map_err(|e| format!("Failed to connect UDP socket to {}: {}", addr, e))
    }

    pub fn send(&self, data: &[u8]) -> Result<usize, String> {
        self.socket
            .send(data)
            .map_err(|e| format!("Failed to send UDP data: {}", e))
    }

    pub fn receive(&self, buffer_size: usize) -> Result<Vec<u8>, String> {
        let mut buffer = vec![0u8; buffer_size];
        match self.socket.recv(&mut buffer) {
            Ok(n) => {
                buffer.truncate(n);
                Ok(buffer)
            }
            Err(e) => Err(format!("Failed to receive UDP data: {}", e)),
        }
    }
}

/// HTTP request method
#[derive(Debug, Clone, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
}

impl HttpMethod {
    pub fn from_string(s: &str) -> Result<Self, String> {
        match s.to_uppercase().as_str() {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            "PUT" => Ok(HttpMethod::Put),
            "DELETE" => Ok(HttpMethod::Delete),
            "PATCH" => Ok(HttpMethod::Patch),
            "HEAD" => Ok(HttpMethod::Head),
            _ => Err(format!("Unknown HTTP method: {}", s)),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
            HttpMethod::Put => "PUT",
            HttpMethod::Delete => "DELETE",
            HttpMethod::Patch => "PATCH",
            HttpMethod::Head => "HEAD",
        }
    }
}

/// HTTP request
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
    pub timeout: Option<Duration>,
}

impl HttpRequest {
    pub fn new(method: HttpMethod, url: String) -> Self {
        Self {
            method,
            url,
            headers: HashMap::new(),
            body: None,
            timeout: Some(Duration::from_secs(30)),
        }
    }

    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
}

/// HTTP response
#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status_code: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn body_string(&self) -> Result<String, String> {
        String::from_utf8(self.body.clone())
            .map_err(|e| format!("Invalid UTF-8 in response body: {}", e))
    }

    pub fn to_value(&self) -> Value {
        let mut response_obj = HashMap::new();
        response_obj.insert(
            "status".to_string(),
            Value::Integer(self.status_code as i64),
        );
        response_obj.insert(
            "statusText".to_string(),
            Value::String(self.status_text.clone()),
        );

        // Convert headers to Value::Object
        let mut headers_obj = HashMap::new();
        for (k, v) in &self.headers {
            headers_obj.insert(k.clone(), Value::String(v.clone()));
        }
        response_obj.insert("headers".to_string(), Value::Object(headers_obj));

        // Try to convert body to string, fallback to nil
        match self.body_string() {
            Ok(s) => response_obj.insert("body".to_string(), Value::String(s)),
            Err(_) => response_obj.insert("body".to_string(), Value::Nil),
        };

        Value::Object(response_obj)
    }
}

/// Simple HTTP client
pub struct HttpClient {
    default_timeout: Duration,
}

impl HttpClient {
    pub fn new() -> Self {
        Self {
            default_timeout: Duration::from_secs(30),
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.default_timeout = timeout;
        self
    }

    /// Perform HTTP GET request
    pub fn get(&self, url: &str) -> Result<HttpResponse, String> {
        let request = HttpRequest::new(HttpMethod::Get, url.to_string());
        self.execute(request)
    }

    /// Perform HTTP POST request
    pub fn post(&self, url: &str, body: Vec<u8>) -> Result<HttpResponse, String> {
        let request = HttpRequest::new(HttpMethod::Post, url.to_string())
            .with_body(body)
            .with_header("Content-Type".to_string(), "application/json".to_string());
        self.execute(request)
    }

    /// Execute HTTP request (simplified implementation)
    pub fn execute(&self, request: HttpRequest) -> Result<HttpResponse, String> {
        // Parse URL to extract host, port, and path
        let url = &request.url;

        // Simple URL parsing (not production-ready, just for demo)
        let (protocol, rest) = if url.starts_with("http://") {
            ("http", &url[7..])
        } else if url.starts_with("https://") {
            return Err("HTTPS not yet supported in basic implementation".to_string());
        } else {
            ("http", url.as_str())
        };

        let (host_port, path) = match rest.find('/') {
            Some(idx) => (&rest[..idx], &rest[idx..]),
            None => (rest, "/"),
        };

        let (host, port) = match host_port.find(':') {
            Some(idx) => (
                &host_port[..idx],
                host_port[idx + 1..].parse::<u16>().unwrap_or(80),
            ),
            None => (host_port, 80),
        };

        // Build HTTP request string
        let mut http_request = format!(
            "{} {} HTTP/1.1\r\nHost: {}\r\n",
            request.method.as_str(),
            path,
            host
        );

        // Add headers
        for (key, value) in &request.headers {
            http_request.push_str(&format!("{}: {}\r\n", key, value));
        }

        // Add body if present
        if let Some(ref body) = request.body {
            http_request.push_str(&format!("Content-Length: {}\r\n", body.len()));
            http_request.push_str("\r\n");
            http_request.push_str(&String::from_utf8_lossy(body));
        } else {
            http_request.push_str("\r\n");
        }

        // Connect and send request
        let addr = format!("{}:{}", host, port);
        let mut stream = TcpStream::connect(&addr)
            .map_err(|e| format!("Failed to connect to {}: {}", addr, e))?;

        if let Some(timeout) = request.timeout {
            stream
                .set_read_timeout(Some(timeout))
                .map_err(|e| format!("Failed to set timeout: {}", e))?;
        }

        stream
            .write_all(http_request.as_bytes())
            .map_err(|e| format!("Failed to send request: {}", e))?;

        // Read response
        let mut response_data = Vec::new();
        stream
            .read_to_end(&mut response_data)
            .map_err(|e| format!("Failed to read response: {}", e))?;

        // Parse response (simplified)
        self.parse_response(&response_data)
    }

    fn parse_response(&self, data: &[u8]) -> Result<HttpResponse, String> {
        let response_str = String::from_utf8_lossy(data);
        let mut lines = response_str.lines();

        // Parse status line
        let status_line = lines.next().ok_or("Empty response")?;
        let parts: Vec<&str> = status_line.split_whitespace().collect();

        if parts.len() < 3 {
            return Err("Invalid status line".to_string());
        }

        let status_code = parts[1]
            .parse::<u16>()
            .map_err(|e| format!("Invalid status code: {}", e))?;
        let status_text = parts[2..].join(" ");

        // Parse headers
        let mut headers = HashMap::new();
        let mut body_start = 0;

        for (i, line) in lines.enumerate() {
            if line.is_empty() {
                body_start = i + 2; // +2 to skip status line and this empty line
                break;
            }

            if let Some(colon_pos) = line.find(':') {
                let key = line[..colon_pos].trim().to_string();
                let value = line[colon_pos + 1..].trim().to_string();
                headers.insert(key, value);
            }
        }

        // Extract body
        let response_lines: Vec<&str> = response_str.lines().collect();
        let body = if body_start < response_lines.len() {
            response_lines[body_start..].join("\n").into_bytes()
        } else {
            Vec::new()
        };

        Ok(HttpResponse {
            status_code,
            status_text,
            headers,
            body,
        })
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Network utilities
pub struct NetUtils;

impl NetUtils {
    /// Resolve hostname to IP address
    pub fn resolve_host(hostname: &str) -> Result<Vec<String>, String> {
        let addrs = format!("{}:0", hostname)
            .to_socket_addrs()
            .map_err(|e| format!("Failed to resolve {}: {}", hostname, e))?;

        Ok(addrs.map(|addr| addr.ip().to_string()).collect())
    }

    /// Get local IP addresses
    pub fn get_local_ip() -> Result<String, String> {
        // This is a simple approach - connect to a public DNS to determine local IP
        let socket =
            UdpSocket::bind("0.0.0.0:0").map_err(|e| format!("Failed to create socket: {}", e))?;

        socket
            .connect("8.8.8.8:80")
            .map_err(|e| format!("Failed to connect: {}", e))?;

        let addr = socket
            .local_addr()
            .map_err(|e| format!("Failed to get local address: {}", e))?;

        Ok(addr.ip().to_string())
    }

    /// Check if a port is available
    pub fn is_port_available(port: u16) -> bool {
        let addr = format!("127.0.0.1:{}", port);
        TcpListener::bind(addr).is_ok()
    }

    /// Parse URL into components
    pub fn parse_url(url: &str) -> Result<HashMap<String, String>, String> {
        let mut result = HashMap::new();

        let (protocol, rest) = if url.starts_with("http://") {
            ("http", &url[7..])
        } else if url.starts_with("https://") {
            ("https", &url[8..])
        } else {
            ("", url)
        };

        result.insert("protocol".to_string(), protocol.to_string());

        let (host_port, path) = match rest.find('/') {
            Some(idx) => (&rest[..idx], &rest[idx..]),
            None => (rest, "/"),
        };

        let (host, port) = match host_port.find(':') {
            Some(idx) => {
                let port_str = &host_port[idx + 1..];
                let port_num =
                    port_str
                        .parse::<u16>()
                        .unwrap_or(if protocol == "https" { 443 } else { 80 });
                (&host_port[..idx], port_num)
            }
            None => (host_port, if protocol == "https" { 443 } else { 80 }),
        };

        result.insert("host".to_string(), host.to_string());
        result.insert("port".to_string(), port.to_string());
        result.insert("path".to_string(), path.to_string());

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_method_parsing() {
        assert_eq!(HttpMethod::from_string("GET").unwrap(), HttpMethod::Get);
        assert_eq!(HttpMethod::from_string("post").unwrap(), HttpMethod::Post);
        assert_eq!(HttpMethod::from_string("PUT").unwrap(), HttpMethod::Put);
    }

    #[test]
    fn test_http_request_builder() {
        let request = HttpRequest::new(HttpMethod::Get, "http://example.com".to_string())
            .with_header("User-Agent".to_string(), "A-lang/1.0".to_string())
            .with_timeout(Duration::from_secs(10));

        assert_eq!(request.method, HttpMethod::Get);
        assert_eq!(request.url, "http://example.com");
        assert_eq!(request.headers.get("User-Agent").unwrap(), "A-lang/1.0");
    }

    #[test]
    fn test_url_parsing() {
        let result = NetUtils::parse_url("http://example.com:8080/path/to/resource").unwrap();
        assert_eq!(result.get("protocol").unwrap(), "http");
        assert_eq!(result.get("host").unwrap(), "example.com");
        assert_eq!(result.get("port").unwrap(), "8080");
        assert_eq!(result.get("path").unwrap(), "/path/to/resource");
    }

    #[test]
    fn test_port_availability() {
        // Port 0 should always be available (OS assigns)
        assert!(NetUtils::is_port_available(0));
    }

    #[test]
    fn test_socket_type() {
        assert_eq!(SocketType::Tcp, SocketType::Tcp);
        assert_ne!(SocketType::Tcp, SocketType::Udp);
    }
}
