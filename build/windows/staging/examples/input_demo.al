// ========================================
// A-lang Input Demo
// Demonstrates the input() function
// ========================================

print("╔════════════════════════════════════════╗")
print("║     A-lang Input Demo (Python-style)   ║")
print("╚════════════════════════════════════════╝")
print("")

// ========================================
// Example 1: Basic Input
// ========================================
print("=== Example 1: Basic Input ===")
print("")

name = input("What is your name? ")
print("Hello, " + name + "!")
print("")

// ========================================
// Example 2: Numeric Input
// ========================================
print("=== Example 2: Numeric Input ===")
print("")

age_str = input("How old are you? ")
age = int(age_str)

if (age >= 18) {
    print("You are an adult (" + str(age) + " years old)")
} else {
    print("You are a minor (" + str(age) + " years old)")
}
print("")

// ========================================
// Example 3: Calculator
// ========================================
print("=== Example 3: Simple Calculator ===")
print("")

num1_str = input("Enter first number: ")
num2_str = input("Enter second number: ")

num1 = float(num1_str)
num2 = float(num2_str)

print("Results:")
print("  " + str(num1) + " + " + str(num2) + " = " + str(num1 + num2))
print("  " + str(num1) + " - " + str(num2) + " = " + str(num1 - num2))
print("  " + str(num1) + " * " + str(num2) + " = " + str(num1 * num2))
if (num2 != 0) {
    print("  " + str(num1) + " / " + str(num2) + " = " + str(num1 / num2))
}
print("")

// ========================================
// Example 4: Menu System
// ========================================
print("=== Example 4: Interactive Menu ===")
print("")
print("Choose an option:")
print("  1. Greet me")
print("  2. Tell me a joke")
print("  3. Show system info")
print("  4. Exit")
print("")

choice = input("Your choice (1-4): ")

if (choice == "1") {
    user = input("What's your name? ")
    print("👋 Hello, " + user + "! Nice to meet you!")
} elif (choice == "2") {
    print("🤣 Why do programmers prefer dark mode?")
    input("Press Enter for the answer...")
    print("Because light attracts bugs! 🐛")
} elif (choice == "3") {
    print("📊 System Information:")
    print("  Language: A-lang v1.0-preview")
    print("  Platform: Rust-powered")
    print("  Features: Time-travel, Reactive, FFI")
} elif (choice == "4") {
    print("👋 Goodbye!")
} else {
    print("❌ Invalid choice!")
}
print("")

// ========================================
// Example 5: Data Collection
// ========================================
print("=== Example 5: User Registration ===")
print("")

username = input("Username: ")
email = input("Email: ")
city = input("City: ")

print("")
print("✅ Registration Summary:")
print("  Username: " + username)
print("  Email: " + email)
print("  City: " + city)
print("")

confirm = input("Is this correct? (yes/no): ")
if (confirm == "yes" || confirm == "y") {
    print("✓ Registration complete!")
} else {
    print("✗ Registration cancelled")
}
print("")

// ========================================
// Example 6: Guess the Number Game
// ========================================
print("=== Example 6: Guess the Number Game ===")
print("")

secret = 42
print("I'm thinking of a number between 1 and 100...")
print("")

attempts = 0
found = false

while (attempts < 5 && !found) {
    guess_str = input("Your guess: ")
    guess = int(guess_str)
    attempts++

    if (guess == secret) {
        print("🎉 Congratulations! You found it in " + str(attempts) + " attempts!")
        found = true
    } elif (guess < secret) {
        print("📈 Too low! Try again.")
    } else {
        print("📉 Too high! Try again.")
    }
}

if (!found) {
    print("😞 Game over! The number was " + str(secret))
}
print("")

// ========================================
// Example 7: Multi-line Input
// ========================================
print("=== Example 7: Multi-line Input ===")
print("")

print("Enter three favorite programming languages:")
lang1 = input("  1. ")
lang2 = input("  2. ")
lang3 = input("  3. ")

languages = [lang1, lang2, lang3]
print("")
print("Your top 3 languages: " + join(languages, ", "))
print("")

// ========================================
// Final Message
// ========================================
print("╔════════════════════════════════════════╗")
print("║     ✓ Input Demo Complete!             ║")
print("╚════════════════════════════════════════╝")
