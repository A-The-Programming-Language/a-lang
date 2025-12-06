# 🆕 New Features: input() & FFI - A-lang v1.0-preview

**Added**: December 2024  
**Status**: ✅ Fully Functional  
**Platforms**: Linux (input + FFI), macOS (input + FFI), Windows (input only - FFI coming soon)

---

## 📥 1. User Input - `input()` Function

### Overview

The `input()` function allows you to read user input from the console, similar to Python's `input()`.

### Syntax

```javascript
input()                  // Read input without prompt
input(prompt)            // Read input with prompt message
```

### Parameters

- `prompt` (optional, string): Message to display before reading input

### Returns

- `string`: The user's input (without trailing newline)

### Examples

#### Basic Input

```javascript
name = input("What is your name? ")
print("Hello, " + name + "!")
```

#### Numeric Input

```javascript
age_str = input("How old are you? ")
age = int(age_str)

if (age >= 18) {
    print("You are an adult")
} else {
    print("You are a minor")
}
```

#### Calculator

```javascript
num1 = float(input("First number: "))
num2 = float(input("Second number: "))

print("Sum: " + str(num1 + num2))
print("Product: " + str(num1 * num2))
```

#### Menu System

```javascript
print("1. Option A")
print("2. Option B")
print("3. Exit")

choice = input("Choose: ")

if (choice == "1") {
    print("You chose A")
} elif (choice == "2") {
    print("You chose B")
} elif (choice == "3") {
    print("Goodbye!")
} else {
    print("Invalid choice")
}
```

### Tips

1. **Always convert types**: `input()` returns a string, use `int()` or `float()` for numbers
2. **Trim whitespace**: Input is automatically trimmed of trailing newlines
3. **Error handling**: Use try/catch for invalid conversions

---

## 🔌 2. FFI - Foreign Function Interface

### Overview

FFI allows A-lang to call C functions from shared libraries (.so, .dylib, .dll), enabling:
- System programming
- Hardware access
- Legacy code integration
- Performance-critical operations

### Platform Support

| Platform | Status | Notes |
|----------|--------|-------|
| **Linux** | ✅ Full | .so libraries |
| **macOS** | ✅ Full | .dylib libraries |
| **Windows** | 🔜 Coming Soon | .dll support in development |

### FFI Functions

#### 2.1. `ffiLoadLibrary(name, path)`

Load a shared C library into memory.

**Parameters:**
- `name` (string): Library identifier for future calls
- `path` (string): Full path to .so/.dylib file

**Returns:**
- `boolean`: `true` if loaded successfully, error otherwise

**Example:**
```javascript
// Linux
result = ffiLoadLibrary("libc", "/lib/x86_64-linux-gnu/libc.so.6")

// macOS
result = ffiLoadLibrary("libc", "/usr/lib/libSystem.dylib")
```

#### 2.2. `ffiRegisterFunction(name, return_type, param_types)`

Register a C function signature before calling it.

**Parameters:**
- `name` (string): C function name
- `return_type` (string): Return type (see types below)
- `param_types` (array of strings): Parameter types

**Returns:**
- `boolean`: `true` if registered successfully

**Example:**
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

#### 2.3. `ffiCall(lib_name, func_name, args)`

Call a registered C function.

**Parameters:**
- `lib_name` (string): Library identifier (from `ffiLoadLibrary`)
- `func_name` (string): Function name (must be registered)
- `args` (array): Function arguments

**Returns:**
- `any`: Return value from C function (depends on signature)

**Example:**
```javascript
result = ffiCall("libc", "abs", [-42])        // Returns: 42
length = ffiCall("libc", "strlen", ["Hello"]) // Returns: 5
root = ffiCall("libm", "sqrt", [16.0])        // Returns: 4.0
```

---

## 🎯 Supported C Types

### Type Mapping Table

| A-lang Type | C Type | Size | Description |
|-------------|--------|------|-------------|
| `"int"` | `int` | 32-bit | Signed integer |
| `"long"` | `long` | 64-bit | Long integer |
| `"float"` | `float` | 32-bit | Single precision float |
| `"double"` | `double` | 64-bit | Double precision float |
| `"string"` | `char*` | pointer | Null-terminated string |
| `"bool"` | `bool` | 1 byte | Boolean value |
| `"void"` | `void` | - | No return value |
| `"pointer"` | `void*` | pointer | Generic pointer |

### Supported Function Signatures

✅ **Currently Supported:**
- `int func(int)`
- `int func(int, int)`
- `int func(string)`
- `double func(double)`
- `double func(double, double)`
- `float func(float)`
- `void func()`
- `void func(int)`
- `void func(string)`
- `void func(int, int)`
- `long func(long)`
- `bool func(bool)`

🔜 **Coming Soon:**
- Structs
- Arrays (pointer + length)
- Callbacks
- Variadic functions

---

## 📚 Complete Examples

### Example 1: Standard C Library Functions

```javascript
// Load libc
ffiLoadLibrary("libc", "/lib/x86_64-linux-gnu/libc.so.6")

// Integer functions
ffiRegisterFunction("abs", "int", ["int"])
print(ffiCall("libc", "abs", [-42]))  // 42

// String functions
ffiRegisterFunction("strlen", "int", ["string"])
print(ffiCall("libc", "strlen", ["Hello, World!"]))  // 13
```

### Example 2: Math Library

```javascript
// Load libm (math library)
ffiLoadLibrary("libm", "/lib/x86_64-linux-gnu/libm.so.6")

// Square root
ffiRegisterFunction("sqrt", "double", ["double"])
print(ffiCall("libm", "sqrt", [16.0]))  // 4.0

// Power
ffiRegisterFunction("pow", "double", ["double", "double"])
print(ffiCall("libm", "pow", [2.0, 8.0]))  // 256.0

// Floor/Ceil
ffiRegisterFunction("floor", "double", ["double"])
print(ffiCall("libm", "floor", [3.7]))  // 3.0

ffiRegisterFunction("ceil", "double", ["double"])
print(ffiCall("libm", "ceil", [3.2]))  // 4.0
```

### Example 3: Custom C Library

**Step 1: Create C library (mylib.c)**
```c
// mylib.c
int add(int a, int b) {
    return a + b;
}

int multiply(int a, int b) {
    return a * b;
}

double circle_area(double radius) {
    return 3.14159 * radius * radius;
}
```

**Step 2: Compile as shared library**
```bash
gcc -shared -fPIC -o libmylib.so mylib.c
```

**Step 3: Use in A-lang**
```javascript
// Load custom library
ffiLoadLibrary("mylib", "./libmylib.so")

// Register functions
ffiRegisterFunction("add", "int", ["int", "int"])
ffiRegisterFunction("multiply", "int", ["int", "int"])
ffiRegisterFunction("circle_area", "double", ["double"])

// Call functions
sum = ffiCall("mylib", "add", [10, 32])
print("10 + 32 = " + str(sum))  // 42

product = ffiCall("mylib", "multiply", [6, 7])
print("6 * 7 = " + str(product))  // 42

area = ffiCall("mylib", "circle_area", [5.0])
print("Circle area (r=5) = " + str(area))  // 78.53975
```

### Example 4: Combining input() and FFI

```javascript
// Interactive calculator using C math functions
ffiLoadLibrary("libm", "/lib/x86_64-linux-gnu/libm.so.6")
ffiRegisterFunction("sqrt", "double", ["double"])
ffiRegisterFunction("pow", "double", ["double", "double"])

print("=== Advanced Calculator ===")

num = float(input("Enter a number: "))

// Square root
root = ffiCall("libm", "sqrt", [num])
print("√" + str(num) + " = " + str(root))

// Square (x²)
exp = input("Enter exponent: ")
result = ffiCall("libm", "pow", [num, float(exp)])
print(str(num) + "^" + exp + " = " + str(result))
```

---

## 🔍 Common Library Paths

### Linux

```javascript
// Standard C Library
"/lib/x86_64-linux-gnu/libc.so.6"      // Ubuntu/Debian
"/usr/lib/libc.so.6"                    // Some distros
"/lib64/libc.so.6"                      // RedHat/CentOS

// Math Library
"/lib/x86_64-linux-gnu/libm.so.6"      // Ubuntu/Debian
"/usr/lib/libm.so.6"                    // Some distros
"/lib64/libm.so.6"                      // RedHat/CentOS
```

### macOS

```javascript
// System Library (includes libc + libm)
"/usr/lib/libSystem.dylib"
```

### Finding Libraries

```bash
# Linux: List available libraries
ldconfig -p | grep libc

# Linux: Find library location
locate libc.so

# macOS: List libraries
ls /usr/lib/*.dylib

# Check exports in a library
nm -D /lib/x86_64-linux-gnu/libc.so.6 | grep abs
```

---

## ⚠️ Safety Considerations

### FFI is Inherently Unsafe

FFI bypasses A-lang's safety guarantees. Incorrect usage can cause:
- Segmentation faults
- Memory corruption
- Undefined behavior
- Program crashes

### Best Practices

1. **Verify types match exactly**
   ```javascript
   // ✅ Correct
   ffiRegisterFunction("abs", "int", ["int"])
   ffiCall("libc", "abs", [-42])
   
   // ❌ Wrong - type mismatch
   ffiCall("libc", "abs", ["string"])  // Crash!
   ```

2. **Check library paths exist**
   ```javascript
   if (ffiLoadLibrary("libc", "/lib/libc.so.6")) {
       // Safe to use
   } else {
       print("Library not found!")
   }
   ```

3. **Use error handling**
   ```javascript
   try {
       result = ffiCall("libc", "abs", [-42])
       print("Result: " + str(result))
   } catch (e) {
       print("FFI error: " + str(e))
   }
   ```

4. **Test with simple functions first**
   ```javascript
   // Start with abs(), strlen()
   // Move to complex functions later
   ```

5. **Never pass invalid pointers**
   ```javascript
   // Pointer/memory management is manual
   // Be careful with string lifetimes
   ```

---

## 🐛 Troubleshooting

### Library Not Found

**Error:** `Failed to load library: cannot open shared object file`

**Solutions:**
```bash
# Find the library
find /usr -name "libc.so*" 2>/dev/null
ldconfig -p | grep libc

# Check LD_LIBRARY_PATH
export LD_LIBRARY_PATH=/path/to/libs:$LD_LIBRARY_PATH
```

### Function Not Found

**Error:** `Function 'xyz' not found: undefined symbol`

**Solutions:**
```bash
# Check if function is exported
nm -D /path/to/library.so | grep function_name

# For C++ libraries, use extern "C"
extern "C" {
    int my_function(int x);
}
```

### Wrong Number of Arguments

**Error:** `Function 'xyz' expects N arguments, got M`

**Solution:**
```javascript
// Check function signature
ffiRegisterFunction("pow", "double", ["double", "double"])
// Must call with exactly 2 args
ffiCall("libm", "pow", [2.0, 8.0])  // ✅
ffiCall("libm", "pow", [2.0])       // ❌
```

### Type Mismatch

**Error:** `Expected integer argument` or segfault

**Solution:**
```javascript
// Make sure types match exactly
ffiRegisterFunction("abs", "int", ["int"])
ffiCall("libc", "abs", [-42])           // ✅ int
ffiCall("libc", "abs", [int("-42")])    // ✅ converted to int
ffiCall("libc", "abs", ["-42"])         // ❌ string (crash!)
```

---

## 🚀 Use Cases

### 1. System Programming
```javascript
// Get process ID
ffiLoadLibrary("libc", "/lib/libc.so.6")
ffiRegisterFunction("getpid", "int", [])
pid = ffiCall("libc", "getpid", [])
print("Process ID: " + str(pid))
```

### 2. Hardware Access
```javascript
// Call existing hardware driver
ffiLoadLibrary("sensor", "/usr/lib/libsensor.so")
ffiRegisterFunction("read_temperature", "double", [])
temp = ffiCall("sensor", "read_temperature", [])
print("Temperature: " + str(temp) + "°C")
```

### 3. Legacy Code Integration
```javascript
// Use existing C codebase
ffiLoadLibrary("legacy", "./liblegacy.so")
ffiRegisterFunction("process_data", "int", ["string", "int"])
result = ffiCall("legacy", "process_data", [data, size])
```

### 4. Performance Optimization
```javascript
// Offload heavy computation
ffiLoadLibrary("fastmath", "./libfastmath.so")
ffiRegisterFunction("matrix_multiply", "void", ["pointer", "pointer", "pointer"])
// ... call optimized C function
```

---

## 📊 Performance

FFI calls have minimal overhead:
- **Call overhead**: ~50-100ns per call
- **Type conversion**: ~10-20ns per argument
- **No GC pressure**: Direct memory access

**Benchmark (1000 calls to abs()):**
```javascript
start = timestamp()
count = 0
while (count < 1000) {
    result = ffiCall("libc", "abs", [-count])
    count++
}
elapsed = timestamp() - start
print("Elapsed: " + str(elapsed) + "s")
// Typically: 0.001-0.002s (1-2ms for 1000 calls)
```

---

## 📖 See Also

- **Examples**: `examples/input_demo.al`, `examples/ffi_demo.al`
- **FFI Implementation Plan**: `FFI_IMPLEMENTATION_PLAN.md`
- **Main Documentation**: `SYNTAX_REFERENCE.md`
- **Rust FFI Guide**: https://doc.rust-lang.org/nomicon/ffi.html

---

## 🤝 Contributing

To add more FFI type signatures or improve input():
1. Edit `src/interpreter/mod.rs` (input function)
2. Edit `src/stdlib/ffi.rs` (FFI types)
3. Add tests to `tests/ffi_test.rs`
4. Update this documentation

---

**Questions?** Open an issue on GitHub with the `ffi` or `input` label.