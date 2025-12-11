//! Integration tests for A-lang standard library
//! Tests IoT/networking/low-level features

use a_lang::stdlib::{
    bytes::{BinaryEncoder, BitOps, ByteBuffer, ByteOrder, StructPacker},
    hardware::{
        GpioController, HardwareManager, I2cController, PinMode, PinState, SpiController, SpiMode,
        UartConfig, UartController,
    },
    network::{HttpClient, HttpMethod, HttpRequest, NetUtils},
    system::{PathUtils, SystemUtils},
};
use a_lang::Value;

// ============================================================================
// BINARY/BYTES TESTS
// ============================================================================

#[test]
fn test_hex_encoding() {
    let data = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]; // "Hello"
    let hex = BinaryEncoder::to_hex(&data);
    assert_eq!(hex, "48656c6c6f");

    let decoded = BinaryEncoder::from_hex(&hex).unwrap();
    assert_eq!(decoded, data);
}

#[test]
fn test_base64_encoding() {
    let data = b"Hello, World!";
    let encoded = BinaryEncoder::to_base64(data);
    let decoded = BinaryEncoder::from_base64(&encoded).unwrap();
    assert_eq!(decoded, data);
}

#[test]
fn test_bit_operations() {
    let value = 0b10101010u64;

    // Test bit get
    assert!(BitOps::get_bit(value, 1));
    assert!(!BitOps::get_bit(value, 0));

    // Test bit set
    let set_value = BitOps::set_bit(value, 0);
    assert_eq!(set_value, 0b10101011);

    // Test bit clear
    let cleared = BitOps::clear_bit(value, 1);
    assert_eq!(cleared, 0b10101000);

    // Test bit toggle
    let toggled = BitOps::toggle_bit(value, 0);
    assert_eq!(toggled, 0b10101011);

    // Test count operations
    assert_eq!(BitOps::count_ones(0xFF), 8);
    assert_eq!(BitOps::count_zeros(0xFF), 56);
}

#[test]
fn test_byte_buffer_operations() {
    let mut buffer = ByteBuffer::new();
    buffer.set_byte_order(ByteOrder::LittleEndian);

    // Write various types
    buffer.write_u8(42);
    buffer.write_i16(-1000);
    buffer.write_u32(123456);
    buffer.write_f32(3.14);

    // Reset position and read back
    buffer.reset();
    assert_eq!(buffer.read_u8().unwrap(), 42);
    assert_eq!(buffer.read_i16().unwrap(), -1000);
    assert_eq!(buffer.read_u32().unwrap(), 123456);
    assert!((buffer.read_f32().unwrap() - 3.14).abs() < 0.001);
}

#[test]
fn test_byte_buffer_endianness() {
    let mut buffer = ByteBuffer::new();

    // Test little endian
    buffer.set_byte_order(ByteOrder::LittleEndian);
    buffer.write_u16(0x1234);
    assert_eq!(buffer.as_bytes(), &[0x34, 0x12]);

    // Test big endian
    let mut buffer = ByteBuffer::new();
    buffer.set_byte_order(ByteOrder::BigEndian);
    buffer.write_u16(0x1234);
    assert_eq!(buffer.as_bytes(), &[0x12, 0x34]);
}

#[test]
fn test_struct_packing() {
    let values = vec![
        Value::Integer(42),
        Value::Integer(-1000),
        Value::Float(3.14),
    ];

    // Pack data
    let packed = StructPacker::pack("Bhd", &values, ByteOrder::LittleEndian).unwrap();
    assert_eq!(packed.len(), 1 + 2 + 8); // u8 + i16 + f64

    // Unpack data
    let unpacked = StructPacker::unpack("Bhd", &packed, ByteOrder::LittleEndian).unwrap();
    assert_eq!(unpacked.len(), 3);

    match &unpacked[0] {
        Value::Integer(v) => assert_eq!(*v, 42),
        _ => panic!("Expected integer"),
    }

    match &unpacked[1] {
        Value::Integer(v) => assert_eq!(*v, -1000),
        _ => panic!("Expected integer"),
    }

    match &unpacked[2] {
        Value::Float(v) => assert!((*v - 3.14).abs() < 0.001),
        _ => panic!("Expected float"),
    }
}

#[test]
fn test_struct_calcsize() {
    assert_eq!(StructPacker::calcsize("B"), 1);
    assert_eq!(StructPacker::calcsize("BH"), 3);
    assert_eq!(StructPacker::calcsize("BHI"), 7);
    assert_eq!(StructPacker::calcsize("BHIq"), 15);
    assert_eq!(StructPacker::calcsize("BHIqd"), 23);
}

// ============================================================================
// SYSTEM UTILITIES TESTS
// ============================================================================

#[test]
fn test_system_exec() {
    let result = SystemUtils::exec("echo hello").unwrap();
    assert!(result.success);
    assert!(result.stdout.contains("hello"));
    assert_eq!(result.exit_code, 0);
}

#[test]
fn test_environment_variables() {
    let key = "ALANG_TEST_VAR";
    let value = "test_value_12345";

    SystemUtils::set_env(key, value);
    assert_eq!(SystemUtils::get_env(key), Some(value.to_string()));

    SystemUtils::remove_env(key);
    assert_eq!(SystemUtils::get_env(key), None);
}

#[test]
fn test_system_info() {
    let info = SystemUtils::get_system_info();
    assert!(info.contains_key("os"));
    assert!(info.contains_key("arch"));
    assert!(info.contains_key("cpus"));
    assert!(info.contains_key("pid"));
}

#[test]
fn test_working_directory() {
    let cwd = SystemUtils::get_cwd().unwrap();
    assert!(!cwd.is_empty());
}

#[test]
fn test_timestamps() {
    let ts1 = SystemUtils::get_timestamp();
    std::thread::sleep(std::time::Duration::from_millis(10));
    let ts2 = SystemUtils::get_timestamp();
    assert!(ts2 >= ts1);
}

#[test]
fn test_path_operations() {
    let joined = PathUtils::join(&[
        "home".to_string(),
        "user".to_string(),
        "file.txt".to_string(),
    ]);
    assert!(joined.contains("home"));
    assert!(joined.contains("file.txt"));

    let basename = PathUtils::basename("/path/to/file.txt");
    assert_eq!(basename, Some("file.txt".to_string()));

    let dirname = PathUtils::dirname("/path/to/file.txt");
    assert!(dirname.is_some());

    let ext = PathUtils::extname("file.txt");
    assert_eq!(ext, Some(".txt".to_string()));
}

// ============================================================================
// NETWORK TESTS
// ============================================================================

#[test]
fn test_url_parsing() {
    let parts = NetUtils::parse_url("http://example.com:8080/api/users").unwrap();
    assert_eq!(parts.get("protocol").unwrap(), "http");
    assert_eq!(parts.get("host").unwrap(), "example.com");
    assert_eq!(parts.get("port").unwrap(), "8080");
    assert_eq!(parts.get("path").unwrap(), "/api/users");
}

#[test]
fn test_url_parsing_default_port() {
    let parts = NetUtils::parse_url("http://example.com/path").unwrap();
    assert_eq!(parts.get("protocol").unwrap(), "http");
    assert_eq!(parts.get("host").unwrap(), "example.com");
    assert_eq!(parts.get("port").unwrap(), "80");
}

#[test]
fn test_port_availability() {
    // Port 0 should always be available (OS assigns)
    assert!(NetUtils::is_port_available(0));
}

#[test]
fn test_http_method_parsing() {
    assert_eq!(HttpMethod::from_string("GET").unwrap(), HttpMethod::Get);
    assert_eq!(HttpMethod::from_string("post").unwrap(), HttpMethod::Post);
    assert_eq!(HttpMethod::from_string("PUT").unwrap(), HttpMethod::Put);
    assert_eq!(
        HttpMethod::from_string("DELETE").unwrap(),
        HttpMethod::Delete
    );
}

#[test]
fn test_http_request_builder() {
    let request = HttpRequest::new(HttpMethod::Get, "http://example.com".to_string())
        .with_header("User-Agent".to_string(), "A-lang/1.0".to_string())
        .with_timeout(std::time::Duration::from_secs(10));

    assert_eq!(request.method, HttpMethod::Get);
    assert_eq!(request.url, "http://example.com");
    assert_eq!(request.headers.get("User-Agent").unwrap(), "A-lang/1.0");
}

#[test]
fn test_http_client_creation() {
    let _client = HttpClient::new();
    // Just verify it can be created without panic
}

// ============================================================================
// HARDWARE SIMULATION TESTS
// ============================================================================

#[test]
fn test_gpio_basic_operations() {
    let mut gpio = GpioController::new();

    // Initialize pin
    gpio.init_pin(13, PinMode::Output).unwrap();

    // Write and read
    gpio.write(13, PinState::High).unwrap();
    assert_eq!(gpio.read(13).unwrap(), PinState::High);

    // Toggle
    gpio.toggle(13).unwrap();
    assert_eq!(gpio.read(13).unwrap(), PinState::Low);
}

#[test]
fn test_gpio_pwm() {
    let mut gpio = GpioController::new();
    gpio.init_pin(9, PinMode::Output).unwrap();

    // Set PWM duty cycle
    assert!(gpio.set_pwm(9, 0.5).is_ok());
    assert!(gpio.set_pwm(9, 1.0).is_ok());

    // Invalid duty cycle should fail
    assert!(gpio.set_pwm(9, 1.5).is_err());
    assert!(gpio.set_pwm(9, -0.1).is_err());
}

#[test]
fn test_gpio_pin_modes() {
    let mut gpio = GpioController::new();

    // Test different pin modes
    gpio.init_pin(1, PinMode::Input).unwrap();
    gpio.init_pin(2, PinMode::Output).unwrap();
    gpio.init_pin(3, PinMode::InputPullUp).unwrap();
    gpio.init_pin(4, PinMode::InputPullDown).unwrap();

    // Can't write to input pin
    assert!(gpio.write(1, PinState::High).is_err());

    // Can write to output pin
    assert!(gpio.write(2, PinState::High).is_ok());
}

#[test]
fn test_i2c_operations() {
    let mut i2c = I2cController::new();

    // Add device
    i2c.add_device(0x48).unwrap();

    // Write to register
    i2c.write(0x48, &[0x01, 0xFF]).unwrap();

    // Read from register
    let data = i2c.read(0x48, 0x01, 1).unwrap();
    assert_eq!(data[0], 0xFF);
}

#[test]
fn test_i2c_scan() {
    let mut i2c = I2cController::new();

    // Add multiple devices
    i2c.add_device(0x48).unwrap();
    i2c.add_device(0x50).unwrap();
    i2c.add_device(0x68).unwrap();

    // Scan for devices
    let devices = i2c.scan();
    assert_eq!(devices.len(), 3);
    assert!(devices.contains(&0x48));
    assert!(devices.contains(&0x50));
    assert!(devices.contains(&0x68));
}

#[test]
fn test_spi_operations() {
    let mut spi = SpiController::new();

    // Add device
    spi.add_device("device0".to_string(), SpiMode::Mode0)
        .unwrap();

    // Transfer data
    let tx_data = vec![0x01, 0x02, 0x03];
    let rx_data = spi.transfer("device0", &tx_data).unwrap();
    assert_eq!(rx_data.len(), tx_data.len());
}

#[test]
fn test_uart_operations() {
    let mut uart = UartController::new();

    // Open port
    let config = UartConfig::new(9600);
    uart.open("serial0".to_string(), config).unwrap();

    // Write data
    let written = uart.write_string("serial0", "Hello UART").unwrap();
    assert_eq!(written, 10);

    // Simulate receiving data
    uart.simulate_receive("serial0", b"Response").unwrap();

    // Read data
    let available = uart.available("serial0").unwrap();
    assert_eq!(available, 8);

    let received = uart.read_string("serial0", 8).unwrap();
    assert_eq!(received, "Response");
}

#[test]
fn test_hardware_manager() {
    let manager = HardwareManager::new();

    // Test GPIO access
    {
        let mut gpio = manager.gpio.lock().unwrap();
        gpio.init_pin(1, PinMode::Output).unwrap();
        gpio.write(1, PinState::High).unwrap();
    }

    // Test I2C access
    {
        let mut i2c = manager.i2c.lock().unwrap();
        i2c.add_device(0x20).unwrap();
    }

    // Test SPI access
    {
        let mut spi = manager.spi.lock().unwrap();
        spi.add_device("spi0".to_string(), SpiMode::Mode0).unwrap();
    }

    // Test UART access
    {
        let mut uart = manager.uart.lock().unwrap();
        uart.open("uart0".to_string(), UartConfig::new(115200))
            .unwrap();
    }

    // Reset everything
    manager.reset();

    // Verify reset
    {
        let gpio = manager.gpio.lock().unwrap();
        assert_eq!(gpio.list_pins().len(), 0);
    }
}

// ============================================================================
// INTEGRATION TESTS
// ============================================================================

#[test]
fn test_complete_iot_workflow() {
    // Simulate a complete IoT device workflow
    let manager = HardwareManager::new();

    // 1. Initialize GPIO for LED
    {
        let mut gpio = manager.gpio.lock().unwrap();
        gpio.init_pin(13, PinMode::Output).unwrap();
        gpio.write(13, PinState::High).unwrap();
    }

    // 2. Initialize I2C for temperature sensor
    {
        let mut i2c = manager.i2c.lock().unwrap();
        i2c.add_device(0x48).unwrap();
        i2c.write(0x48, &[0x00, 0x60]).unwrap(); // Config register
    }

    // 3. Read temperature
    {
        let i2c = manager.i2c.lock().unwrap();
        let temp_data = i2c.read(0x48, 0x00, 2).unwrap();
        assert_eq!(temp_data.len(), 2);
    }

    // 4. Turn off LED
    {
        let mut gpio = manager.gpio.lock().unwrap();
        gpio.write(13, PinState::Low).unwrap();
    }
}

#[test]
fn test_binary_protocol_encoding() {
    // Simulate encoding sensor data for transmission
    let mut buffer = ByteBuffer::new();
    buffer.set_byte_order(ByteOrder::LittleEndian);

    // Protocol: [START][DEVICE_ID][SENSOR_TYPE][VALUE][CHECKSUM]
    buffer.write_u8(0xFF); // START
    buffer.write_u32(12345); // Device ID
    buffer.write_u8(1); // Sensor type: temperature
    buffer.write_f32(23.5); // Temperature value

    // Calculate checksum (simple sum with wrapping)
    let data = buffer.as_bytes();
    let checksum: u8 = data.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
    buffer.write_u8(checksum);

    // Verify packet
    let packet = buffer.to_vec();
    assert!(packet.len() > 0);
    assert_eq!(packet[0], 0xFF); // Verify START byte
}

#[test]
fn test_system_info_collection() {
    // Collect comprehensive system information
    let info = SystemUtils::get_system_info();

    // Verify essential fields exist
    assert!(info.contains_key("os"));
    assert!(info.contains_key("arch"));
    assert!(info.contains_key("cpus"));

    let os = &info["os"];
    let arch = &info["arch"];

    println!("System: {} {}", os, arch);

    // Get additional system details
    let cwd = SystemUtils::get_cwd().unwrap();
    let timestamp = SystemUtils::get_timestamp();

    assert!(!cwd.is_empty());
    assert!(timestamp > 0);
}
