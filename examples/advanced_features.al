// A-lang: Advanced Features Demo
// Comprehensive showcase of File I/O, JSON, Math, and all features

print("╔════════════════════════════════════════════════════╗")
print("║  A-lang: Advanced Features Demo                    ║")
print("║  File I/O + JSON + Math + Complete Features        ║")
print("╚════════════════════════════════════════════════════╝")
print("")

// ===== 1. FILE I/O OPERATIONS =====
print("=== 1. File I/O Operations ===")

// Write to file
data = "Hello from A-lang!\nThis is line 2\nThis is line 3"
writeFile("/tmp/alang_test.txt", data)
print("✓ File written to /tmp/alang_test.txt")

// Read file
if (fileExists("/tmp/alang_test.txt")) {
    content = readFile("/tmp/alang_test.txt")
    print(`✓ File read: ${len(content)} characters`)
} else {
    print("✗ File does not exist")
}

// Read lines
lines = readLines("/tmp/alang_test.txt")
print(`✓ Read ${len(lines)} lines:`)
for (i in range(len(lines))) {
    line = lines[i]
    print(`  Line ${i + 1}: ${line}`)
}

// Append to file
appendFile("/tmp/alang_test.txt", "Appended line")
print("✓ Appended to file")
print("")

// ===== 2. JSON OPERATIONS =====
print("=== 2. JSON Operations ===")

// Create object and convert to JSON
person = {
    name: "Alice",
    age: 30,
    city: "TechCity",
    skills: ["JavaScript", "Python", "A-lang"]
}

jsonString = stringifyJSON(person)
print("✓ Object to JSON:")
print(`  ${jsonString}`)

// Parse JSON back
parsed = parseJSON(jsonString)
print("✓ JSON parsed back:")
print(`  Name: ${parsed.name}`)
print(`  Age: ${parsed.age}`)
print(`  City: ${parsed.city}`)

// Save JSON to file
writeFile("/tmp/person.json", jsonString)
print("✓ JSON saved to /tmp/person.json")

// Load JSON from file
loadedJson = readFile("/tmp/person.json")
loadedPerson = parseJSON(loadedJson)
print(`✓ JSON loaded: ${loadedPerson.name} from ${loadedPerson.city}`)
print("")

// ===== 3. ADVANCED MATH =====
print("=== 3. Advanced Math Functions ===")

// Basic math
print(`sqrt(64) = ${sqrt(64)}`)
print(`pow(2, 8) = ${pow(2, 8)}`)
print(`pow(3, 3) = ${pow(3, 3)}`)

// Constants
print(`PI = ${PI}`)
print(`E = ${E}`)

// Trigonometry
angle = PI / 4
print(`sin(PI/4) = ${sin(angle)}`)
print(`cos(PI/4) = ${cos(angle)}`)
print(`tan(PI/4) = ${tan(angle)}`)

// Random numbers
print("Random numbers:")
for (i in 1..6) {
    r = random()
    print(`  ${i}. random() = ${r}`)
}

print("Random integers 1-100:")
for (i in 1..6) {
    ri = randomInt(1, 100)
    print(`  ${i}. randomInt(1,100) = ${ri}`)
}
print("")

// ===== 4. PRACTICAL EXAMPLE: DATA PROCESSING =====
print("=== 4. Practical Example: Data Processing ===")

// Generate sample data
users = [
    {name: "Alice", score: 95},
    {name: "Bob", score: 87},
    {name: "Charlie", score: 92},
    {name: "Diana", score: 78}
]

// Save as JSON
usersJson = stringifyJSON(users)
writeFile("/tmp/users.json", usersJson)
print("✓ Sample data saved")

// Process data
loadedUsers = parseJSON(readFile("/tmp/users.json"))
print(`✓ Loaded ${len(loadedUsers)} users`)

// Calculate average using reduce
scores = map(loadedUsers, u => u.score)
totalScore = reduce(scores, (a, b) => a + b, 0)
avgScore = totalScore / len(scores)
print(`Average score: ${avgScore}`)

// Find high scorers
highScorers = filter(loadedUsers, u => u.score >= 90)
print(`High scorers (>=90): ${len(highScorers)}`)
for (user in highScorers) {
    print(`  - ${user.name}: ${user.score}`)
}
print("")

// ===== 5. TEMPLATE STRINGS + FILE I/O =====
print("=== 5. Template Strings + File I/O ===")

// Generate HTML report
title = "User Report"
date = "2024-01-15"
html = `<!DOCTYPE html>
<html>
<head><title>${title}</title></head>
<body>
<h1>${title}</h1>
<p>Date: ${date}</p>
<p>Total Users: ${len(users)}</p>
<p>Average Score: ${avgScore}</p>
</body>
</html>`

writeFile("/tmp/report.html", html)
print("✓ HTML report generated: /tmp/report.html")
print("")

// ===== 6. MATH + MAP/FILTER =====
print("=== 6. Math + Functional Programming ===")

// Generate numbers
nums = range(1, 11)
print(`Numbers: 1-10`)

// Calculate square roots
sqrts = map(nums, n => sqrt(n))
print("Square roots:")
print(`  ${sqrts[0]}, ${sqrts[1]}, ${sqrts[2]}...`)

// Calculate powers
powers = map(nums, n => pow(n, 2))
print("Squares:")
print(`  ${powers[0]}, ${powers[1]}, ${powers[2]}, ${powers[3]}, ${powers[4]}...`)

// Trigonometric values
angles = map(range(0, 7), n => n * PI / 6)
sines = map(angles, a => sin(a))
print(`Sine values calculated for ${len(sines)} angles`)
print("")

// ===== 7. COMPLEX DATA FLOW =====
print("=== 7. Complex Data Flow ===")

// Simulate sensor data
fn generateSensorData(count) {
    data = []
    for (i in range(count)) {
        reading = {
            id: i,
            temperature: 20 + randomInt(-5, 5),
            humidity: 50 + randomInt(-10, 10),
            timestamp: i * 1000
        }
        data = push(data, reading)
    }
    return data
}

// Generate and save sensor data
sensorData = generateSensorData(10)
writeFile("/tmp/sensors.json", stringifyJSON(sensorData))
print(`✓ Generated ${len(sensorData)} sensor readings`)

// Load and analyze
loaded = parseJSON(readFile("/tmp/sensors.json"))
temperatures = map(loaded, s => s.temperature)
avgTemp = reduce(temperatures, (a, b) => a + b, 0) / len(temperatures)
maxTemp = max(temperatures[0], temperatures[1], temperatures[2], temperatures[3], temperatures[4])
minTemp = min(temperatures[0], temperatures[1], temperatures[2], temperatures[3], temperatures[4])

print(`Temperature Analysis:`)
print(`  Average: ${avgTemp}°C`)
print(`  Max: ${maxTemp}°C`)
print(`  Min: ${minTemp}°C`)

// Find anomalies (temp > 23)
anomalies = filter(loaded, s => s.temperature > 23)
if (len(anomalies) > 0) {
    print(`⚠ ${len(anomalies)} anomalies detected`)
} else {
    print(`✓ No anomalies`)
}
print("")

// ===== 8. STRING PROCESSING WITH FILES =====
print("=== 8. String Processing with Files ===")

// Create CSV data
csv = "Name,Age,City\nAlice,30,NYC\nBob,25,LA\nCharlie,35,Chicago"
writeFile("/tmp/data.csv", csv)
print("✓ CSV file created")

// Process CSV
lines = readLines("/tmp/data.csv")
header = lines[0]
print(`Header: ${header}`)

// Parse data rows
print("Data rows:")
for (i in range(1, len(lines))) {
    row = lines[i]
    parts = split(row, ",")
    if (len(parts) >= 3) {
        name = parts[0]
        age = parts[1]
        city = parts[2]
        print(`  ${name} (${age}) - ${city}`)
    }
}
print("")

// ===== 9. COMBINING EVERYTHING =====
print("=== 9. Ultimate Combination ===")

// Create a complete data pipeline
fn processDataPipeline() {
    // 1. Generate data
    data = map(range(1, 21), n => {
        value: n,
        squared: pow(n, 2),
        sqrt: sqrt(n),
        isEven: n % 2 == 0
    })

    // 2. Save to JSON
    writeFile("/tmp/pipeline.json", stringifyJSON(data))

    // 3. Load and filter
    loaded = parseJSON(readFile("/tmp/pipeline.json"))
    evens = filter(loaded, item => item.isEven)

    // 4. Calculate statistics
    values = map(evens, item => item.value)
    sum = reduce(values, (a, b) => a + b, 0)
    count = len(values)

    // 5. Generate report
    report = `Data Pipeline Report
Generated: ${count} even numbers
Sum: ${sum}
Average: ${sum / count}`

    writeFile("/tmp/report.txt", report)

    return report
}

result = processDataPipeline()
print("✓ Pipeline executed successfully")
print(result)
print("")

// ===== SUMMARY =====
print("╔════════════════════════════════════════════════════╗")
print("║  ✅ File I/O: read, write, append, exists          ║")
print("║  ✅ JSON: parse, stringify                         ║")
print("║  ✅ Math: sqrt, pow, sin, cos, tan, PI, E          ║")
print("║  ✅ Random: random(), randomInt()                  ║")
print("║  ✅ Template strings: interpolation                ║")
print("║  ✅ Map/Filter/Reduce: functional programming      ║")
print("║  ✅ Complex pipelines: real-world usage            ║")
print("║                                                    ║")
print("║  A-lang: Production-ready scripting! 🚀           ║")
print("╚════════════════════════════════════════════════════╝")

// Cleanup (optional)
// Note: In real usage, you might want to keep these files
print("")
print("Demo complete! Check /tmp/ for generated files:")
print("  - /tmp/alang_test.txt")
print("  - /tmp/person.json")
print("  - /tmp/users.json")
print("  - /tmp/sensors.json")
print("  - /tmp/report.html")
print("  - /tmp/data.csv")
print("  - /tmp/pipeline.json")
print("  - /tmp/report.txt")
