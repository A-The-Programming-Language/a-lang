# A-lang Syntax Guide

Complete reference for A-lang syntax and features.

**Version**: 1.0-preview  
**Last Updated**: December 2024

---

## Table of Contents

1. [Basic Syntax](#basic-syntax)
2. [Data Types](#data-types)
3. [Variables](#variables)
4. [Operators](#operators)
5. [Control Flow](#control-flow)
6. [Functions](#functions)
7. [Arrays](#arrays)
8. [Objects](#objects)
9. [Strings](#strings)
10. [Input/Output](#inputoutput)
11. [Time-Travel Debugging](#time-travel-debugging)
12. [Reactive Variables](#reactive-variables)
13. [FFI (Foreign Function Interface)](#ffi-foreign-function-interface)
14. [Error Handling](#error-handling)
15. [Built-in Functions](#built-in-functions)

---

## Basic Syntax

### Comments

```javascript
// Single-line comment

/* 
   Multi-line
   comment
*/
```

### Semicolons

Semicolons are optional (like JavaScript):

```javascript
x = 5
y = 10

// or
x = 5;
y = 10;
```

---

## Data Types

A-lang has 7 primitive types:

```javascript
// Integer
age = 42

// Float
pi = 3.14159

// String
name = "Alice"
message = 'Hello'

// Boolean
active = true
disabled = false

// Nil (null/undefined)
empty = nil

// Array
numbers = [1, 2, 3, 4, 5]

// Object
person = {
    name: "Bob",
    age: 30,
    city: "NYC"
}
```

---

## Variables

### Declaration

Variables are dynamically typed (no `let`, `var`, or `const` needed):

```javascript
x = 10
name = "A-lang"
active = true
```

### Type Conversion

```javascript
// String to number
num = int("42")        // 42
pi = float("3.14")     // 3.14

// Number to string
str(123)               // "123"
str(3.14)              // "3.14"

// Check type
type_of(42)            // "integer"
type_of("hello")       // "string"
type_of(true)          // "boolean"
type_of([1, 2])        // "array"
```

---

## Operators

### Arithmetic

```javascript
a + b    // Addition
a - b    // Subtraction
a * b    // Multiplication
a / b    // Division
a % b    // Modulo

a++      // Increment
a--      // Decrement
a += 5   // Add and assign
a -= 5   // Subtract and assign
a *= 2   // Multiply and assign
a /= 2   // Divide and assign
```

### Comparison

```javascript
a == b   // Equal
a != b   // Not equal
a > b    // Greater than
a < b    // Less than
a >= b   // Greater or equal
a <= b   // Less or equal
```

### Logical

```javascript
a && b   // AND
a || b   // OR
!a       // NOT
```

### Bitwise

```javascript
a & b    // Bitwise AND
a | b    // Bitwise OR
a ^ b    // Bitwise XOR
~a       // Bitwise NOT
a << 2   // Left shift
a >> 2   // Right shift
```

---

## Control Flow

### If/Else

**Note**: Parentheses around conditions are **required**:

```javascript
if (x > 10) {
    print("x is large")
} elif (x > 5) {
    print("x is medium")
} else {
    print("x is small")
}

// Single line
if (x > 0) { print("positive") }
```

### While Loop

```javascript
count = 0
while (count < 10) {
    print(count)
    count++
}
```

### For Loop

```javascript
// For-in loop
for (i in range(10)) {
    print(i)  // 0, 1, 2, ..., 9
}

// Iterate array
numbers = [1, 2, 3, 4, 5]
for (num in numbers) {
    print(num)
}

// Range with start and end
for (i in range(1, 6)) {
    print(i)  // 1, 2, 3, 4, 5
}
```

### Break and Continue

```javascript
for (i in range(10)) {
    if (i == 5) {
        break  // Exit loop
    }
    if (i % 2 == 0) {
        continue  // Skip to next iteration
    }
    print(i)
}
```

---

## Functions

### Function Declaration

```javascript
fn greet(name) {
    return "Hello, " + name + "!"
}

result = greet("Alice")
print(result)  // "Hello, Alice!"
```

### Arrow Functions

```javascript
// Single parameter
double = (x) => x * 2

// Multiple parameters
add = (a, b) => a + b

// With block
multiply = (a, b) => {
    result = a * b
    return result
}
```

### Default Parameters

```javascript
fn greet(name, greeting) {
    if (greeting == nil) {
        greeting = "Hello"
    }
    return greeting + ", " + name
}
```

### Closures

```javascript
fn makeCounter() {
    count = 0
    return fn() {
        count++
        return count
    }
}

counter = makeCounter()
print(counter())  // 1
print(counter())  // 2
print(counter())  // 3
```

---

## Arrays

### Creating Arrays

```javascript
empty = []
numbers = [1, 2, 3, 4, 5]
mixed = [1, "two", 3.0, true, [5, 6]]
```

### Accessing Elements

```javascript
arr = [10, 20, 30, 40]
print(arr[0])   // 10
print(arr[2])   // 30

arr[1] = 25     // Modify
```

### Array Operations

```javascript
arr = [1, 2, 3]

// Add element
arr = push(arr, 4)        // [1, 2, 3, 4]

// Remove last
arr = pop(arr)            // [1, 2, 3]

// Length
len(arr)                  // 3

// Slice
slice(arr, 0, 2)          // [1, 2]

// Index of element
indexOf(arr, 2)           // 1

// Check if contains
includes(arr, 2)          // true

// Join to string
join(arr, ", ")           // "1, 2, 3"
```

### Range

```javascript
range(5)           // [0, 1, 2, 3, 4]
range(1, 6)        // [1, 2, 3, 4, 5]
range(0, 10, 2)    // [0, 2, 4, 6, 8]
```

---

## Objects

### Creating Objects

```javascript
person = {
    name: "Alice",
    age: 30,
    city: "NYC"
}
```

### Accessing Properties

```javascript
// Dot notation
print(person.name)     // "Alice"

// Bracket notation
print(person["age"])   // 30

// Modify
person.age = 31
person["city"] = "LA"
```

### Object Operations

```javascript
person = {
    name: "Bob",
    age: 25
}

// Get keys
keys(person)           // ["name", "age"]

// Get values
values(person)         // ["Bob", 25]

// Number of properties
len(person)            // 2
```

---

## Strings

### String Operations

```javascript
text = "Hello, World!"

// Length
len(text)                         // 13

// Concatenation
"Hello" + " " + "World"           // "Hello World"

// Split
split(text, ", ")                 // ["Hello", "World!"]

// Join
join(["A", "B", "C"], "-")        // "A-B-C"

// Case conversion
toUpperCase("hello")              // "HELLO"
toLowerCase("WORLD")              // "world"

// Trim whitespace
trim("  hello  ")                 // "hello"

// Replace
replace("Hello", "l", "L")        // "HeLlo"

// Substring/Slice
text[0]                           // "H"
slice(text, 0, 5)                 // "Hello"
```

---

## Input/Output

### Output

```javascript
// Print to console
print("Hello, World!")
print(42)
print(x, y, z)  // Multiple values
```

### Input

```javascript
// Read user input (Python-style)
name = input("What is your name? ")
print("Hello, " + name)

// Read and convert
age = int(input("Your age: "))
if (age >= 18) {
    print("Adult")
}
```

### File I/O

```javascript
// Read entire file
content = readFile("data.txt")

// Read lines
lines = readLines("data.txt")
for (line in lines) {
    print(line)
}

// Write file
writeFile("output.txt", "Hello, World!")

// Append to file
appendFile("log.txt", "New log entry\n")

// Check if file exists
if (fileExists("config.json")) {
    config = readFile("config.json")
}

// Delete file
deleteFile("temp.txt")
```

---

## Time-Travel Debugging

Take snapshots and rewind execution:

```javascript
// Take snapshot
x = 10
snapshot("before")

x = x * 2
snapshot("after")

// Rewind to previous snapshot
rewind("before")
print(x)  // 10 (back in time!)

// Rewind by number of steps
rewind(1)

// Multiple snapshots
for (i in range(5)) {
    total += i
    snapshot("step_" + str(i))
}

// Go back to specific step
rewind("step_2")
```

### Checkpoint Example

```javascript
fn calculate(n) {
    result = 0
    checkpoint("start")
    
    for (i in range(n)) {
        result += i
        snapshot("iteration_" + str(i))
    }
    
    return result
}

value = calculate(100)

// Jump back to checkpoint
rewind("start")
```

---

## Reactive Variables

Variables that automatically update when dependencies change:

```javascript
// Create reactive variable
reactive count = 0

// Computed value (auto-updates)
computed double = () => count * 2
computed squared = () => count * count

// Effect (runs on changes)
effect () => {
    print("Count changed: " + str(count))
    print("Double: " + str(double))
}

// Update reactive variable
count = 5
// Automatically prints:
// Count changed: 5
// Double: 10

print(squared)  // 25
```

### Complex Reactive Example

```javascript
reactive temperature = 20
reactive humidity = 50

computed feelLike = () => {
    if (temperature > 30 && humidity > 70) {
        return "Very Hot"
    } elif (temperature < 10) {
        return "Cold"
    } else {
        return "Comfortable"
    }
}

effect () => {
    print("Temperature: " + str(temperature) + "°C")
    print("Feels like: " + feelLike)
}

temperature = 35
humidity = 80
// Automatically updates and prints
```

---

## FFI (Foreign Function Interface)

Call C functions directly from A-lang:

### Load Library

```javascript
// Linux
ffiLoadLibrary("libc", "/lib/x86_64-linux-gnu/libc.so.6")

// macOS
ffiLoadLibrary("libc", "/usr/lib/libSystem.dylib")
```

### Register Function Signature

```javascript
// int abs(int x)
ffiRegisterFunction("abs", "int", ["int"])

// size_t strlen(const char *s)
ffiRegisterFunction("strlen", "int", ["string"])

// double sqrt(double x)
ffiRegisterFunction("sqrt", "double", ["double"])

// double pow(double x, double y)
ffiRegisterFunction("pow", "double", ["double", "double"])
```

### Call C Function

```javascript
result = ffiCall("libc", "abs", [-42])
print(result)  // 42

length = ffiCall("libc", "strlen", ["Hello"])
print(length)  // 5

root = ffiCall("libm", "sqrt", [16.0])
print(root)  // 4.0
```

### Supported Types

| A-lang Type | C Type | Description |
|-------------|--------|-------------|
| `"int"` | `int` | 32-bit integer |
| `"long"` | `long` | 64-bit integer |
| `"float"` | `float` | 32-bit float |
| `"double"` | `double` | 64-bit float |
| `"string"` | `char*` | Null-terminated string |
| `"bool"` | `bool` | Boolean |
| `"void"` | `void` | No return value |
| `"pointer"` | `void*` | Generic pointer |

### Complete FFI Example

```javascript
// Load math library
ffiLoadLibrary("libm", "/lib/x86_64-linux-gnu/libm.so.6")

// Register functions
ffiRegisterFunction("sqrt", "double", ["double"])
ffiRegisterFunction("pow", "double", ["double", "double"])
ffiRegisterFunction("sin", "double", ["double"])
ffiRegisterFunction("cos", "double", ["double"])

// Use them
print(ffiCall("libm", "sqrt", [144.0]))      // 12.0
print(ffiCall("libm", "pow", [2.0, 8.0]))    // 256.0
print(ffiCall("libm", "sin", [0.0]))         // 0.0
print(ffiCall("libm", "cos", [0.0]))         // 1.0
```

---

## Error Handling

### Try/Catch

```javascript
try {
    result = riskyOperation()
    print("Success: " + str(result))
} catch (error) {
    print("Error: " + str(error))
}
```

### Throw

```javascript
fn divide(a, b) {
    if (b == 0) {
        throw "Division by zero"
    }
    return a / b
}

try {
    result = divide(10, 0)
} catch (e) {
    print("Caught: " + str(e))
}
```

---

## Built-in Functions

### Math Functions

```javascript
abs(-5)           // 5
min(1, 2, 3)      // 1
max(1, 2, 3)      // 3
floor(3.7)        // 3
ceil(3.2)         // 4
round(3.5)        // 4
```

### String Functions

```javascript
len("hello")                    // 5
split("a,b,c", ",")            // ["a", "b", "c"]
join(["a", "b"], ",")          // "a,b"
toUpperCase("hello")           // "HELLO"
toLowerCase("WORLD")           // "world"
trim("  text  ")               // "text"
replace("hello", "l", "L")     // "heLLo"
```

### Array Functions

```javascript
len([1, 2, 3])                 // 3
push([1, 2], 3)                // [1, 2, 3]
pop([1, 2, 3])                 // [1, 2]
slice([1,2,3,4], 1, 3)         // [2, 3]
indexOf([1,2,3], 2)            // 1
includes([1,2,3], 2)           // true
range(5)                       // [0, 1, 2, 3, 4]
```

### Type Functions

```javascript
type_of(42)        // "integer"
int("42")          // 42
float("3.14")      // 3.14
str(123)           // "123"
```

### Object Functions

```javascript
keys({a: 1, b: 2})       // ["a", "b"]
values({a: 1, b: 2})     // [1, 2]
```

### System Functions

```javascript
sleep(1000)        // Sleep 1 second (milliseconds)
timestamp()        // Unix timestamp
exit(0)            // Exit program
```

---

## Examples

### Hello World

```javascript
print("Hello, World!")
```

### Fibonacci

```javascript
fn fibonacci(n) {
    if (n <= 1) {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

print(fibonacci(10))  // 55
```

### Interactive Calculator

```javascript
print("Simple Calculator")
num1 = float(input("First number: "))
op = input("Operator (+, -, *, /): ")
num2 = float(input("Second number: "))

if (op == "+") {
    print("Result: " + str(num1 + num2))
} elif (op == "-") {
    print("Result: " + str(num1 - num2))
} elif (op == "*") {
    print("Result: " + str(num1 * num2))
} elif (op == "/") {
    if (num2 != 0) {
        print("Result: " + str(num1 / num2))
    } else {
        print("Error: Division by zero")
    }
}
```

### Counter with Time-Travel

```javascript
count = 0
snapshot("start")

for (i in range(5)) {
    count += i
    snapshot("step_" + str(i))
    print("Count: " + str(count))
}

print("\nRewinding to step 2...")
rewind("step_2")
print("Count is now: " + str(count))
```

### Reactive Temperature Monitor

```javascript
reactive temp = 20

computed status = () => {
    if (temp > 30) { return "Hot" }
    elif (temp < 10) { return "Cold" }
    else { return "Comfortable" }
}

effect () => {
    print("Temperature: " + str(temp) + "°C - " + status)
}

temp = 35  // Prints: Temperature: 35°C - Hot
temp = 5   // Prints: Temperature: 5°C - Cold
```

### FFI Math Operations

```javascript
ffiLoadLibrary("libm", "/lib/x86_64-linux-gnu/libm.so.6")

ffiRegisterFunction("sqrt", "double", ["double"])
ffiRegisterFunction("pow", "double", ["double", "double"])

numbers = [4.0, 9.0, 16.0, 25.0]
print("Square roots:")
for (num in numbers) {
    root = ffiCall("libm", "sqrt", [num])
    print("  sqrt(" + str(num) + ") = " + str(root))
}

print("\nPowers of 2:")
for (i in range(1, 11)) {
    power = ffiCall("libm", "pow", [2.0, float(i)])
    print("  2^" + str(i) + " = " + str(power))
}
```

---

## Style Guide

### Naming Conventions

```javascript
// Variables and functions: camelCase
userName = "Alice"
fn getUserName() { ... }

// Constants: UPPER_CASE (by convention)
MAX_SIZE = 100
API_KEY = "secret"
```

### Indentation

Use 4 spaces or 2 spaces consistently:

```javascript
fn example() {
    if (condition) {
        doSomething()
    }
}
```

### Spacing

```javascript
// Good
x = 5 + 3
if (x > 10) { ... }

// Avoid
x=5+3
if(x>10){ ... }
```

---

## Differences from JavaScript

1. **Parentheses required in conditions**:
   - A-lang: `if (x > 5) { ... }`
   - JavaScript: `if (x > 5) { ... }` or `if x > 5 { ... }` (not allowed in A-lang)

2. **No `let`, `var`, or `const`**:
   - A-lang: `x = 5`
   - JavaScript: `let x = 5`

3. **`elif` instead of `else if`**:
   - A-lang: `elif (x > 5) { ... }`
   - JavaScript: `else if (x > 5) { ... }`

4. **`fn` keyword for functions**:
   - A-lang: `fn add(a, b) { ... }`
   - JavaScript: `function add(a, b) { ... }`

5. **`nil` instead of `null`/`undefined`**:
   - A-lang: `x = nil`
   - JavaScript: `x = null` or `x = undefined`

---

## Resources

- **GitHub**: https://github.com/A-The-Programming-Language/a-lang
- **Documentation**: See `docs/` directory
- **Examples**: See `examples/` directory
- **Contributing**: See `CONTRIBUTING.md`

---

**Version**: 1.0-preview  
**License**: MIT or Apache 2.0  
**Last Updated**: December 2024