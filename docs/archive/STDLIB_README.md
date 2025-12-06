# A-lang Standard Library - IoT/Networking/Low-Level Features

Welcome to the extended standard library for A-lang! This library provides powerful features for IoT development, network programming, and low-level system operations.

## 📚 Table of Contents

- [Overview](#overview)
- [Modules](#modules)
  - [1. Network Module](#1-network-module)
  - [2. System Utilities](#2-system-utilities)
  - [3. Binary/Bytes Handling](#3-binarybytes-handling)
  - [4. Hardware Simulation](#4-hardware-simulation)
  - [5. FFI (Foreign Function Interface)](#5-ffi-foreign-function-interface)
  - [6. Struct Packing](#6-struct-packing)
- [Quick Start](#quick-start)
- [Examples](#examples)
- [API Reference](#api-reference)
- [Best Practices](#best-practices)

---

## Overview

The A-lang standard library extends the core language with specialized features for:

- **IoT Development**: GPIO, I2C, SPI, UART interfaces (simulated for testing)
- **Network Programming**: TCP/UDP sockets, HTTP client, networking utilities
- **System Operations**: Process execution, environment variables, file system operations
- **Binary Data**: Hex/Base64 encoding, bit manipulation, struct packing
- **C/C++ Interop**: FFI for calling native libraries

### Why Use This Library?

✅ **Development without Hardware**: Test IoT code with simulated GPIO, sensors, and buses  
✅ **Network-Ready**: Built-in HTTP client and socket support  
✅ **C-Compatible**: Struct packing and FFI for C library integration  
✅ **Cross-Platform**: Works on Linux, macOS, and Windows  
✅ **Zero External Dependencies**: Pure Rust implementation for reliability  

---

## Modules

### 1. Network Module

Full-featured networking support for building connected applications.

#### Features

- **HTTP Client**: Simple HTTP GET/POST requests
- **TCP Sockets**: Client and server implementations
- **UDP Sockets**: Datagram communication
- **Network Utilities**: URL parsing, hostname resolution, IP detection

#### Example: HTTP Request

```alang
// Make a GET request
let response = httpGet("http://api.example.com/data");
print("Status: " + response.status);
print("Body: " + response.body);

// Make a POST request
let data = stringifyJSON({"name": "sensor_01", "value": 42});
let postResponse = httpPost("http://api.example.com/data", data);
```

#### Example: URL Parsing

```alang
let urlParts = parseUrl("http://example.com:8080/api/users?id=123");
print("Protocol: " + urlParts.protocol);  // "http"
print("Host: " + urlParts.host);          // "example.com"
print("Port: " + urlParts.port);          // "8080"
print("Path: " + urlParts.path);          // "/api/users?id=123"
```

#### Example: TCP Client

```rust
// Rust-level API (exposed to A-lang)
use a_lang::stdlib::network::TcpConnection;

let mut conn = TcpConnection::connect("127.0.0.1:8080")?;
conn.send(b"Hello, server!")?;
let response = conn.receive_string(1024)?;
```

---

### 2. System Utilities

Interact with the operating system and execute commands.

#### Features

- **Process Execution**: Run shell commands and capture output
- **Environment Variables**: Get/set/remove env vars
- **Path Utilities**: Join, normalize, and manipulate file paths
- **System Information**: OS, architecture, CPU count, timestamps
- **Working Directory**: Get and change current directory

#### Example: Execute Commands

```alang
// Execute a shell command
let result = exec("ls -la");
print("Exit code: " + result.exitCode);
print("Output: " + result.stdout);
print("Success: " + result.success);

// Environment variables
setEnv("MY_VAR", "Hello");
let value = getEnv("MY_VAR");
print(value);  // "Hello"
```

#### Example: System Information

```alang
let sysInfo = getSystemInfo();
print("OS: " + sysInfo.os);
print("Architecture: " + sysInfo.arch);
print("CPUs: " + sysInfo.cpus);
print("Username: " + sysInfo.username);
print("Home: " + sysInfo.home);
```

#### Example: Path Operations

```alang
// Join path components
let fullPath = pathJoin("home", "user", "documents", "file.txt");
// Result: "home/user/documents/file.txt" (Unix) or "home\user\documents\file.txt" (Windows)

// Get basename
let filename = pathBasename("/path/to/file.txt");
print(filename);  // "file.txt"

// Current directory
let cwd = getCwd();
print("Working directory: " + cwd);
```

---

### 3. Binary/Bytes Handling

Low-level binary data manipulation for protocols and file formats.

#### Features

- **Hex Encoding/Decoding**: Convert between bytes and hex strings
- **Base64 Encoding/Decoding**: Standard Base64 operations
- **Bit Operations**: Get, set, clear, toggle individual bits
- **Byte Buffers**: Read/write binary data with endianness control
- **Struct Packing**: Pack/unpack data in C-compatible formats

#### Example: Hex and Base64

```alang
// Hex encoding
let data = [0x48, 0x65, 0x6C, 0x6C, 0x6F];  // "Hello"
let hexString = toHex(data);
print(hexString);  // "48656c6c6f"

let decoded = fromHex(hexString);
print(decoded);  // [72, 101, 108, 108, 111]

// Base64 encoding
let message = "Hello, World!";
let base64 = toBase64(message);
print(base64);  // "SGVsbG8sIFdvcmxkIQ=="

let decodedMsg = fromBase64(base64);
```

#### Example: Bit Operations

```alang
let value = 170;  // 0b10101010

// Check if bit 1 is set
let bit1 = bitGet(value, 1);
print(bit1);  // true

// Set bit 0
let newValue = bitSet(value, 0);
print(newValue);  // 171 (0b10101011)

// Clear bit 1
let cleared = bitClear(newValue, 1);
print(cleared);  // 169 (0b10101001)
```

#### Example: Byte Buffers (Rust API)

```rust
use a_lang::stdlib::bytes::{ByteBuffer, ByteOrder};

let mut buffer = ByteBuffer::new();
buffer.set_byte_order(ByteOrder::LittleEndian);

// Write data
buffer.write_u16(1000);
buffer.write_i32(-42);
buffer.write_f32(3.14);

// Read data
buffer.reset();
let val1 = buffer.read_u16()?;
let val2 = buffer.read_i32()?;
let val3 = buffer.read_f32()?;
```

---

### 4. Hardware Simulation

Simulate IoT hardware interfaces for development and testing without physical devices.

#### Features

- **GPIO**: Digital I/O pins with PWM support
- **I2C**: I2C bus with device simulation
- **SPI**: SPI bus with multiple modes
- **UART**: Serial communication simulation

#### Example: GPIO Control

```alang
// Initialize GPIO pin 13 as output (LED)
gpioInit(13, "output");

// Blink LED
for i in 0..10 {
    gpioWrite(13, 1);  // Turn on
    sleep(500);
    gpioWrite(13, 0);  // Turn off
    sleep(500);
}

// Read input pin
gpioInit(2, "input_pullup");
let state = gpioRead(2);
print("Button state: " + state);
```

#### Example: I2C Communication (Rust API)

```rust
use a_lang::stdlib::hardware::I2cController;

let mut i2c = I2cController::new();

// Add a simulated device at address 0x48
i2c.add_device(0x48)?;

// Write to register 0x01
i2c.write(0x48, &[0x01, 0xFF])?;

// Read from register 0x01
let data = i2c.read(0x48, 0x01, 2)?;
println!("Read: {:?}", data);
```

#### Example: UART Communication (Rust API)

```rust
use a_lang::stdlib::hardware::{UartController, UartConfig};

let mut uart = UartController::new();

// Open UART port
let config = UartConfig::new(9600);  // 9600 baud
uart.open("serial0".to_string(), config)?;

// Send data
uart.write_string("serial0", "Hello UART!\n")?;

// Simulate receiving data
uart.simulate_receive("serial0", b"Response\n")?;

// Read data
let received = uart.read_string("serial0", 100)?;
println!("Received: {}", received);
```

---

### 5. FFI (Foreign Function Interface)

Call C/C++ functions from A-lang for maximum performance and library integration.

#### Features

- **Dynamic Library Loading**: Load .so/.dylib/.dll files at runtime
- **Type Mapping**: Automatic conversion between A-lang and C types
- **Function Signatures**: Type-safe function declarations
- **Cross-Platform**: Works on Unix-like systems (Linux/macOS)

#### Example: Load C Library (Rust API)

```rust
use a_lang::stdlib::ffi::{FFIContext, FFIType};

let mut ffi = FFIContext::new();

// Load libc
ffi.load_library("libc", "/lib/x86_64-linux-gnu/libc.so.6")?;

// Register function signature: int abs(int)
ffi.register_signature("abs", FFIType::Int, vec![FFIType::Int]);

// Call the function
let result = ffi.call_function("libc", "abs", vec![Value::Integer(-42)])?;
// result = Value::Integer(42)
```

#### Supported Types

- `void` - No return value
- `int` / `i32` - 32-bit signed integer
- `long` / `i64` - 64-bit signed integer
- `float` / `f32` - 32-bit floating point
- `double` / `f64` - 64-bit floating point
- `char*` / `string` - C string
- `void*` / `pointer` - Opaque pointer
- `bool` - Boolean value

---

### 6. Struct Packing

Pack and unpack binary data in C-compatible struct layouts.

#### Format Specifiers

- `b` - signed byte (i8)
- `B` - unsigned byte (u8)
- `h` - signed short (i16)
- `H` - unsigned short (u16)
- `i` - signed int (i32)
- `I` - unsigned int (u32)
- `q` - signed long (i64)
- `Q` - unsigned long (u64)
- `f` - float (f32)
- `d` - double (f64)

#### Example: Pack Data (Rust API)

```rust
use a_lang::stdlib::bytes::{StructPacker, ByteOrder};
use a_lang::interpreter::value::Value;

// Pack: unsigned byte, signed short, float
let values = vec![
    Value::Integer(42),
    Value::Integer(-1000),
    Value::Float(3.14),
];

let packed = StructPacker::pack("Bhf", &values, ByteOrder::LittleEndian)?;
// packed = [42, 24, 252, 195, 245, 72, 64]

// Unpack
let unpacked = StructPacker::unpack("Bhf", &packed, ByteOrder::LittleEndian)?;
// unpacked = [Value::Integer(42), Value::Integer(-1000), Value::Float(3.14)]
```

#### Example: Calculate Size

```rust
let size = StructPacker::calcsize("BHIqd");
// size = 1 + 2 + 4 + 8 + 8 = 23 bytes
```

---

## Quick Start

### 1. Basic Usage

```alang
// System info
let info = getSystemInfo();
print("Running on " + info.os + " " + info.arch);

// Execute command
let result = exec("echo Hello");
print(result.stdout);

// Network
let response = httpGet("http://example.com");
print("Status: " + response.status);

// Binary encoding
let encoded = toBase64("Hello, World!");
print(encoded);
```

### 2. IoT Device Example

```alang
// Create a temperature sensor device
fn createTempSensor(pin) {
    return {
        "pin": pin,
        "read": fn() {
            // In real hardware, read from ADC or I2C sensor
            return 22 + (timestamp() % 10);
        },
        "calibrate": fn() {
            print("Calibrating sensor on pin " + this.pin);
        }
    };
}

let sensor = createTempSensor(5);
sensor.calibrate();

// Monitor temperature
for i in 0..5 {
    let temp = sensor.read();
    print("Temperature: " + temp + "°C");
    sleep(1000);
}
```

### 3. Network API Client

```alang
fn apiClient(baseUrl, apiKey) {
    return {
        "get": fn(endpoint) {
            let url = baseUrl + endpoint;
            let response = httpGet(url);
            return parseJSON(response.body);
        },
        
        "post": fn(endpoint, data) {
            let url = baseUrl + endpoint;
            let body = stringifyJSON(data);
            return httpPost(url, body);
        }
    };
}

let api = apiClient("https://api.example.com", "secret");
let data = api.get("/sensors/temp01");
print("Current temp: " + data.value);
```

---

## Examples

See the complete examples in:
- `examples/stdlib_demo.al` - Comprehensive feature demonstration
- `examples/iot_device.al` - IoT device simulation (coming soon)
- `examples/network_client.al` - Network programming examples (coming soon)

---

## API Reference

### Network Functions (A-lang)

```alang
httpGet(url: string) -> {status, statusText, headers, body}
httpPost(url: string, body: string) -> {status, statusText, headers, body}
parseUrl(url: string) -> {protocol, host, port, path}
getLocalIp() -> string
isPortAvailable(port: int) -> bool
```

### System Functions (A-lang)

```alang
exec(command: string) -> {exitCode, stdout, stderr, success}
getEnv(key: string) -> string | nil
setEnv(key: string, value: string) -> nil
getSystemInfo() -> {os, arch, cpus, username, home, cwd, ...}
getCwd() -> string
setCwd(path: string) -> nil
timestamp() -> int
sleep(millis: int) -> nil
pathJoin(...components: string) -> string
pathBasename(path: string) -> string
```

### Binary Functions (A-lang)

```alang
toHex(bytes: array) -> string
fromHex(hex: string) -> array
toBase64(data: string | array) -> string
fromBase64(base64: string) -> array
bitGet(value: int, bit: int) -> bool
bitSet(value: int, bit: int) -> int
bitClear(value: int, bit: int) -> int
bitToggle(value: int, bit: int) -> int
```

### Hardware Functions (A-lang)

```alang
gpioInit(pin: int, mode: string) -> nil
gpioWrite(pin: int, state: int) -> nil
gpioRead(pin: int) -> int
gpioPwm(pin: int, duty: float) -> nil
```

---

## Best Practices

### 1. Error Handling

Always check for errors when working with I/O operations:

```alang
let result = exec("some-command");
if result.success {
    print("Success: " + result.stdout);
} else {
    print("Error: " + result.stderr);
    print("Exit code: " + result.exitCode);
}
```

### 2. Resource Cleanup

Clean up resources when done:

```rust
// In Rust code
let mut conn = TcpConnection::connect("127.0.0.1:8080")?;
// ... use connection ...
conn.close()?;  // Always close
```

### 3. Testing Without Hardware

Use the simulated hardware interfaces for testing:

```alang
// This code works identically on real hardware and in simulation
gpioInit(13, "output");
gpioWrite(13, 1);
sleep(1000);
gpioWrite(13, 0);
```

### 4. Binary Protocol Design

Always include validation in binary protocols:

```alang
fn encodePacket(data) {
    let packet = [0xFF];  // START marker
    push(packet, len(data));
    for byte in data { push(packet, byte); }
    push(packet, calculateChecksum(data));
    push(packet, 0xFE);  // END marker
    return packet;
}
```

### 5. Network Timeouts

Set reasonable timeouts for network operations:

```rust
conn.set_timeout(5000)?;  // 5 second timeout
```

---

## Performance Considerations

### Network
- **HTTP**: Simple blocking implementation, suitable for IoT
- **TCP/UDP**: Low-level control for performance-critical applications

### Binary Operations
- **Zero-copy where possible**: ByteBuffer uses efficient Vec operations
- **Endianness**: Configure once, use throughout

### Hardware Simulation
- **Minimal overhead**: Simulated hardware has near-zero latency
- **Thread-safe**: All hardware controllers use Arc<Mutex<>> for safety

---

## Platform Support

| Feature | Linux | macOS | Windows |
|---------|-------|-------|---------|
| Network (HTTP/TCP/UDP) | ✅ | ✅ | ✅ |
| System Utilities | ✅ | ✅ | ✅ |
| Binary/Bytes | ✅ | ✅ | ✅ |
| Hardware Simulation | ✅ | ✅ | ✅ |
| FFI | ✅ | ✅ | ⚠️ Limited |

**Note**: FFI requires platform-specific dynamic library formats (.so on Linux, .dylib on macOS, .dll on Windows).

---

## Future Enhancements

- [ ] HTTPS support (TLS/SSL)
- [ ] WebSocket client
- [ ] MQTT protocol
- [ ] Bluetooth Low Energy (BLE)
- [ ] Real GPIO support via sysfs/gpiod
- [ ] Direct I2C/SPI device access
- [ ] More FFI type mappings (structs, callbacks)
- [ ] Async network operations

---

## Contributing

We welcome contributions! Areas of interest:

1. Additional protocol implementations (MQTT, CoAP, etc.)
2. Real hardware backends for GPIO/I2C/SPI
3. More comprehensive FFI support
4. Performance optimizations
5. Documentation and examples

---

## License

This library is part of A-lang and is released under the MIT License.

---

## Support

- **Documentation**: See the main A-lang README
- **Examples**: Check the `examples/` directory
- **Issues**: Report bugs on GitHub
- **Community**: Join our Discord/forum (coming soon)

---

**Happy coding with A-lang! 🚀**