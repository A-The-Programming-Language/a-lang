// Reactive Features Demo
// Demonstrates computed values and effects in A-lang

print("╔═══════════════════════════════════════════════╗")
print("║     A-lang Reactive Features Demo            ║")
print("║     Computed Values & Effects                 ║")
print("╚═══════════════════════════════════════════════╝")
print("")

print("─── Example 1: Smart Temperature Monitor ───")
print("")

reactive temperature = 20.0
const TEMP_THRESHOLD = 25.0

computed tempStatus = () => {
  if (temperature > TEMP_THRESHOLD) {
    return "HOT"
  } else {
    return "COOL"
  }
}

effect () => {
  print("Temperature: " + str(temperature) + "°C - Status: " + tempStatus)
}

print("")
print("Increasing temperature...")
temperature = 28.0

computed tempStatus = () => {
  if (temperature > TEMP_THRESHOLD) {
    return "HOT"
  } else {
    return "COOL"
  }
}

effect () => {
  print("Temperature: " + str(temperature) + "°C - Status: " + tempStatus)
}

print("")
print("─── Example 2: Smart Home Hub ───")
print("")

reactive temp = 24.5
const MAX_TEMP = 30.0

computed status = () => {
  if (temp > MAX_TEMP) {
    return "CRITICAL"
  } else {
    return "NORMAL"
  }
}

effect () => {
  print("🏠 System Status: " + status)
  if (status == "CRITICAL") {
    print("   ⚠️  Fan ON - GPIO 13 activated")
  } else {
    print("   ✓ All systems normal")
  }
}

print("")
print("Simulating temperature spike to 32°C...")
temp = 32.0

computed status = () => {
  if (temp > MAX_TEMP) {
    return "CRITICAL"
  } else {
    return "NORMAL"
  }
}

effect () => {
  print("🏠 System Status: " + status)
  if (status == "CRITICAL") {
    print("   ⚠️  Fan ON - GPIO 13 activated")
  } else {
    print("   ✓ All systems normal")
  }
}

print("")
print("─── Example 3: Counter with Computed Values ───")
print("")

reactive counter = 0

computed double = () => {
  return counter * 2
}

computed square = () => {
  return counter * counter
}

effect () => {
  print("Counter: " + str(counter))
  print("  Double: " + str(double))
  print("  Square: " + str(square))
}

print("")
print("Incrementing counter to 5...")
counter = 5

computed double = () => {
  return counter * 2
}

computed square = () => {
  return counter * counter
}

effect () => {
  print("Counter: " + str(counter))
  print("  Double: " + str(double))
  print("  Square: " + str(square))
}

print("")
print("Incrementing counter to 10...")
counter = 10

computed double = () => {
  return counter * 2
}

computed square = () => {
  return counter * counter
}

effect () => {
  print("Counter: " + str(counter))
  print("  Double: " + str(double))
  print("  Square: " + str(square))
}

print("")
print("─── Example 4: Alert System ───")
print("")

reactive batteryLevel = 100

computed batteryStatus = () => {
  if (batteryLevel < 20) {
    return "CRITICAL"
  } elif (batteryLevel < 50) {
    return "LOW"
  } else {
    return "OK"
  }
}

effect () => {
  print("🔋 Battery: " + str(batteryLevel) + "% - " + batteryStatus)
  if (batteryStatus == "CRITICAL") {
    print("   🚨 CHARGE NOW!")
  } elif (batteryStatus == "LOW") {
    print("   ⚠️  Consider charging soon")
  }
}

print("")
print("Draining battery to 45%...")
batteryLevel = 45

computed batteryStatus = () => {
  if (batteryLevel < 20) {
    return "CRITICAL"
  } elif (batteryLevel < 50) {
    return "LOW"
  } else {
    return "OK"
  }
}

effect () => {
  print("🔋 Battery: " + str(batteryLevel) + "% - " + batteryStatus)
  if (batteryStatus == "CRITICAL") {
    print("   🚨 CHARGE NOW!")
  } elif (batteryStatus == "LOW") {
    print("   ⚠️  Consider charging soon")
  }
}

print("")
print("Draining battery to 15%...")
batteryLevel = 15

computed batteryStatus = () => {
  if (batteryLevel < 20) {
    return "CRITICAL"
  } elif (batteryLevel < 50) {
    return "LOW"
  } else {
    return "OK"
  }
}

effect () => {
  print("🔋 Battery: " + str(batteryLevel) + "% - " + batteryStatus)
  if (batteryStatus == "CRITICAL") {
    print("   🚨 CHARGE NOW!")
  } elif (batteryStatus == "LOW") {
    print("   ⚠️  Consider charging soon")
  }
}

print("")
print("╔═══════════════════════════════════════════════╗")
print("║     Demo Complete!                            ║")
print("║     Computed & Effect Features Working ✓      ║")
print("╚═══════════════════════════════════════════════╝")
