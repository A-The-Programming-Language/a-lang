// A-lang: JavaScript-Style Syntax Demo
// Shows how A-lang combines JS familiarity with revolutionary features

print("╔════════════════════════════════════════════════╗")
print("║  A-lang: JavaScript-Style Syntax Demo         ║")
print("╚════════════════════════════════════════════════╝")
print("")

// ===== VARIABLES =====
print("=== Variables ===")

// No let/var needed - just assign!
name = "Alice"
age = 25
isStudent = true
print("Name: " + name)
print("Age: " + age)

// Constants still work
const PI = 3.14159
const MAX_USERS = 100
print("PI: " + PI)
print("")

// ===== IF/ELIF/ELSE (JS Style with parentheses) =====
print("=== Control Flow ===")

score = 85
if (score >= 90) {
    grade = "A"
} elif (score >= 80) {
    grade = "B"
} elif (score >= 70) {
    grade = "C"
} else {
    grade = "F"
}
print("Score: " + score + " -> Grade: " + grade)
print("")

// ===== WHILE LOOPS (with parentheses) =====
print("=== While Loop ===")
countdown = 3
while (countdown > 0) {
    print("  " + countdown + "...")
    countdown = countdown - 1
}
print("  Blast off!")
print("")

// ===== FOR-IN LOOPS =====
print("=== For-In Loops ===")

// Array iteration
fruits = ["apple", "banana", "cherry"]
print("Fruits:")
for (fruit in fruits) {
    print("  - " + fruit)
}

// Range iteration
print("Range 1..5:")
for (i in 1..6) {
    print("  " + i)
}
print("")

// ===== FUNCTIONS =====
print("=== Functions ===")

fn greet(name) {
    return "Hello, " + name + "!"
}

fn calculateArea(width, height) {
    return width * height
}

print(greet("Bob"))
area = calculateArea(5, 10)
print("Area: " + area)
print("")

// ===== TRY/CATCH/FINALLY (JS Style) =====
print("=== Try/Catch/Finally ===")

fn divide(a, b) {
    if (b == 0) {
        throw "Cannot divide by zero!"
    }
    return a / b
}

try {
    result = divide(10, 2)
    print("10 / 2 = " + result)

    result = divide(10, 0)
    print("This won't print")
} catch (error) {
    print("Error: " + error)
} finally {
    print("Cleanup done")
}
print("")

// ===== ARRAYS & OBJECTS =====
print("=== Arrays & Objects ===")

numbers = [1, 2, 3, 4, 5]
print("Array: " + numbers[0] + ", " + numbers[1])

person = {
    name: "Charlie",
    age: 30,
    city: "TechCity"
}
print("Person: " + person.name + " from " + person.city)
print("")

// ===== REACTIVE VARIABLES (WOW #2) =====
print("=== Reactive Variables (WOW Factor) ===")

reactive counter = 0
print("Counter: " + counter)

counter = 5
print("Counter updated: " + counter)

counter = counter + 10
print("Counter after +10: " + counter)
print("")

// ===== TIME-TRAVEL DEBUGGING (WOW #1) =====
print("=== Time-Travel Debugging (WOW Factor) ===")

x = 100
print("x = " + x)

checkpoint "before_change"
x = 200
print("x changed to: " + x)

snapshot
x = 300
print("x changed again to: " + x)

print("Rewinding to checkpoint...")
rewind to "before_change"
print("x after rewind: " + x)
print("")

// ===== FIBONACCI EXAMPLE =====
print("=== Fibonacci Function ===")

fn fib(n) {
    if (n <= 1) {
        return n
    }
    return fib(n - 1) + fib(n - 2)
}

print("fib(7) = " + fib(7))
print("fib(10) = " + fib(10))
print("")

// ===== NESTED CONDITIONS =====
print("=== Nested Conditions ===")

temperature = 25
humidity = 70

if (temperature > 30) {
    if (humidity > 60) {
        print("Hot and humid!")
    } else {
        print("Hot but dry")
    }
} elif (temperature > 20) {
    if (humidity > 60) {
        print("Warm and humid")
    } else {
        print("Comfortable weather")
    }
} else {
    print("Cold weather")
}
print("")

// ===== RECURSIVE FACTORIAL =====
print("=== Factorial (Recursive) ===")

fn factorial(n) {
    if (n <= 1) {
        return 1
    }
    return n * factorial(n - 1)
}

print("5! = " + factorial(5))
print("7! = " + factorial(7))
print("")

// ===== COMPLEX LOGIC =====
print("=== Complex Logic ===")

fn isPrime(n) {
    if (n < 2) {
        return false
    }
    i = 2
    while (i * i <= n) {
        if (n % i == 0) {
            return false
        }
        i = i + 1
    }
    return true
}

print("Checking primes:")
for (num in 2..10) {
    if (isPrime(num)) {
        print("  " + num + " is prime")
    }
}
print("")

// ===== SUMMARY =====
print("╔════════════════════════════════════════════════╗")
print("║  ✅ Variables without let/var                  ║")
print("║  ✅ if/elif/else with parentheses              ║")
print("║  ✅ while loops with parentheses               ║")
print("║  ✅ for-in loops                               ║")
print("║  ✅ try/catch/finally                          ║")
print("║  ✅ Reactive variables                         ║")
print("║  ✅ Time-travel debugging                      ║")
print("║                                                ║")
print("║  A-lang: JavaScript simplicity                 ║")
print("║          + Rust performance                    ║")
print("║          + Revolutionary features! 🚀          ║")
print("╚════════════════════════════════════════════════╝")
