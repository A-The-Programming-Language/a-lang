// A-lang: Critical Features Demo
// Showcasing Template Strings, Map/Filter/Reduce, and Enhanced Stdlib

print("╔═════════════════════════════════════════════════╗")
print("║  A-lang: Critical Features Demo                 ║")
print("║  Template Strings + Map/Filter/Reduce + More    ║")
print("╚═════════════════════════════════════════════════╝")
print("")

// ===== 1. TEMPLATE STRINGS =====
print("=== 1. Template Strings (Interpolation) ===")

name = "Bob"
age = 25
city = "TechCity"

// Simple interpolation
greeting = `Hello, ${name}!`
print(greeting)

// Multiple interpolations
info = `${name} is ${age} years old and lives in ${city}`
print(info)

// Expressions in interpolation
x = 10
y = 20
result = `${x} + ${y} = ${x + y}`
print(result)

// Complex expressions
price = 99.99
discount = 0.2
final = `Price: $${price}, Discount: ${discount * 100}%, Final: $${price * (1 - discount)}`
print(final)
print("")

// ===== 2. MAP FUNCTION =====
print("=== 2. Map Function ===")

numbers = [1, 2, 3, 4, 5]
print(`Original: [${numbers[0]}, ${numbers[1]}, ${numbers[2]}, ${numbers[3]}, ${numbers[4]}]`)

// Double each number
doubled = map(numbers, x => x * 2)
print(`Doubled: [${doubled[0]}, ${doubled[1]}, ${doubled[2]}, ${doubled[3]}, ${doubled[4]}]`)

// Square each number
squared = map(numbers, x => x * x)
print(`Squared: [${squared[0]}, ${squared[1]}, ${squared[2]}, ${squared[3]}, ${squared[4]}]`)

// Convert to strings
words = ["hello", "world", "from", "alang"]
uppercase = map(words, w => toUpperCase(w))
print(`Uppercase: ${uppercase[0]}, ${uppercase[1]}, ${uppercase[2]}, ${uppercase[3]}`)
print("")

// ===== 3. FILTER FUNCTION =====
print("=== 3. Filter Function ===")

nums = range(1, 11)
print(`Numbers 1-10: [${nums[0]}, ${nums[1]}, ${nums[2]}...${nums[9]}]`)

// Filter even numbers
evens = filter(nums, x => x % 2 == 0)
print(`Even numbers: [${evens[0]}, ${evens[1]}, ${evens[2]}, ${evens[3]}, ${evens[4]}]`)

// Filter numbers greater than 5
bigNums = filter(nums, x => x > 5)
print(`Numbers > 5: [${bigNums[0]}, ${bigNums[1]}, ${bigNums[2]}, ${bigNums[3]}, ${bigNums[4]}]`)

// Filter using function
isEven = x => x % 2 == 0
onlyEvens = filter(range(1, 21), isEven)
print(`Evens 1-20: length = ${len(onlyEvens)}`)
print("")

// ===== 4. REDUCE FUNCTION =====
print("=== 4. Reduce Function ===")

values = [1, 2, 3, 4, 5]
print(`Values: [${values[0]}, ${values[1]}, ${values[2]}, ${values[3]}, ${values[4]}]`)

// Sum all numbers
sum = reduce(values, (a, b) => a + b, 0)
print(`Sum: ${sum}`)

// Product of all numbers
product = reduce(values, (a, b) => a * b, 1)
print(`Product: ${product}`)

// Find maximum using max function
max_val = max(values[0], values[1], values[2], values[3], values[4])
print(`Max: ${max_val}`)

// Concatenate strings
strs = ["A", "lang", "is", "awesome"]
concatenated = reduce(strs, (a, b) => a + " " + b)
print(`Concatenated: "${concatenated}"`)
print("")

// ===== 5. CHAINING OPERATIONS =====
print("=== 5. Chaining Map/Filter/Reduce ===")

data = range(1, 11)
print(`Data: ${data[0]}-${data[9]}`)

// Chain: filter evens, then double, then sum
step1 = filter(data, x => x % 2 == 0)
step2 = map(step1, x => x * 2)
result = reduce(step2, (a, b) => a + b, 0)

print(`Even numbers: [${step1[0]}, ${step1[1]}, ${step1[2]}, ${step1[3]}, ${step1[4]}]`)
print(`Doubled: [${step2[0]}, ${step2[1]}, ${step2[2]}, ${step2[3]}, ${step2[4]}]`)
print(`Sum of doubled evens: ${result}`)
print("")

// ===== 6. STRING FUNCTIONS =====
print("=== 6. Enhanced String Functions ===")

text = "  Hello World  "
print(`Original: "${text}"`)
print(`Trimmed: "${trim(text)}"`)
print(`Upper: "${toUpperCase(text)}"`)
print(`Lower: "${toLowerCase(text)}"`)

sentence = "The quick brown fox"
print(`Original: "${sentence}"`)
print(`Replaced: "${replace(sentence, "fox", "cat")}"`)

csv = "apple,banana,cherry"
fruits = split(csv, ",")
print(`Split CSV: ${fruits[0]}, ${fruits[1]}, ${fruits[2]}`)

joined = join(fruits, " | ")
print(`Joined: "${joined}"`)
print("")

// ===== 7. ARRAY FUNCTIONS =====
print("=== 7. Enhanced Array Functions ===")

arr = [5, 2, 8, 1, 9, 3]
print(`Array: [${arr[0]}, ${arr[1]}, ${arr[2]}, ${arr[3]}, ${arr[4]}, ${arr[5]}]`)

reversed = reverse(arr)
print(`Reversed: [${reversed[0]}, ${reversed[1]}, ${reversed[2]}, ${reversed[3]}, ${reversed[4]}, ${reversed[5]}]`)

sliced = slice(arr, 1, 4)
print(`Sliced [1:4]: [${sliced[0]}, ${sliced[1]}, ${sliced[2]}]`)

print(`Index of 8: ${indexOf(arr, 8)}`)
print(`Includes 9: ${includes(arr, 9)}`)
print(`Includes 99: ${includes(arr, 99)}`)
print("")

// ===== 8. PRACTICAL EXAMPLES =====
print("=== 8. Practical Examples ===")

// Example 1: Process user data
users = ["alice", "bob", "charlie"]
userGreetings = map(users, u => `Hello, ${toUpperCase(u)}!`)
print("User greetings:")
for (greeting in userGreetings) {
    print(`  ${greeting}`)
}

// Example 2: Calculate statistics
scores = [85, 92, 78, 95, 88, 76, 90]
print(`Scores: length = ${len(scores)}`)

total = reduce(scores, (a, b) => a + b, 0)
average = total / len(scores)
print(`Average score: ${average}`)

passing = filter(scores, s => s >= 80)
print(`Passing scores (>=80): ${len(passing)} students`)

// Example 3: Data transformation
temperatures_c = [0, 10, 20, 30, 40]
temperatures_f = map(temperatures_c, c => c * 9 / 5 + 32)
print("")
print("Temperature conversion (C to F):")
for (i in range(len(temperatures_c))) {
    c = temperatures_c[i]
    f = temperatures_f[i]
    print(`  ${c}°C = ${f}°F`)
}
print("")

// ===== 9. COMPOUND WITH NEW FEATURES =====
print("=== 9. Combining Everything ===")

// Complex pipeline with template strings
inventory = range(1, 6)
print("Processing inventory...")

processed = map(inventory, id => id * 10)

filtered = filter(processed, q => q >= 30)
total = reduce(filtered, (a, b) => a + b, 0)

summary = `Total items: ${len(inventory)}, Filtered: ${len(filtered)}, Sum: ${total}`
print(summary)
print("")

// ===== SUMMARY =====
print("╔═════════════════════════════════════════════════╗")
print("║  ✅ Template strings with ${} interpolation     ║")
print("║  ✅ map() - Transform arrays                    ║")
print("║  ✅ filter() - Select items                     ║")
print("║  ✅ reduce() - Aggregate values                 ║")
print("║  ✅ Enhanced string functions                   ║")
print("║  ✅ Enhanced array functions                    ║")
print("║                                                 ║")
print("║  A-lang: Now with FULL modern features! 🚀     ║")
print("╚═════════════════════════════════════════════════╝")
