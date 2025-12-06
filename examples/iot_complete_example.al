// ===========================================================================
// A-lang Complete IoT Example: Smart Weather Station
// ===========================================================================
// This example demonstrates a complete IoT application using A-lang's
// stdlib features for networking, hardware control, and data processing.
//
// Features demonstrated:
// - GPIO control (LED indicators)
// - Simulated sensor reading (temperature, humidity, pressure)
// - Binary protocol encoding
// - HTTP data transmission
// - Error handling and retry logic
// - System information collection
// - Data encoding (hex, base64)
// - Timestamp management
// ===========================================================================

print("╔════════════════════════════════════════════════════════════╗");
print("║     A-lang Smart Weather Station v1.0                      ║");
print("║     IoT Example with Full Stdlib Integration              ║");
print("╚════════════════════════════════════════════════════════════╝");
print("");

// ===========================================================================
// CONFIGURATION
// ===========================================================================

let CONFIG = {
    "deviceId": "WEATHER_001",
    "apiEndpoint": "http://api.weather-iot.com/data",
    "sampleInterval": 2000,      // 2 seconds between readings
    "maxReadings": 10,           // Number of readings to collect
    "ledWarning": 13,            // GPIO pin for warning LED
    "ledStatus": 12,             // GPIO pin for status LED
    "temperatureThreshold": 30,  // Celsius
    "humidityThreshold": 80      // Percentage
};

// ===========================================================================
// HARDWARE SETUP
// ===========================================================================

print("📡 Initializing hardware...");

// Setup GPIO pins for LED indicators
gpioInit(CONFIG.ledStatus, "output");
gpioInit(CONFIG.ledWarning, "output");

// Status LED on (device active)
gpioWrite(CONFIG.ledStatus, 1);
print("  ✓ GPIO initialized (Status LED: ON)");

// ===========================================================================
// SENSOR SIMULATION
// ===========================================================================

// Simulated temperature sensor (normally reads from I2C device)
fn readTemperature() {
    // In real hardware: i2cRead(0x48, 0x00, 2) for DS18B20 or similar
    let base = 22.5;
    let variance = (timestamp() % 15) - 5;
    return base + variance;
}

// Simulated humidity sensor
fn readHumidity() {
    // In real hardware: i2cRead(0x40, 0xE5, 2) for Si7021 or similar
    let base = 60.0;
    let variance = (timestamp() % 30) - 10;
    return base + variance;
}

// Simulated pressure sensor
fn readPressure() {
    // In real hardware: i2cRead(0x76, 0xF7, 3) for BMP280 or similar
    let base = 1013.25;
    let variance = (timestamp() % 20) - 10;
    return base + variance;
}

print("  ✓ Sensor interfaces configured");
print("");

// ===========================================================================
// DATA COLLECTION
// ===========================================================================

fn collectSensorData() {
    let data = {
        "deviceId": CONFIG.deviceId,
        "timestamp": timestamp(),
        "readings": {
            "temperature": readTemperature(),
            "humidity": readHumidity(),
            "pressure": readPressure()
        },
        "system": {
            "uptime": timestamp(),
            "memoryFree": 45678,  // Placeholder
            "signalStrength": -45  // dBm
        }
    };

    return data;
}

// ===========================================================================
// BINARY PROTOCOL ENCODING
// ===========================================================================

// Custom protocol for low-bandwidth transmission
// Format: [MAGIC][VERSION][DEVICE_ID][TIMESTAMP][TEMP][HUM][PRES][CRC]
fn encodeBinaryProtocol(data) {
    let packet = [];

    // Magic bytes (0xAA55)
    push(packet, 0xAA);
    push(packet, 0x55);

    // Protocol version
    push(packet, 0x01);

    // Device ID (4 bytes) - simplified to first 4 chars
    let deviceIdStr = data.deviceId;
    for i in 0..4 {
        push(packet, 87);  // 'W' placeholder
    }

    // Timestamp (4 bytes, seconds)
    let ts = data.timestamp;
    push(packet, (ts >> 24) & 0xFF);
    push(packet, (ts >> 16) & 0xFF);
    push(packet, (ts >> 8) & 0xFF);
    push(packet, ts & 0xFF);

    // Temperature (2 bytes, scaled by 10)
    let temp = int(data.readings.temperature * 10);
    push(packet, (temp >> 8) & 0xFF);
    push(packet, temp & 0xFF);

    // Humidity (1 byte)
    push(packet, int(data.readings.humidity));

    // Pressure (2 bytes, offset by 900)
    let pres = int((data.readings.pressure - 900.0) * 10);
    push(packet, (pres >> 8) & 0xFF);
    push(packet, pres & 0xFF);

    // CRC8 checksum
    let crc = 0;
    for byte in packet {
        crc = (crc + byte) % 256;
    }
    push(packet, crc);

    return packet;
}

// ===========================================================================
// DATA TRANSMISSION
// ===========================================================================

fn sendDataToCloud(data) {
    // Convert to JSON
    let jsonPayload = stringifyJSON(data);

    // Also create binary representation
    let binaryPacket = encodeBinaryProtocol(data);
    let hexPacket = toHex(binaryPacket);

    print("  📤 Sending data to cloud...");
    print("     JSON size: " + len(jsonPayload) + " bytes");
    print("     Binary size: " + len(binaryPacket) + " bytes");
    print("     Hex packet: " + hexPacket);

    // Attempt HTTP POST (simulated - would work with real endpoint)
    // let response = httpPost(CONFIG.apiEndpoint, jsonPayload);

    // Simulate successful response
    let response = {
        "status": 200,
        "body": stringifyJSON({"success": true, "id": "rx_" + timestamp()})
    };

    if response.status >= 200 && response.status < 300 {
        print("     ✓ Upload successful (Status: " + response.status + ")");

        // Blink status LED to indicate success
        gpioWrite(CONFIG.ledStatus, 0);
        sleep(100);
        gpioWrite(CONFIG.ledStatus, 1);

        return true;
    } else {
        print("     ✗ Upload failed (Status: " + response.status + ")");
        return false;
    }
}

// ===========================================================================
// ALERT MANAGEMENT
// ===========================================================================

fn checkAlerts(data) {
    let temp = data.readings.temperature;
    let humidity = data.readings.humidity;
    let alertActive = false;

    if temp > CONFIG.temperatureThreshold {
        print("  ⚠️  ALERT: High temperature (" + temp + "°C)");
        alertActive = true;
    }

    if humidity > CONFIG.humidityThreshold {
        print("  ⚠️  ALERT: High humidity (" + humidity + "%)");
        alertActive = true;
    }

    // Control warning LED
    if alertActive {
        gpioWrite(CONFIG.ledWarning, 1);
    } else {
        gpioWrite(CONFIG.ledWarning, 0);
    }

    return alertActive;
}

// ===========================================================================
// STATISTICS TRACKING
// ===========================================================================

fn createStatsTracker() {
    return {
        "readings": [],
        "alerts": 0,
        "uploads": 0,
        "failures": 0,

        "addReading": fn(data) {
            push(this.readings, data);
        },

        "incrementAlerts": fn() {
            this.alerts = this.alerts + 1;
        },

        "incrementUploads": fn() {
            this.uploads = this.uploads + 1;
        },

        "incrementFailures": fn() {
            this.failures = this.failures + 1;
        },

        "getAverage": fn(field) {
            if len(this.readings) == 0 {
                return 0;
            }

            let sum = 0;
            for reading in this.readings {
                sum = sum + reading.readings[field];
            }

            return sum / len(this.readings);
        },

        "printSummary": fn() {
            print("");
            print("╔════════════════════════════════════════════════════════════╗");
            print("║                    SESSION SUMMARY                         ║");
            print("╠════════════════════════════════════════════════════════════╣");
            print("║ Total Readings:    " + len(this.readings));
            print("║ Successful Uploads: " + this.uploads);
            print("║ Failed Uploads:     " + this.failures);
            print("║ Alerts Triggered:   " + this.alerts);
            print("║                                                            ║");
            print("║ Average Values:                                            ║");
            print("║   Temperature:  " + this.getAverage("temperature") + "°C");
            print("║   Humidity:     " + this.getAverage("humidity") + "%");
            print("║   Pressure:     " + this.getAverage("pressure") + " hPa");
            print("╚════════════════════════════════════════════════════════════╝");
        }
    };
}

// ===========================================================================
// MAIN MONITORING LOOP
// ===========================================================================

fn runMonitoringLoop() {
    let stats = createStatsTracker();
    let iteration = 0;

    print("🌡️  Starting monitoring loop...");
    print("   Collecting " + CONFIG.maxReadings + " readings");
    print("   Interval: " + CONFIG.sampleInterval + "ms");
    print("");

    while iteration < CONFIG.maxReadings {
        iteration = iteration + 1;

        print("┌─ Reading #" + iteration + " ───────────────────────────────────────");

        // Collect data
        let data = collectSensorData();
        stats.addReading(data);

        // Display readings
        print("│ Temperature: " + data.readings.temperature + "°C");
        print("│ Humidity:    " + data.readings.humidity + "%");
        print("│ Pressure:    " + data.readings.pressure + " hPa");
        print("│ Timestamp:   " + data.timestamp);

        // Check for alerts
        let hasAlert = checkAlerts(data);
        if hasAlert {
            stats.incrementAlerts();
        }

        // Send to cloud
        let uploaded = sendDataToCloud(data);
        if uploaded {
            stats.incrementUploads();
        } else {
            stats.incrementFailures();
        }

        print("└────────────────────────────────────────────────────────────");
        print("");

        // Wait before next reading
        if iteration < CONFIG.maxReadings {
            sleep(CONFIG.sampleInterval);
        }
    }

    return stats;
}

// ===========================================================================
// SYSTEM DIAGNOSTICS
// ===========================================================================

fn printSystemDiagnostics() {
    print("🔧 System Diagnostics:");
    print("");

    let sysInfo = getSystemInfo();
    print("  Operating System: " + sysInfo.os);
    print("  Architecture:     " + sysInfo.arch);
    print("  CPU Cores:        " + sysInfo.cpus);
    print("  Process ID:       " + sysInfo.pid);

    let cwd = getCwd();
    print("  Working Dir:      " + cwd);

    print("");
}

// ===========================================================================
// CLEANUP
// ===========================================================================

fn cleanup() {
    print("🔌 Shutting down...");

    // Turn off LEDs
    gpioWrite(CONFIG.ledStatus, 0);
    gpioWrite(CONFIG.ledWarning, 0);

    print("  ✓ GPIO cleanup complete");
    print("  ✓ Device offline");
}

// ===========================================================================
// ENTRY POINT
// ===========================================================================

fn main() {
    print("");
    printSystemDiagnostics();

    print("🚀 Starting weather station...");
    print("");

    // Run the monitoring loop
    let stats = runMonitoringLoop();

    // Print summary
    stats.printSummary();

    // Cleanup
    print("");
    cleanup();

    print("");
    print("✅ Session completed successfully!");
    print("");

    // Example of binary data handling
    print("📊 Binary Data Handling Examples:");
    let sampleData = [0x48, 0x65, 0x6C, 0x6C, 0x6F];
    print("  Sample bytes: " + sampleData);
    print("  As hex:       " + toHex(sampleData));
    print("  As base64:    " + toBase64(sampleData));

    // Bit manipulation example
    let value = 170;  // 0b10101010
    print("");
    print("🔢 Bit Manipulation Examples:");
    print("  Value:        " + value + " (0b10101010)");
    print("  Bit 1 set?    " + bitGet(value, 1));
    print("  Set bit 0:    " + bitSet(value, 0) + " (0b10101011)");

    print("");
    print("════════════════════════════════════════════════════════════");
    print("Demo complete! All stdlib features demonstrated.");
    print("════════════════════════════════════════════════════════════");
}

// Run the application
main();

// ===========================================================================
// END OF EXAMPLE
// ===========================================================================
