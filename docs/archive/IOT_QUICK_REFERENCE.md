# A-lang IoT/Networking Quick Reference

Quick reference for IoT, networking, and low-level features in A-lang.

---

## 📡 Network Functions

### HTTP Client

```alang
// GET request
let response = httpGet("http://api.example.com/data");
print(response.status);      // Status code (e.g., 200)
print(response.body);         // Response body

// POST request
let data = stringifyJSON({"sensor": "temp01", "value": 23.5});
let response = httpPost("http://api.example.com/data", data);
```

### URL Utilities

```alang
// Parse URL
let parts = parseUrl("http://example.com:8080/api/users");
print(parts.protocol);  // "http"
print(parts.host);      // "example.com"
print(parts.port);      // "8080"
print(parts.path);      // "/api/users"

// Network info
let ip = getLocalIp();
let available = isPortAvailable(8080);
```

---

## 🖥️ System Utilities

### Command Execution

```alang
// Execute command
let result = exec("ls -la");
print(result.exitCode);  // 0 for success
print(result.stdout);    // Standard output
print(result.stderr);    // Standard error
print(result.success);   // true/false
```

### Environment Variables

```alang
// Set and get
setEnv("API_KEY", "secret123");
let key = getEnv("API_KEY");        // "secret123"
let missing = getEnv("NOT_SET");    // nil

// System info
let info = getSystemInfo();
print(info.os);          // "linux", "macos", "windows"
print(info.arch);        // "x86_64", "aarch64", etc.
print(info.cpus);        // Number of CPUs
print(info.username);    // Current user
```

### Path Operations

```alang
// Join paths
let path = pathJoin("home", "user", "file.txt");
// Result: "home/user/file.txt"

// Extract basename
let name = pathBasename("/path/to/file.txt");
// Result: "file.txt"

// Working directory
let cwd = getCwd();
setCwd("/new/path");
```

### Time & Sleep

```alang
// Unix timestamp (seconds)
let now = timestamp();

// Sleep (milliseconds)
sleep(1000);  // Sleep for 1 second
```

---

## 💾 Binary/Bytes Handling

### Hex Encoding

```alang
// Encode to hex
let bytes = [72, 101, 108, 108, 111];  // "Hello"
let hex = toHex(bytes);
// Result: "48656c6c6f"

// Decode from hex
let decoded = fromHex("48656c6c6f");
// Result: [72, 101, 108, 108, 111]
```

### Base64 Encoding

```alang
// Encode string
let encoded = toBase64("Hello, World!");
// Result: "SGVsbG8sIFdvcmxkIQ=="

// Encode bytes
let bytes = [72, 101, 108, 108, 111];
let encoded = toBase64(bytes);

// Decode
let decoded = fromBase64("SGVsbG8sIFdvcmxkIQ==");
// Returns array of bytes
```

### Bit Operations

```alang
let value = 170;  // 0b10101010

// Check if bit is set
let isSet = bitGet(value, 1);  // true

// Set bit
let newValue = bitSet(value, 0);  // 171 (0b10101011)

// Clear bit
let cleared = bitClear(value, 1);  // 168 (0b10101000)

// Toggle bit
let toggled = bitToggle(value, 0);  // 171
```

---

## 🔌 Hardware Simulation (GPIO/I2C/SPI/UART)

### GPIO (Digital I/O)

```alang
// Initialize pin as output
gpioInit(13, "output");

// Write HIGH/LOW
gpioWrite(13, 1);  // Turn on
gpioWrite(13, 0);  // Turn off

// Read pin (input mode)
gpioInit(2, "input_pullup");
let state = gpioRead(2);

// PWM (0.0 to 1.0)
gpioPwm(9, 0.5);   // 50% duty cycle
```

**Pin Modes**:
- `"input"` - High impedance input
- `"output"` - Push-pull output
- `"input_pullup"` - Input with pull-up resistor
- `"input_pulldown"` - Input with pull-down resistor

### I2C (Coming Soon)

```rust
// Rust API (not yet exposed to A-lang)
let mut i2c = I2cController::new();
i2c.add_device(0x48)?;
i2c.write(0x48, &[0x01, 0xFF])?;
let data = i2c.read(0x48, 0x01, 2)?;
```

### SPI (Coming Soon)

```rust
// Rust API (not yet exposed to A-lang)
let mut spi = SpiController::new();
spi.add_device("device0".to_string(), SpiMode::Mode0)?;
let rx = spi.transfer("device0", &[0x01, 0x02, 0x03])?;
```

### UART (Coming Soon)

```rust
// Rust API (not yet exposed to A-lang)
let mut uart = UartController::new();
uart.open("serial0".to_string(), UartConfig::new(9600))?;
uart.write_string("serial0", "Hello")?;
let data = uart.read_string("serial0", 100)?;
```

---

## 🎯 Common Patterns

### IoT Sensor Reading Loop

```alang
fn readSensor() {
    // Simulate sensor reading
    return 22 + (timestamp() % 10);
}

// Monitor loop
for i in 0..10 {
    let temp = readSensor();
    print("Temperature: " + temp + "°C");
    
    // Alert on high temp
    if temp > 25 {
        print("⚠️ WARNING: High temperature!");
    }
    
    sleep(1000);
}
```

### Data Collection & Upload

```alang
fn collectAndUpload() {
    // Collect sensor data
    let data = {
        "deviceId": "sensor_001",
        "timestamp": timestamp(),
        "temperature": readSensor(),
        "humidity": readHumidity()
    };
    
    // Convert to JSON
    let json = stringifyJSON(data);
    
    // Upload to cloud
    let response = httpPost("https://api.iot.com/data", json);
    
    if response.status == 200 {
        print("✓ Data uploaded successfully");
    } else {
        print("✗ Upload failed: " + response.status);
    }
}
```

### Binary Protocol Implementation

```alang
// Encode packet: [START][LENGTH][DATA][CHECKSUM][END]
fn encodePacket(data) {
    let packet = [0xFF];  // START marker
    
    push(packet, len(data));
    
    for byte in data {
        push(packet, byte);
    }
    
    // Simple checksum
    let checksum = 0;
    for byte in data {
        checksum = (checksum + byte) % 256;
    }
    push(packet, checksum);
    
    push(packet, 0xFE);  // END marker
    
    return packet;
}

// Use it
let payload = [0x01, 0x02, 0x03];
let packet = encodePacket(payload);
let hexPacket = toHex(packet);
print("Packet: " + hexPacket);
```

### LED Blink (GPIO)

```alang
fn blinkLed(pin, times, delayMs) {
    gpioInit(pin, "output");
    
    for i in 0..times {
        gpioWrite(pin, 1);
        sleep(delayMs);
        gpioWrite(pin, 0);
        sleep(delayMs);
    }
}

// Blink pin 13 ten times, 500ms on/off
blinkLed(13, 10, 500);
```

### API Client Wrapper

```alang
fn createApiClient(baseUrl, apiKey) {
    return {
        "baseUrl": baseUrl,
        "apiKey": apiKey,
        
        "get": fn(endpoint) {
            let url = this.baseUrl + endpoint;
            let response = httpGet(url);
            return parseJSON(response.body);
        },
        
        "post": fn(endpoint, data) {
            let url = this.baseUrl + endpoint;
            let body = stringifyJSON(data);
            return httpPost(url, body);
        }
    };
}

// Usage
let api = createApiClient("https://api.example.com", "key123");
let users = api.get("/users");
let result = api.post("/users", {"name": "Alice"});
```

---

## 🔧 Error Handling

### Check Command Results

```alang
let result = exec("some-command");
if result.success {
    print("Success: " + result.stdout);
} else {
    print("Error: " + result.stderr);
    print("Exit code: " + result.exitCode);
}
```

### Check HTTP Status

```alang
let response = httpGet("http://api.example.com/data");
if response.status >= 200 && response.status < 300 {
    print("Success!");
} else {
    print("HTTP error: " + response.status);
}
```

### Validate Before Use

```alang
let envVar = getEnv("IMPORTANT_VAR");
if envVar == nil {
    print("Error: IMPORTANT_VAR not set!");
    // Handle error
} else {
    print("Using: " + envVar);
}
```

---

## 📊 Data Types

### HTTP Response Object

```javascript
{
    "status": 200,              // int
    "statusText": "OK",         // string
    "headers": {...},           // object
    "body": "..."              // string
}
```

### Exec Result Object

```javascript
{
    "exitCode": 0,              // int
    "stdout": "...",            // string
    "stderr": "...",            // string
    "success": true             // bool
}
```

### System Info Object

```javascript
{
    "os": "linux",              // string
    "arch": "x86_64",           // string
    "family": "unix",           // string
    "cpus": "8",                // string
    "pid": "12345",             // string
    "username": "user",         // string
    "hostname": "host",         // string
    "home": "/home/user",       // string
    "cwd": "/path",             // string
    "temp": "/tmp"              // string
}
```

---

## 🚀 Performance Tips

1. **Cache repeated operations**
   ```alang
   let info = getSystemInfo();  // Cache this
   // Use info.os, info.arch, etc. multiple times
   ```

2. **Batch network requests**
   ```alang
   // Instead of many small requests, batch data
   let batch = [data1, data2, data3];
   let json = stringifyJSON(batch);
   httpPost(url, json);
   ```

3. **Use appropriate sleep intervals**
   ```alang
   // For sensors: 100-1000ms
   sleep(1000);
   
   // For rapid polling: 10-50ms
   sleep(10);
   ```

4. **Minimize GPIO toggles**
   ```alang
   // Change state only when needed
   let currentState = gpioRead(pin);
   if currentState != desiredState {
       gpioWrite(pin, desiredState);
   }
   ```

---

## 📚 See Also

- **Full Documentation**: `STDLIB_README.md`
- **Examples**: `examples/stdlib_demo.al`
- **Test Suite**: `tests/stdlib_test.rs`
- **Feature List**: `IOT_FEATURES_COMPLETE.md`

---

**Last Updated**: January 2024  
**Version**: 1.0.0