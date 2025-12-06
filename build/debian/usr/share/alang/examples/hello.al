// Hello World in A-lang
// JavaScript-style syntax - simple and familiar!

// Variables - no let/var needed, just assign
message = "Hello, A-lang!"
print(message)

// You can still use const for constants
const VERSION = "1.0.0"
print("Version: " + VERSION)

// Basic arithmetic
x = 10
y = 20
sum = x + y
print("The sum of " + x + " and " + y + " is " + sum)

// Functions - clean and simple
fn greet(name) {
    return "Hello, " + name + "!"
}

print(greet("World"))
print(greet("Developer"))

// Arrays
languages = ["Rust", "Python", "JavaScript", "A-lang"]
print("My favorite language is: " + languages[3])

// If-elif-else with parentheses (JS style)
score = 85
if (score >= 90) {
    print("Grade: A")
} elif (score >= 80) {
    print("Grade: B")
} elif (score >= 70) {
    print("Grade: C")
} else {
    print("Grade: D")
}

// While loop with parentheses
print("\nCountdown:")
count = 3
while (count > 0) {
    print(count)
    count = count - 1
}
print("Liftoff!")

// For-in loop (iterator style)
print("\nIterating array:")
for (lang in languages) {
    print("  - " + lang)
}

// Range iteration
print("\nRange 1..6:")
for (i in 1..6) {
    print(i)
}

// Try-catch-finally (JS style)
print("\nTry-catch example:")
try {
    x = 10
    y = 0
    if (y == 0) {
        throw "Division by zero!"
    }
    result = x / y
} catch (error) {
    print("Error caught: " + error)
} finally {
    print("Cleanup completed")
}

// Factorial function
fn factorial(n) {
    if (n <= 1) {
        return 1
    }
    return n * factorial(n - 1)
}

print("\nFactorial of 5: " + factorial(5))

// Objects
person = {
    name: "Alice",
    age: 30,
    city: "Tech City"
}

print("\nPerson: " + person.name + ", age " + person.age)

// Nested conditions
temperature = 25
if (temperature > 30) {
    print("\nIt's hot!")
} elif (temperature > 20) {
    print("\nIt's warm!")
} else {
    print("\nIt's cold!")
}

// Multiple operations in sequence
print("\nCalculating...")
a = 5
b = 10
c = a + b
d = c * 2
print("Result: " + d)

print("\n✨ A-lang: Simple. Fast. Powerful. 🚀")
