// A-lang: Operators & Features Demo
// Testing compound operators, lambdas, and standard library

print("=== A-lang New Features Demo ===")
print("")

// Compound Assignment Operators
print("1. Compound Operators")
x = 10
print("x = " + x)
x += 5
print("x += 5 -> " + x)
x -= 3
print("x -= 3 -> " + x)
x *= 2
print("x *= 2 -> " + x)
x /= 4
print("x /= 4 -> " + x)
print("")

// Increment and Decrement
print("2. Increment/Decrement")
counter = 0
print("counter = " + counter)
counter++
print("counter++ -> " + counter)
counter++
print("counter++ -> " + counter)
counter--
print("counter-- -> " + counter)
print("")

// Arrow Functions (Lambdas)
print("3. Arrow Functions")
double = x => x * 2
print("double(5) = " + double(5))

square = x => x * x
print("square(4) = " + square(4))

add = (a, b) => a + b
print("add(3, 7) = " + add(3, 7))

multiply = (x, y) => x * y
print("multiply(6, 7) = " + multiply(6, 7))
print("")

// Math Functions
print("4. Math Functions")
print("abs(-15) = " + abs(-15))
print("min(5, 3, 9, 1) = " + min(5, 3, 9, 1))
print("max(5, 3, 9, 1) = " + max(5, 3, 9, 1))
print("floor(3.7) = " + floor(3.7))
print("ceil(3.2) = " + ceil(3.2))
print("round(3.5) = " + round(3.5))
print("")

// Type Conversion
print("5. Type Conversion")
print("int('42') = " + int("42"))
print("float('3.14') = " + float("3.14"))
print("str(123) = " + str(123))
print("")

// String Operations
print("6. String Operations")
text = "apple,banana,cherry"
parts = split(text, ",")
print("split result:")
for (part in parts) {
    print("  - " + part)
}

words = ["Hello", "World"]
joined = join(words, " ")
print("join result: " + joined)
print("")

// Array Operations
print("7. Array Operations")
arr = [1, 2, 3]
print("Original: [1, 2, 3]")
arr2 = push(arr, 4)
print("After push(4): length = " + len(arr2))
arr3 = pop(arr2)
print("After pop: length = " + len(arr3))
print("")

// Object Operations
print("8. Object Operations")
person = {
    name: "Alice",
    age: 30
}
objKeys = keys(person)
print("Keys: " + objKeys[0] + ", " + objKeys[1])
print("")

// Range Function
print("9. Range Function")
r = range(5)
print("range(5): " + r[0] + ", " + r[1] + ", " + r[2] + ", " + r[3] + ", " + r[4])
print("")

// Combining Features
print("10. Combining Everything")
sum = 0
numbers = range(1, 6)
for (n in numbers) {
    sum += n
}
print("Sum of range(1, 6): " + sum)

fn factorial(n) {
    result = 1
    i = 1
    while (i <= n) {
        result *= i
        i++
    }
    return result
}
print("factorial(5) = " + factorial(5))
print("")

print("=== All Features Working! ===")
