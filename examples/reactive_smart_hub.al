// Reactive Smart Hub Demo
// Demonstrates computed values and effects

print("=== Reactive Smart Hub ===")
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
  print("System Status: " + status)
  if (status == "CRITICAL") {
     print("⚠️  Fan ON - GPIO 13 activated")
  } else {
     print("✓ Fan OFF - System normal")
  }
}

print("")
print("Initial temperature: " + str(temp) + "°C")
print("")

print("Simulating temperature increase...")
sleep(1000)
temp = 32.0

computed status = () => {
  if (temp > MAX_TEMP) {
    return "CRITICAL"
  } else {
    return "NORMAL"
  }
}

effect () => {
  print("System Status: " + status)
  if (status == "CRITICAL") {
     print("⚠️  Fan ON - GPIO 13 activated")
  } else {
     print("✓ Fan OFF - System normal")
  }
}

print("")
print("Temperature raised to: " + str(temp) + "°C")
print("")

print("Cooling down...")
sleep(1000)
temp = 25.0

computed status = () => {
  if (temp > MAX_TEMP) {
    return "CRITICAL"
  } else {
    return "NORMAL"
  }
}

effect () => {
  print("System Status: " + status)
  if (status == "CRITICAL") {
     print("⚠️  Fan ON - GPIO 13 activated")
  } else {
     print("✓ Fan OFF - System normal")
  }
}

print("")
print("Temperature lowered to: " + str(temp) + "°C")
print("")
print("=== Demo Complete ===")
