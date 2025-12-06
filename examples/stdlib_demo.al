// ============================================================================
// A-lang Standard Library Demo - IoT/Networking/Low-Level Features
// ============================================================================
// This file demonstrates the extended stdlib features for IoT development,
// networking, and low-level system operations.
// ============================================================================

print("=== A-lang Standard Library Demo ===");
print("");

// ============================================================================
// 1. SYSTEM UTILITIES
// ============================================================================
print("--- 1. System Utilities ---");

// Get system information
let sysInfo = getSystemInfo();
print("Operating System: " + sysInfo.os);
print("Architecture: " + sysInfo.arch);
print("CPUs: " + sysInfo.cpus);
print("Current Directory: " + getCwd());
print("");

// Environment variables
setEnv("ALANG_TEST", "Hello from A-lang!");
let testVar = getEnv("ALANG_TEST");
print("Environment variable: " + testVar);
print("");

// Execute system command
print("Executing 'echo Hello from shell':");
let result = exec("echo Hello from shell");
print("Exit code: " + result.exitCode);
print("Output: " + result.stdout);
print("Success: " + result.success);
print("");

// Path utilities
let fullPath = pathJoin("home", "user", "documents", "file.txt");
print("Joined path: " + fullPath);
let baseName = pathBasename(fullPath);
print("Basename: " + baseName);
print("");

// Timestamps
let now = timestamp();
print("Current timestamp: " + now);
print("");

// ============================================================================
// 2. NETWORK FUNCTIONS
// ============================================================================
print("--- 2. Network Functions ---");

// Parse URL
let urlParts = parseUrl("http://example.com:8080/api/users");
print("Protocol: " + urlParts.protocol);
print("Host: " + urlParts.host);
print("Port: " + urlParts.port);
print("Path: " + urlParts.path);
print("");

// Check if port is available
let portAvailable = isPortAvailable(8080);
print("Is port 8080 available? " + portAvailable);
print("");

// Get local IP (may fail in some environments)
// Uncomment to test:
// let localIp = getLocalIp();
// print("Local IP: " + localIp);

// HTTP GET request (example - requires network)
// Uncomment to test with a real endpoint:
// print("Making HTTP GET request...");
// let response = httpGet("http://httpbin.org/get");
// print("Status: " + response.status);
// print("Body preview: " + response.body);
print("HTTP functions available: httpGet, httpPost");
print("");

// ============================================================================
// 3. BINARY/BYTES HANDLING
// ============================================================================
print("--- 3. Binary/Bytes Handling ---");

// Hex encoding
let data = [72, 101, 108, 108, 111]; // "Hello" in ASCII
let hexString = toHex(data);
print("Hex encoded: " + hexString);

let decoded = fromHex(hexString);
print("Hex decoded: " + decoded);
print("");

// Base64 encoding
let message = "Hello, World!";
let base64 = toBase64(message);
print("Base64 encoded: " + base64);

let decodedB64 = fromBase64(base64);
print("Base64 decoded bytes: " + decodedB64);
print("");

// Bit operations
let value = 170; // 0b10101010
print("Original value: " + value);

let bit1 = bitGet(value, 1);
print("Bit 1 is: " + bit1);

let newValue = bitSet(value, 0);
print("After setting bit 0: " + newValue);
print("");

// ============================================================================
// 4. HARDWARE SIMULATION (GPIO, I2C, SPI, UART)
// ============================================================================
print("--- 4. Hardware Simulation ---");

print("GPIO Functions:");
print("  gpioInit(pin, mode) - Initialize GPIO pin");
print("  gpioWrite(pin, state) - Write to GPIO pin");
print("  gpioRead(pin) - Read from GPIO pin");
print("");

print("Example GPIO usage:");
print("  gpioInit(13, 'output');  // LED pin");
print("  gpioWrite(13, 1);        // Turn on LED");
print("  sleep(1000);             // Wait 1 second");
print("  gpioWrite(13, 0);        // Turn off LED");
print("");

print("I2C Functions:");
print("  i2cInit(address) - Initialize I2C device");
print("  i2cWrite(address, register, data) - Write to I2C");
print("  i2cRead(address, register, count) - Read from I2C");
print("");

print("SPI Functions:");
print("  spiInit(device, mode) - Initialize SPI device");
print("  spiTransfer(device, data) - SPI transfer");
print("");

print("UART Functions:");
print("  uartOpen(port, baudRate) - Open UART port");
print("  uartWrite(port, data) - Write to UART");
print("  uartRead(port, count) - Read from UART");
print("");

// ============================================================================
// 5. IoT EXAMPLE: TEMPERATURE MONITORING SYSTEM
// ============================================================================
print("--- 5. IoT Example: Temperature Monitor ---");

// Simulated temperature sensor
fn readTemperature() {
    // In real hardware, this would read from an I2C/SPI sensor
    // For demo, we generate a random-ish value
    let base = 20;
    let variance = timestamp() % 10;
    return base + variance;
}

// Monitor temperature
fn monitorTemperature(duration) {
    print("Starting temperature monitoring...");
    let startTime = timestamp();
    let readings = [];

    for i in 0..duration {
        let temp = readTemperature();
        push(readings, temp);
        print("Reading " + i + ": " + temp + "°C");

        // Alert if temperature is too high
        if temp > 25 {
            print("  ⚠️  WARNING: High temperature detected!");
        }

        sleep(100); // Wait 100ms between readings
    }

    // Calculate average
    let sum = 0;
    for temp in readings {
        sum = sum + temp;
    }
    let avg = sum / len(readings);
    print("Average temperature: " + avg + "°C");

    return readings;
}

// Run monitoring for 5 readings
let tempData = monitorTemperature(5);
print("");

// ============================================================================
// 6. NETWORKING EXAMPLE: API CLIENT
// ============================================================================
print("--- 6. Networking Example: API Client ---");

fn apiClient(baseUrl) {
    return {
        "baseUrl": baseUrl,

        "get": fn(endpoint) {
            let url = this.baseUrl + endpoint;
            print("GET " + url);
            // return httpGet(url); // Uncomment for real usage
            return {"status": 200, "data": "Mock response"};
        },

        "post": fn(endpoint, data) {
            let url = this.baseUrl + endpoint;
            print("POST " + url);
            print("Data: " + data);
            // return httpPost(url, data); // Uncomment for real usage
            return {"status": 201, "data": "Created"};
        }
    };
}

// Create API client
let api = apiClient("https://api.example.com");
print("API client created for: https://api.example.com");
print("");

// ============================================================================
// 7. DATA SERIALIZATION EXAMPLE
// ============================================================================
print("--- 7. Data Serialization ---");

// Prepare sensor data
let sensorData = {
    "deviceId": "sensor_001",
    "timestamp": timestamp(),
    "temperature": 23.5,
    "humidity": 65.2,
    "location": {
        "lat": 37.7749,
        "lon": -122.4194
    }
};

print("Sensor data object:");
print("  Device: " + sensorData.deviceId);
print("  Temp: " + sensorData.temperature + "°C");
print("  Humidity: " + sensorData.humidity + "%");
print("");

// Convert to JSON and encode
let jsonData = stringifyJSON(sensorData);
let encodedData = toBase64(jsonData);
print("Encoded data length: " + len(encodedData) + " bytes");
print("");

// ============================================================================
// 8. PROTOCOL IMPLEMENTATION EXAMPLE
// ============================================================================
print("--- 8. Protocol Implementation ---");

// Simple protocol: [START][LENGTH][DATA][CHECKSUM][END]
fn encodeProtocol(data) {
    let packet = [0xFF]; // START byte

    // Add length
    push(packet, len(data));

    // Add data
    for byte in data {
        push(packet, byte);
    }

    // Calculate simple checksum (sum of bytes % 256)
    let checksum = 0;
    for byte in data {
        checksum = checksum + byte;
    }
    push(packet, checksum % 256);

    // END byte
    push(packet, 0xFE);

    return packet;
}

fn decodeProtocol(packet) {
    if len(packet) < 4 {
        return {"error": "Packet too short"};
    }

    if packet[0] != 255 {
        return {"error": "Invalid START byte"};
    }

    let dataLen = packet[1];
    let data = [];

    for i in 0..dataLen {
        push(data, packet[2 + i]);
    }

    return {"data": data, "valid": true};
}

// Test protocol
let testData = [65, 66, 67]; // "ABC"
let encoded = encodeProtocol(testData);
print("Encoded packet: " + encoded);

let decoded = decodeProtocol(encoded);
if decoded.valid {
    print("Decoded data: " + decoded.data);
    print("Protocol validation: ✓ SUCCESS");
} else {
    print("Protocol validation: ✗ FAILED");
}
print("");

// ============================================================================
// 9. PERFORMANCE TIMING
// ============================================================================
print("--- 9. Performance Timing ---");

fn heavyComputation(n) {
    let result = 0;
    for i in 0..n {
        result = result + i * i;
    }
    return result;
}

let startTime = timestamp();
let result = heavyComputation(1000);
let endTime = timestamp();
let duration = endTime - startTime;

print("Computation result: " + result);
print("Time taken: " + duration + " seconds");
print("");

// ============================================================================
// 10. COMPLETE IoT DEVICE SIMULATION
// ============================================================================
print("--- 10. Complete IoT Device Simulation ---");

fn iotDevice(deviceId) {
    return {
        "id": deviceId,
        "sensors": {},
        "status": "idle",

        "addSensor": fn(name, readFn) {
            this.sensors[name] = readFn;
            print("Sensor '" + name + "' added to " + this.id);
        },

        "readSensors": fn() {
            let readings = {};
            print("Reading sensors for " + this.id + "...");

            for sensorName in keys(this.sensors) {
                let readFn = this.sensors[sensorName];
                readings[sensorName] = readFn();
                print("  " + sensorName + ": " + readings[sensorName]);
            }

            return readings;
        },

        "sendData": fn(readings) {
            let payload = {
                "deviceId": this.id,
                "timestamp": timestamp(),
                "readings": readings
            };

            print("Sending data to cloud...");
            // In real scenario: httpPost(apiEndpoint, stringifyJSON(payload))
            print("Data sent successfully (simulated)");

            return true;
        },

        "run": fn(iterations) {
            this.status = "running";
            print("Device " + this.id + " starting...");

            for i in 0..iterations {
                print("");
                print("Cycle " + (i + 1) + ":");
                let readings = this.readSensors();
                this.sendData(readings);
                sleep(200);
            }

            this.status = "idle";
            print("Device " + this.id + " stopped.");
        }
    };
}

// Create IoT device
let device = iotDevice("ESP32_001");

// Add sensors
device.addSensor("temperature", fn() { return 22 + (timestamp() % 5); });
device.addSensor("humidity", fn() { return 60 + (timestamp() % 20); });
device.addSensor("pressure", fn() { return 1013 + (timestamp() % 10); });

print("");

// Run device for 3 cycles
device.run(3);

print("");
print("=== Demo Complete ===");
print("");
print("Available stdlib features:");
print("  ✓ System utilities (exec, env, paths)");
print("  ✓ Network functions (HTTP, TCP/UDP, sockets)");
print("  ✓ Binary handling (hex, base64, bit ops)");
print("  ✓ Hardware simulation (GPIO, I2C, SPI, UART)");
print("  ✓ FFI for C/C++ interop (advanced)");
print("  ✓ Struct packing for C compatibility");
print("");
print("Perfect for IoT, networking, and low-level development!");
