//! Standard library modules for A-lang
//! Provides extended functionality for IoT, networking, and low-level operations

// Module declarations
pub mod bytes;
pub mod database;
pub mod ffi;
pub mod hardware;
pub mod http_server;
pub mod integration;
pub mod network;
pub mod system;
pub mod websocket;

// Re-export commonly used types for convenience
pub use bytes::{BinaryEncoder, BitOps, ByteBuffer, ByteOrder, StructPacker};
pub use database::{Database, DatabaseConfig, QueryBuilder, QueryResult, Transaction};
pub use ffi::{FFIContext, FFISignature, FFIType};
pub use hardware::{
    GpioController, HardwareManager, I2cController, PinMode, PinState, SpiController, SpiMode,
    UartConfig, UartController, UartParity,
};
pub use http_server::{
    ExpressApp, HttpRequest as ServerHttpRequest, HttpResponseBuilder, HttpServer, ServerConfig,
};
pub use integration::{register_stdlib_functions, StdlibContext};
pub use network::{
    HttpClient, HttpMethod, HttpRequest, HttpResponse, NetUtils, SocketType, TcpConnection,
    TcpServer, UdpSocketWrapper,
};
pub use system::{PathUtils, ProcessResult, SystemUtils, Timer};
pub use websocket::{
    ConnectionState, MessageType, WebSocketClient, WebSocketMessage, WebSocketServer,
    WebSocketServerConfig, WebSocketUtils,
};

/// Prelude module for common imports
pub mod prelude {
    pub use crate::stdlib::bytes::{BinaryEncoder, ByteBuffer, ByteOrder};
    pub use crate::stdlib::database::{Database, DatabaseConfig, QueryBuilder};
    pub use crate::stdlib::ffi::{FFIContext, FFIType};
    pub use crate::stdlib::hardware::{GpioController, HardwareManager, PinMode, PinState};
    pub use crate::stdlib::http_server::{ExpressApp, HttpServer, ServerConfig};
    pub use crate::stdlib::network::{HttpClient, NetUtils, TcpConnection};
    pub use crate::stdlib::system::{PathUtils, SystemUtils};
    pub use crate::stdlib::websocket::{WebSocketClient, WebSocketServer};
}

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const STDLIB_VERSION: &str = "1.0.0";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_info() {
        assert!(!VERSION.is_empty());
        assert!(!STDLIB_VERSION.is_empty());
    }

    #[test]
    fn test_byte_order_creation() {
        let _buffer = ByteBuffer::new();
        assert!(true);
    }

    #[test]
    fn test_gpio_creation() {
        let _gpio = GpioController::new();
        assert!(true);
    }

    #[test]
    fn test_http_client_creation() {
        let _client = HttpClient::new();
        assert!(true);
    }

    #[test]
    fn test_server_config_creation() {
        let config = ServerConfig::new("127.0.0.1".to_string(), 3000);
        assert_eq!(config.port, 3000);
    }

    #[test]
    fn test_database_config_creation() {
        let config = DatabaseConfig::default();
        assert_eq!(config.port, 3306);
    }

    #[test]
    fn test_websocket_message_creation() {
        let msg = WebSocketMessage::text("Hello".to_string());
        assert_eq!(msg.as_text().unwrap(), "Hello");
    }

    #[test]
    fn test_system_utils() {
        let os = SystemUtils::get_os();
        assert!(!os.is_empty());
    }

    #[test]
    fn test_hardware_manager() {
        let _manager = HardwareManager::new();
        assert!(true);
    }
}
