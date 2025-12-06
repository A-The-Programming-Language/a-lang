// Fibonacci with Time-Travel Debugging
// Demonstrates A-lang's revolutionary time-travel feature

print("=== Fibonacci with Time-Travel ===")
print("")

fn fibonacci(n) {
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

// Calculate fibonacci(10)
n = 10
print("Computing fibonacci(" + n + ")...")

checkpoint "before_fib"
result = fibonacci(n)
print("Result: " + result)

print("")
print("Taking a snapshot of current state...")
snapshot

// Change something
x = 100
print("x = " + x)

// Now let's time-travel back!
print("")
print("Rewinding to checkpoint...")
rewind to "before_fib"

// Try with a smaller number
n2 = 7
print("Computing fibonacci(" + n2 + ") after rewind...")
result2 = fibonacci(n2)
print("Result: " + result2)

print("")
print("=== Time-Travel Features ===")
print("  snapshot    - Save current state")
print("  checkpoint  - Named save point")
print("  rewind N    - Go back N steps")
print("  rewind to X - Jump to checkpoint X")

print("")
print("=== Demo Complete ===")
