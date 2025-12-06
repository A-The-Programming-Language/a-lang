// ========================================
// A-lang FFI Demo
// Foreign Function Interface - Calling C functions
// ========================================

print("╔════════════════════════════════════════╗")
print("║     A-lang FFI Demo - Call C Functions ║")
print("╚════════════════════════════════════════╝")
print("")

// ========================================
// Example 1: Load libc (Standard C Library)
// ========================================
print("=== Example 1: Loading C Library ===")
print("")

// Try to load libc (different paths for different systems)
libc_paths = [
    "/lib/x86_64-linux-gnu/libc.so.6",      // Ubuntu/Debian
    "/usr/lib/libc.so.6",                    // Some Linux
    "/lib64/libc.so.6",                      // RedHat/CentOS
    "/usr/lib/libSystem.dylib"               // macOS
]

loaded = false
for (path in libc_paths) {
    if (ffiLoadLibrary("libc", path)) {
        print("✓ Successfully loaded libc from: " + path)
        loaded = true
        break
    }
}

if (!loaded) {
    print("❌ Failed to load libc. FFI may not be available on this system.")
    print("   (Windows support coming soon)")
    exit(1)
}

// Load libm (math library) for mathematical functions
libm_paths = [
    "/lib/x86_64-linux-gnu/libm.so.6",      // Ubuntu/Debian
    "/usr/lib/libm.so.6",                    // Some Linux
    "/lib64/libm.so.6",                      // RedHat/CentOS
    "/usr/lib/libSystem.dylib"               // macOS (same as libc)
]

loaded_math = false
for (path in libm_paths) {
    if (ffiLoadLibrary("libm", path)) {
        print("✓ Successfully loaded libm from: " + path)
        loaded_math = true
        break
    }
}

print("")

// ========================================
// Example 2: Integer Functions
// ========================================
print("=== Example 2: Integer Functions ===")
print("")

// abs(int) -> int - Absolute value
ffiRegisterFunction("abs", "int", ["int"])
result = ffiCall("libc", "abs", [-42])
print("abs(-42) = " + str(result))

result = ffiCall("libc", "abs", [42])
print("abs(42) = " + str(result))

result = ffiCall("libc", "abs", [-999])
print("abs(-999) = " + str(result))
print("")

// ========================================
// Example 3: String Functions
// ========================================
print("=== Example 3: String Functions ===")
print("")

// strlen(char*) -> int - String length
ffiRegisterFunction("strlen", "int", ["string"])

text1 = "Hello, World!"
len1 = ffiCall("libc", "strlen", [text1])
print("strlen('" + text1 + "') = " + str(len1))

text2 = "A-lang"
len2 = ffiCall("libc", "strlen", [text2])
print("strlen('" + text2 + "') = " + str(len2))

text3 = ""
len3 = ffiCall("libc", "strlen", [text3])
print("strlen('') = " + str(len3))
print("")

// ========================================
// Example 4: Math Functions
// ========================================
print("=== Example 4: Math Functions ===")
print("")

// sqrt(double) -> double - Square root
if (loaded_math) {
    ffiRegisterFunction("sqrt", "double", ["double"])

    num = 16.0
    root = ffiCall("libm", "sqrt", [num])
    print("sqrt(" + str(num) + ") = " + str(root))

    num = 2.0
    root = ffiCall("libm", "sqrt", [num])
    print("sqrt(" + str(num) + ") = " + str(root))

    num = 144.0
    root = ffiCall("libm", "sqrt", [num])
    print("sqrt(" + str(num) + ") = " + str(root))
} else {
    print("(sqrt skipped - libm not available)")
}
print("")

// pow(double, double) -> double - Power
if (loaded_math) {
    ffiRegisterFunction("pow", "double", ["double", "double"])

    base = 2.0
    exp = 8.0
    result = ffiCall("libm", "pow", [base, exp])
    print("pow(" + str(base) + ", " + str(exp) + ") = " + str(result))

    base = 3.0
    exp = 4.0
    result = ffiCall("libm", "pow", [base, exp])
    print("pow(" + str(base) + ", " + str(exp) + ") = " + str(result))

    base = 10.0
    exp = 2.0
    result = ffiCall("libm", "pow", [base, exp])
    print("pow(" + str(base) + ", " + str(exp) + ") = " + str(result))
} else {
    print("(pow skipped - libm not available)")
}
print("")

// ========================================
// Example 5: Practical Use Case - Text Processing
// ========================================
print("=== Example 5: Practical Use Case ===")
print("")

texts = ["short", "medium text", "a very long text string"]

print("Text Length Analysis:")
for (text in texts) {
    len = ffiCall("libc", "strlen", [text])
    print("  '" + text + "' -> " + str(len) + " characters")
}
print("")

// ========================================
// Example 6: Error Handling
// ========================================
print("=== Example 6: Error Handling ===")
print("")

print("Testing error scenarios...")

// This should work
try {
    result = ffiCall("libc", "abs", [123])
    print("✓ Valid call: abs(123) = " + str(result))
} catch (e) {
    print("✗ Unexpected error: " + str(e))
}

// Try calling unregistered function
try {
    result = ffiCall("libc", "nonexistent", [])
    print("✗ Should have failed!")
} catch (e) {
    print("✓ Caught expected error: Function not registered")
}

print("")

// ========================================
// Example 7: Performance Test
// ========================================
print("=== Example 7: Performance Test ===")
print("")

print("Calling abs() 1000 times...")
start = timestamp()
count = 0
while (count < 1000) {
    result = ffiCall("libc", "abs", [-count])
    count++
}
end = timestamp()
elapsed = end - start
print("✓ Completed in " + str(elapsed) + " seconds")
print("")

// ========================================
// Example 8: Supported Types Summary
// ========================================
print("=== Example 8: Supported FFI Types ===")
print("")
print("Supported C types:")
print("  ✓ int      - 32-bit signed integer")
print("  ✓ long     - 64-bit signed integer")
print("  ✓ float    - 32-bit floating point")
print("  ✓ double   - 64-bit floating point")
print("  ✓ string   - char* (null-terminated)")
print("  ✓ bool     - boolean value")
print("  ✓ void     - no return value")
print("  ✓ pointer  - void* (generic pointer)")
print("")

// ========================================
// Example 9: Common libc Functions
// ========================================
print("=== Example 9: More libc Functions ===")
print("")

if (loaded_math) {
    // floor(double) -> double
    ffiRegisterFunction("floor", "double", ["double"])
    val = 3.7
    result = ffiCall("libm", "floor", [val])
    print("floor(" + str(val) + ") = " + str(result))

    // ceil(double) -> double
    ffiRegisterFunction("ceil", "double", ["double"])
    val = 3.2
    result = ffiCall("libm", "ceil", [val])
    print("ceil(" + str(val) + ") = " + str(result))

    // fabs(double) -> double - Floating point absolute
    ffiRegisterFunction("fabs", "double", ["double"])
    val = -5.5
    result = ffiCall("libm", "fabs", [val])
    print("fabs(" + str(val) + ") = " + str(result))
} else {
    print("(Math functions skipped - libm not available)")
}

print("")

// ========================================
// Example 10: Custom Library (Optional)
// ========================================
print("=== Example 10: Custom C Library (Optional) ===")
print("")

print("To use custom C libraries:")
print("  1. Create your C library:")
print("     // mylib.c")
print("     int add(int a, int b) { return a + b; }")
print("")
print("  2. Compile as shared library:")
print("     gcc -shared -fPIC -o libmylib.so mylib.c")
print("")
print("  3. Load and use in A-lang:")
print("     ffiLoadLibrary('mylib', './libmylib.so')")
print("     ffiRegisterFunction('add', 'int', ['int', 'int'])")
print("     result = ffiCall('mylib', 'add', [10, 32])")
print("")

// Try to load custom library (if exists)
if (ffiLoadLibrary("custom", "./libcustom.so")) {
    print("✓ Found custom library!")
    print("  (Example custom functions could be called here)")
} else {
    print("  (No custom library found - this is optional)")
}
print("")

// ========================================
// Final Statistics
// ========================================
print("╔════════════════════════════════════════╗")
print("║     ✓ FFI Demo Complete!               ║")
print("╚════════════════════════════════════════╝")
print("")
print("Summary:")
print("  • Loaded C library: libc")
print("  • Tested integer functions (abs)")
print("  • Tested string functions (strlen)")
print("  • Tested math functions (sqrt, pow)")
print("  • Demonstrated error handling")
print("  • Showed performance capabilities")
print("")
print("FFI enables A-lang to call any C library!")
print("Use cases:")
print("  - System programming")
print("  - Hardware drivers")
print("  - Legacy code integration")
print("  - Performance-critical operations")
