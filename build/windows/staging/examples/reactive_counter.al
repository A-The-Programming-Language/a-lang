// Reactive Counter Example
// Demonstrates A-lang's reactive programming with simple syntax

print("=== Reactive Counter Demo ===")
print("")

// Create reactive variables (changes auto-propagate)
reactive counter = 0

print("Initial counter: " + counter)
print("")

// Update counter with simple = syntax
print("Setting counter to 5...")
counter = 5
print("Counter: " + counter)
print("")

// Another update
print("Setting counter to 10...")
counter = 10
print("Counter: " + counter)
print("")

// More reactive variables
reactive width = 10
reactive height = 20

print("Rectangle dimensions:")
print("Width: " + width)
print("Height: " + height)
print("")

print("Changing dimensions...")
width = 15
height = 25
print("Width: " + width)
print("Height: " + height)
print("")

// Reactive is powerful but syntax is simple!
reactive score = 0
print("Game score: " + score)

score = score + 100
print("Got 100 points! Score: " + score)

score = score + 50
print("Got 50 points! Score: " + score)

print("")
print("=== Demo Complete ===")
