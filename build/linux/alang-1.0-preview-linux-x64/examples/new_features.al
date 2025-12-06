// A-lang: New Features Demo
// Showcasing: Operators, Lambdas, and Standard Library

print("╔════════════════════════════════════════════════╗")
print("║  A-lang: New Features Demo                     ║")
print("╚════════════════════════════════════════════════╝")
print("")

// ===== COMPOUND ASSIGNMENT OPERATORS =====
print("=== Compound Assignment Operators ===")

x = 10
print("x = " + x)

x += 5
print("x += 5  -> " + x)

x -= 3
print("x -= 3  -> " + x)

x *= 2
print("x *= 2  -> " + x)

x /= 4
print("x /= 4  -> " + x)

x %= 5
print("x %= 5  -> " + x)
print("")

// ===== INCREMENT/DECREMENT =====
print("=== Increment/Decrement ===")

counter = 0
print("counter = " + counter)

counter++
print("counter++ -> " + counter)

counter++
print("counter++ -> " + counter)

counter--
print("counter-- -> " + counter)

++counter
print("++counter -> " + counter)

--counter
print("--counter -> " + counter)
print("")

// ===== ARROW FUNCTIONS / LAMBDAS =====
print("=== Arrow Functions (Lambdas) ===")

// Single parameter
double = x => x * 2
print("double(5) = " + double(5))

square = x => x * x
print("square(4) = " + square(4))

// Multiple parameters
add = (a, b) => a + b
print("add(3, 7) = " + add(3, 7))

multiply = (x, y) => x * y
print("multiply(6, 7) = " + multiply(6, 7))

// More complex lambda
isEven = n => n % 2 == 0
print("isEven(4) = " + isEven(4))
print("isEven(7) = " + isEven(7))
print("")

// ===== STANDARD LIBRARY - MATH =====
print("=== Standard Library: Math Functions ===")

print("abs(-15) = " + abs(-15))
print("abs(15) = " + abs(15))

print("min(5, 3, 9, 1) = " + min(5, 3, 9, 1))
print("max(5, 3, 9, 1) = " + max(5, 3, 9, 1))

print("floor(3.7) = " + floor(3.7))
print("ceil(3.2) = " + ceil(3.2))
print("round(3.5) = " + round(3.5))
print("")

// ===== STANDARD LIBRARY - TYPE CONVERSION =====
print("=== Standard Library: Type Conversion ===")

numStr = "42"
print('str: "42" -> int: ' + int(numStr))

floatStr = "3.14"
print('str: "3.14" -> float: ' + float(floatStr))

num = 123
print("int: 123 -> str: " + str(num))
print("")

// ===== STANDARD LIBRARY - STRING OPERATIONS =====
print("=== Standard Library: String Operations ===")

text = "apple,banana,cherry"
parts = split(text, ",")
print("split('" + text + "', ',') = ")
for (part in parts) {
    print("  - " + part)
}

words = ["Hello", "World", "from", "A-lang"]
joined = join(words, " ")
print("join(['Hello', 'World', ...], ' ') = '" + joined + "'")
print("")

// ===== STANDARD LIBRARY - ARRAY OPERATIONS =====
print("=== Standard Library: Array Operations ===")

arr = [1, 2, 3]
print("Original array: [1, 2, 3]")

arr2 = push(arr, 4)
print("push(arr, 4) = " + arr2[0] + ", " + arr2[1] + ", " + arr2[2] + ", " + arr2[3])

arr3 = pop(arr2)
print("pop(arr2) = " + arr3[0] + ", " + arr3[1] + ", " + arr3[2])

print("len(arr3) = " + len(arr3))
print("")

// ===== STANDARD LIBRARY - OBJECT OPERATIONS =====
print("=== Standard Library: Object Operations ===")

person = {
    name: "Alice",
    age: 30,
    city: "TechCity"
}

objKeys = keys(person)
print("keys(person):")
for (key in objKeys) {
    print("  - " + key)
}

objValues = values(person)
print("values(person):")
for (val in objValues) {
    print("  - " + val)
}
print("")

// ===== STANDARD LIBRARY - RANGE =====
print("=== Standard Library: Range ===")

r1 = range(5)
print("range(5) = " + r1[0] + ", " + r1[1] + ", " + r1[2] + ", " + r1[3] + ", " + r1[4])

r2 = range(3, 8)
print("range(3, 8) = " + r2[0] + ", " + r2[1] + ", " + r2[2] + ", " + r2[3] + ", " + r2[4])
print("")

// ===== COMBINING EVERYTHING =====
print("=== Combining Everything ===")

// Using lambdas with compound operators
factorial = n => {
    result = 1
    i = 1
    while (i <= n) {
        result *= i
        i++
    }
    return result
}

print("factorial(5) = " + factorial(5))
print("factorial(7) = " + factorial(7))

// Processing array with lambdas
numbers = range(1, 6)
print("\nNumbers: " + numbers[0] + ", " + numbers[1] + ", " + numbers[2] + ", " + numbers[3] + ", " + numbers[4])

// Manual map using lambda
doubled = []
for (n in numbers) {
    doubled = push(doubled, double(n))
}
print("Doubled: " + doubled[0] + ", " + doubled[1] + ", " + doubled[2] + ", " + doubled[3] + ", " + doubled[4])

// Sum with compound operator
sum = 0
for (n in numbers) {
    sum += n
}
print("Sum: " + sum)

// Product with compound operator
product = 1
for (n in numbers) {
    product *= n
}
print("Product: " + product)
print("")

// ===== FUN EXAMPLE: CALCULATOR =====
print("=== Calculator Example ===")

calc = (op, a, b) => {
    if (op == "+") {
        return a + b
    } elif (op == "-") {
        return a - b
    } elif (op == "*") {
        return a * b
    } elif (op == "/") {
        return a / b
    } else {
        return "Unknown operation"
    }
}

print("calc('+', 10, 5) = " + calc("+", 10, 5))
print("calc('-', 10, 5) = " + calc("-", 10, 5))
print("calc('*', 10, 5) = " + calc("*", 10, 5))
print("calc('/', 10, 5) = " + calc("/", 10, 5))
print("")

// ===== SUMMARY =====
print("╔════════════════════════════════════════════════╗")
print("║  ✅ Compound operators (+=, -=, *=, /=, %=)    ║")
print("║  ✅ Increment/decrement (++, --)               ║")
print("║  ✅ Arrow functions (x => expr)                ║")
print("║  ✅ Math functions (abs, min, max, floor...)   ║")
print("║  ✅ Type conversion (int, float, str)          ║")
print("║  ✅ String operations (split, join)            ║")
print("║  ✅ Array operations (push, pop, len)          ║")
print("║  ✅ Object operations (keys, values)           ║")
print("║  ✅ Range function                             ║")
print("║                                                ║")
print("║  A-lang: Now with even more power! 💪         ║")
print("╚════════════════════════════════════════════════╝")
