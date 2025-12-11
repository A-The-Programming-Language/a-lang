//! WebSocket module for A-lang
//! Provides WebSocket client and server functionality

use crate::interpreter::value::Value;
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::runtime::Runtime;
use tokio::sync::mpsc;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message as WsMessage, MaybeTlsStream, WebSocketStream,
};

/// WebSocket message type
#[derive(Debug, Clone)]
pub enum MessageType {
    Text,
    Binary,
    Ping,
    Pong,
    Close,
}

/// WebSocket message
#[derive(Debug, Clone)]
pub struct WebSocketMessage {
    pub msg_type: MessageType,
    pub data: Vec<u8>,
}

impl WebSocketMessage {
    pub fn text(text: String) -> Self {
        Self {
            msg_type: MessageType::Text,
            data: text.into_bytes(),
        }
    }

    pub fn binary(data: Vec<u8>) -> Self {
        Self {
            msg_type: MessageType::Binary,
            data,
        }
    }

    pub fn as_text(&self) -> Result<String, String> {
        String::from_utf8(self.data.clone()).map_err(|e| format!("Invalid UTF-8: {}", e))
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    pub fn to_value(&self) -> Value {
        let mut msg = HashMap::new();

        let type_str = match self.msg_type {
            MessageType::Text => "text",
            MessageType::Binary => "binary",
            MessageType::Ping => "ping",
            MessageType::Pong => "pong",
            MessageType::Close => "close",
        };

        msg.insert("type".to_string(), Value::String(type_str.to_string()));

        match self.msg_type {
            MessageType::Text => {
                if let Ok(text) = self.as_text() {
                    msg.insert("data".to_string(), Value::String(text));
                }
            }
            MessageType::Binary => {
                let bytes: Vec<Value> = self
                    .data
                    .iter()
                    .map(|&b| Value::Integer(b as i64))
                    .collect();
                msg.insert("data".to_string(), Value::Array(bytes));
            }
            _ => {}
        }

        Value::Object(msg)
    }
}

/// WebSocket connection state
#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    Connecting,
    Open,
    Closing,
    Closed,
}

/// WebSocket client
pub struct WebSocketClient {
    url: String,
    state: Arc<Mutex<ConnectionState>>,
    tx: Option<mpsc::UnboundedSender<WsMessage>>,
    runtime: Option<Runtime>,
}

impl WebSocketClient {
    pub fn new(url: String) -> Self {
        Self {
            url,
            state: Arc::new(Mutex::new(ConnectionState::Closed)),
            tx: None,
            runtime: None,
        }
    }

    /// Connect to WebSocket server (simplified implementation)
    pub fn connect(&mut self) -> Result<(), String> {
        *self.state.lock().unwrap() = ConnectionState::Connecting;

        let (tx, _rx) = mpsc::unbounded_channel::<WsMessage>();
        self.tx = Some(tx);

        // Simplified: mark as connected (full async implementation requires more setup)
        *self.state.lock().unwrap() = ConnectionState::Open;
        println!("✓ WebSocket client created: {}", self.url);

        Ok(())
    }

    /// Send text message
    pub fn send_text(&self, text: &str) -> Result<(), String> {
        if let Some(tx) = &self.tx {
            tx.send(WsMessage::Text(text.to_string()))
                .map_err(|e| format!("Failed to send: {}", e))?;
            Ok(())
        } else {
            Err("Not connected".to_string())
        }
    }

    /// Send binary message
    pub fn send_binary(&self, data: Vec<u8>) -> Result<(), String> {
        if let Some(tx) = &self.tx {
            tx.send(WsMessage::Binary(data))
                .map_err(|e| format!("Failed to send: {}", e))?;
            Ok(())
        } else {
            Err("Not connected".to_string())
        }
    }

    /// Close connection
    pub fn close(&mut self) -> Result<(), String> {
        *self.state.lock().unwrap() = ConnectionState::Closing;

        if let Some(tx) = &self.tx {
            tx.send(WsMessage::Close(None))
                .map_err(|e| format!("Failed to close: {}", e))?;
        }

        if let Some(runtime) = self.runtime.take() {
            runtime.shutdown_background();
        }

        *self.state.lock().unwrap() = ConnectionState::Closed;
        Ok(())
    }

    /// Get connection state
    pub fn state(&self) -> ConnectionState {
        self.state.lock().unwrap().clone()
    }

    /// Check if connected
    pub fn is_connected(&self) -> bool {
        *self.state.lock().unwrap() == ConnectionState::Open
    }
}

impl Drop for WebSocketClient {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

/// WebSocket server configuration
#[derive(Debug, Clone)]
pub struct WebSocketServerConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: usize,
}

impl WebSocketServerConfig {
    pub fn new(host: String, port: u16) -> Self {
        Self {
            host,
            port,
            max_connections: 100,
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Default for WebSocketServerConfig {
    fn default() -> Self {
        Self::new("127.0.0.1".to_string(), 8080)
    }
}

/// WebSocket connection handler
pub type ConnectionHandler =
    Arc<dyn Fn(String, WebSocketMessage) -> Result<(), String> + Send + Sync>;

/// WebSocket server
pub struct WebSocketServer {
    config: WebSocketServerConfig,
    connections: Arc<Mutex<HashMap<String, mpsc::UnboundedSender<WsMessage>>>>,
    on_message: Option<ConnectionHandler>,
    runtime: Option<Runtime>,
}

impl WebSocketServer {
    pub fn new(config: WebSocketServerConfig) -> Self {
        Self {
            config,
            connections: Arc::new(Mutex::new(HashMap::new())),
            on_message: None,
            runtime: None,
        }
    }

    /// Set message handler
    pub fn on_message(&mut self, handler: ConnectionHandler) {
        self.on_message = Some(handler);
    }

    /// Send message to specific client
    pub fn send_to(&self, client_id: &str, message: WebSocketMessage) -> Result<(), String> {
        let connections = self.connections.lock().unwrap();

        if let Some(tx) = connections.get(client_id) {
            let ws_msg = match message.msg_type {
                MessageType::Text => {
                    WsMessage::Text(String::from_utf8_lossy(&message.data).to_string())
                }
                MessageType::Binary => WsMessage::Binary(message.data),
                _ => return Err("Unsupported message type".to_string()),
            };

            tx.send(ws_msg)
                .map_err(|e| format!("Failed to send: {}", e))?;
            Ok(())
        } else {
            Err(format!("Client {} not found", client_id))
        }
    }

    /// Broadcast message to all clients
    pub fn broadcast(&self, message: WebSocketMessage) -> Result<(), String> {
        let connections = self.connections.lock().unwrap();

        for (client_id, tx) in connections.iter() {
            let ws_msg = match message.msg_type {
                MessageType::Text => {
                    WsMessage::Text(String::from_utf8_lossy(&message.data).to_string())
                }
                MessageType::Binary => WsMessage::Binary(message.data.clone()),
                _ => continue,
            };

            if let Err(e) = tx.send(ws_msg) {
                eprintln!("Failed to send to {}: {}", client_id, e);
            }
        }

        Ok(())
    }

    /// Get list of connected client IDs
    pub fn get_clients(&self) -> Vec<String> {
        self.connections.lock().unwrap().keys().cloned().collect()
    }

    /// Get number of connected clients
    pub fn client_count(&self) -> usize {
        self.connections.lock().unwrap().len()
    }

    /// Start the server (placeholder - requires full async implementation)
    pub fn listen(&mut self) -> Result<(), String> {
        println!(
            "🔌 WebSocket server starting on ws://{}",
            self.config.address()
        );
        println!("⚠️  Full WebSocket server requires async runtime integration");
        Ok(())
    }

    /// Stop the server
    pub fn stop(&mut self) {
        self.connections.lock().unwrap().clear();

        if let Some(runtime) = self.runtime.take() {
            runtime.shutdown_background();
        }
    }
}

impl Drop for WebSocketServer {
    fn drop(&mut self) {
        self.stop();
    }
}

/// WebSocket utilities
pub struct WebSocketUtils;

impl WebSocketUtils {
    /// Create WebSocket URL from HTTP URL
    pub fn http_to_ws(url: &str) -> String {
        url.replace("http://", "ws://")
            .replace("https://", "wss://")
    }

    /// Validate WebSocket URL
    pub fn is_valid_url(url: &str) -> bool {
        url.starts_with("ws://") || url.starts_with("wss://")
    }

    /// Parse WebSocket frame (simplified)
    pub fn parse_frame(data: &[u8]) -> Result<WebSocketMessage, String> {
        if data.is_empty() {
            return Err("Empty frame".to_string());
        }

        // Simplified parsing
        Ok(WebSocketMessage::binary(data.to_vec()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_websocket_message_text() {
        let msg = WebSocketMessage::text("Hello".to_string());
        assert_eq!(msg.as_text().unwrap(), "Hello");
    }

    #[test]
    fn test_websocket_message_binary() {
        let data = vec![1, 2, 3, 4, 5];
        let msg = WebSocketMessage::binary(data.clone());
        assert_eq!(msg.as_bytes(), &data[..]);
    }

    #[test]
    fn test_connection_state() {
        let client = WebSocketClient::new("ws://example.com".to_string());
        assert_eq!(client.state(), ConnectionState::Closed);
    }

    #[test]
    fn test_server_config() {
        let config = WebSocketServerConfig::new("localhost".to_string(), 9000);
        assert_eq!(config.address(), "localhost:9000");
    }

    #[test]
    fn test_websocket_utils() {
        assert_eq!(
            WebSocketUtils::http_to_ws("http://example.com"),
            "ws://example.com"
        );
        assert_eq!(
            WebSocketUtils::http_to_ws("https://example.com"),
            "wss://example.com"
        );

        assert!(WebSocketUtils::is_valid_url("ws://example.com"));
        assert!(WebSocketUtils::is_valid_url("wss://example.com"));
        assert!(!WebSocketUtils::is_valid_url("http://example.com"));
    }

    #[test]
    fn test_message_to_value() {
        let msg = WebSocketMessage::text("Hello".to_string());
        let value = msg.to_value();

        match value {
            Value::Object(obj) => {
                assert_eq!(obj.get("type").unwrap(), &Value::String("text".to_string()));
                assert_eq!(
                    obj.get("data").unwrap(),
                    &Value::String("Hello".to_string())
                );
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_server_client_count() {
        let config = WebSocketServerConfig::default();
        let server = WebSocketServer::new(config);
        assert_eq!(server.client_count(), 0);
    }

    #[test]
    fn test_client_creation() {
        let client = WebSocketClient::new("ws://localhost:8080".to_string());
        assert!(!client.is_connected());
    }
}
