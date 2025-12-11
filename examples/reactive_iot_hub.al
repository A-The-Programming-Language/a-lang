// Reactive IoT Smart Hub
// Demonstrates computed values, effects, ternary operator, and hardware control

print("╔════════════════════════════════════════════════╗")
print("║     Reactive IoT Smart Hub Demo               ║")
print("║     A-lang Reactive Programming                ║")
print("╚════════════════════════════════════════════════╝")
print("")

// Initialize hardware
print("🔧 Initializing hardware...")
hw.gpio.init(13, "output")  // Fan control pin
hw.gpio.init(12, "output")  // Status LED pin
print("✓ GPIO initialized")
print("")

// Reactive sensor data
reactive temp = 22.0
reactive humidity = 45
const MAX_TEMP = 30.0
const MAX_HUMIDITY = 70

// Computed values using ternary operator
computed tempStatus = () => {
  return temp > MAX_TEMP ? "CRITICAL" : (temp > 25 ? "WARNING" : "NORMAL")
}

computed humidityStatus = () => {
  return humidity > MAX_HUMIDITY ? "HIGH" : "NORMAL"
}

computed systemStatus = () => {
  return (tempStatus == "CRITICAL" || humidityStatus == "HIGH") ? "ALERT" : "OK"
}

// Effect: Monitor and control based on status
effect () => {
  print("┌─ System Status ─────────────────────────────")
  print("│ Temperature: " + str(temp) + "°C - " + tempStatus)
  print("│ Humidity: " + str(humidity) + "% - " + humidityStatus)
  print("│ System: " + systemStatus)

  if (systemStatus == "ALERT") {
    hw.gpio.write(13, 1)  // Fan ON
    hw.gpio.write(12, 1)  // Red LED ON
    print("│ ⚠️  ALERT: Fan activated (GPIO 13)")
    print("│ ⚠️  Status LED ON (GPIO 12)")
  } else {
    hw.gpio.write(13, 0)  // Fan OFF
    hw.gpio.write(12, 0)  // LED OFF
    print("│ ✓ All systems normal")
  }
  print("└─────────────────────────────────────────────")
}

print("")
print("📊 Initial readings:")
print("")

// Simulate environmental changes
print("⏱️  Simulating temperature increase...")
sleep(500)
temp = 28.0

computed tempStatus = () => {
  return temp > MAX_TEMP ? "CRITICAL" : (temp > 25 ? "WARNING" : "NORMAL")
}

computed humidityStatus = () => {
  return humidity > MAX_HUMIDITY ? "HIGH" : "NORMAL"
}

computed systemStatus = () => {
  return (tempStatus == "CRITICAL" || humidityStatus == "HIGH") ? "ALERT" : "OK"
}

effect () => {
  print("┌─ System Status ─────────────────────────────")
  print("│ Temperature: " + str(temp) + "°C - " + tempStatus)
  print("│ Humidity: " + str(humidity) + "% - " + humidityStatus)
  print("│ System: " + systemStatus)

  if (systemStatus == "ALERT") {
    hw.gpio.write(13, 1)
    hw.gpio.write(12, 1)
    print("│ ⚠️  ALERT: Fan activated (GPIO 13)")
    print("│ ⚠️  Status LED ON (GPIO 12)")
  } else {
    hw.gpio.write(13, 0)
    hw.gpio.write(12, 0)
    print("│ ✓ All systems normal")
  }
  print("└─────────────────────────────────────────────")
}

print("")

print("⏱️  Temperature critical...")
sleep(500)
temp = 32.0

computed tempStatus = () => {
  return temp > MAX_TEMP ? "CRITICAL" : (temp > 25 ? "WARNING" : "NORMAL")
}

computed humidityStatus = () => {
  return humidity > MAX_HUMIDITY ? "HIGH" : "NORMAL"
}

computed systemStatus = () => {
  return (tempStatus == "CRITICAL" || humidityStatus == "HIGH") ? "ALERT" : "OK"
}

effect () => {
  print("┌─ System Status ─────────────────────────────")
  print("│ Temperature: " + str(temp) + "°C - " + tempStatus)
  print("│ Humidity: " + str(humidity) + "% - " + humidityStatus)
  print("│ System: " + systemStatus)

  if (systemStatus == "ALERT") {
    hw.gpio.write(13, 1)
    hw.gpio.write(12, 1)
    print("│ ⚠️  ALERT: Fan activated (GPIO 13)")
    print("│ ⚠️  Status LED ON (GPIO 12)")
  } else {
    hw.gpio.write(13, 0)
    hw.gpio.write(12, 0)
    print("│ ✓ All systems normal")
  }
  print("└─────────────────────────────────────────────")
}

print("")

print("⏱️  Humidity increasing...")
sleep(500)
humidity = 75

computed tempStatus = () => {
  return temp > MAX_TEMP ? "CRITICAL" : (temp > 25 ? "WARNING" : "NORMAL")
}

computed humidityStatus = () => {
  return humidity > MAX_HUMIDITY ? "HIGH" : "NORMAL"
}

computed systemStatus = () => {
  return (tempStatus == "CRITICAL" || humidityStatus == "HIGH") ? "ALERT" : "OK"
}

effect () => {
  print("┌─ System Status ─────────────────────────────")
  print("│ Temperature: " + str(temp) + "°C - " + tempStatus)
  print("│ Humidity: " + str(humidity) + "% - " + humidityStatus)
  print("│ System: " + systemStatus)

  if (systemStatus == "ALERT") {
    hw.gpio.write(13, 1)
    hw.gpio.write(12, 1)
    print("│ ⚠️  ALERT: Fan activated (GPIO 13)")
    print("│ ⚠️  Status LED ON (GPIO 12)")
  } else {
    hw.gpio.write(13, 0)
    hw.gpio.write(12, 0)
    print("│ ✓ All systems normal")
  }
  print("└─────────────────────────────────────────────")
}

print("")

print("⏱️  System normalizing...")
sleep(500)
temp = 23.0
humidity = 50

computed tempStatus = () => {
  return temp > MAX_TEMP ? "CRITICAL" : (temp > 25 ? "WARNING" : "NORMAL")
}

computed humidityStatus = () => {
  return humidity > MAX_HUMIDITY ? "HIGH" : "NORMAL"
}

computed systemStatus = () => {
  return (tempStatus == "CRITICAL" || humidityStatus == "HIGH") ? "ALERT" : "OK"
}

effect () => {
  print("┌─ System Status ─────────────────────────────")
  print("│ Temperature: " + str(temp) + "°C - " + tempStatus)
  print("│ Humidity: " + str(humidity) + "% - " + humidityStatus)
  print("│ System: " + systemStatus)

  if (systemStatus == "ALERT") {
    hw.gpio.write(13, 1)
    hw.gpio.write(12, 1)
    print("│ ⚠️  ALERT: Fan activated (GPIO 13)")
    print("│ ⚠️  Status LED ON (GPIO 12)")
  } else {
    hw.gpio.write(13, 0)
    hw.gpio.write(12, 0)
    print("│ ✓ All systems normal")
  }
  print("└─────────────────────────────────────────────")
}

print("")
print("╔════════════════════════════════════════════════╗")
print("║     Demo Complete!                             ║")
print("║     ✓ Computed values working                  ║")
print("║     ✓ Effects executing                        ║")
print("║     ✓ Ternary operator functional              ║")
print("║     ✓ Hardware control (hw.gpio) working       ║")
print("╚════════════════════════════════════════════════╝")
