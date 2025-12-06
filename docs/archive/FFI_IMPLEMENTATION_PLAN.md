# 🔌 FFI Implementation Plan - A-lang

**Status**: 🚧 In Progress  
**Version**: 1.0-preview  
**Target Release**: 1.1.0  
**Last Updated**: December 2024

---

## 📊 Current Status

### ✅ What's Already Implemented

#### 1. Core FFI Module (`src/stdlib/ffi.rs` - 298 lines)
- ✅ `FFIType` enum with C type mappings
- ✅ `FFISignature` for function signatures
- ✅ `FFILibrary` wrapper around `libloading::Library`
- ✅ `FFIContext` for managing libraries and functions
- ✅ Type conversion functions (Value ↔ C types)
- ✅ Basic function calling with `libloading`
- ✅ Unit tests for types and conversions

#### 2. Supported C Types
```rust
FFIType::Void       // void
FFIType::Int        // c_int / i32
FFIType::Long       // c_long / i64
FFIType::Float      // c_float / f32
FFIType::Double     // c_double / f64
FFIType::String     // char* / CString
FFIType::Pointer    // void* / *const c_void
FFIType::Bool       // bool
```

#### 3. Type Conversions
```rust
// A-lang → C
value_to_c_int(value: &Value) -> Result<c_int, String>
value_to_c_double(value: &Value) -> Result<c_double, String>
value_to_c_string(value: &Value) -> Result<CString, String>

// C → A-lang
c_int_to_value(val: c_int) -> Value
c_double_to_value(val: c_double) -> Value
c_string_to_value(ptr: *const c_char) -> Result<Value, String>
```

### ❌ What's Missing

1. ❌ Integration with main interpreter
2. ❌ Exposed functions in A-lang (`ffiLoadLibrary`, `ffiCall`, etc.)
3. ❌ Windows support (currently Unix-only)
4. ❌ Complex type support (structs, arrays, callbacks)
5. ❌ Documentation and examples
6. ❌ Error handling improvements
7. ❌ Memory safety guarantees

---

## 🎯 Implementation Roadmap

### Phase 1: Basic Integration (Week 1) ⚡ HIGH PRIORITY

#### Task 1.1: Connect FFIContext to Interpreter
**File**: `src/interpreter/mod.rs`

```rust
// Add to Interpreter struct
pub struct Interpreter {
    // ... existing fields
    ffi_context: Arc<Mutex<FFIContext>>,
}

// Initialize in new()
impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Self {
            // ... existing fields
            ffi_context: Arc::new(Mutex::new(FFIContext::new())),
        };
        interpreter.register_builtins();
        interpreter.register_ffi_builtins(); // NEW
        interpreter
    }
}
```

#### Task 1.2: Register FFI Built-in Functions
**File**: `src/interpreter/mod.rs`

```rust
fn register_ffi_builtins(&mut self) {
    // ffiLoadLibrary(name: string, path: string) -> boolean
    let ffi_ctx = Arc::clone(&self.ffi_context);
    self.env.define(
        "ffiLoadLibrary".to_string(),
        Value::Native(Arc::new(move |args| {
            if args.len() != 2 {
                return Err("ffiLoadLibrary expects 2 arguments (name, path)".to_string());
            }
            
            match (&args[0], &args[1]) {
                (Value::String(name), Value::String(path)) => {
                    let mut ctx = ffi_ctx.lock().unwrap();
                    match ctx.load_library(name, path) {
                        Ok(_) => Ok(Value::Boolean(true)),
                        Err(e) => Err(format!("Failed to load library: {}", e)),
                    }
                }
                _ => Err("ffiLoadLibrary expects (string, string)".to_string()),
            }
        })),
    );

    // ffiRegisterFunction(name: string, return_type: string, param_types: array) -> boolean
    let ffi_ctx = Arc::clone(&self.ffi_context);
    self.env.define(
        "ffiRegisterFunction".to_string(),
        Value::Native(Arc::new(move |args| {
            if args.len() != 3 {
                return Err("ffiRegisterFunction expects 3 arguments".to_string());
            }
            
            match (&args[0], &args[1], &args[2]) {
                (Value::String(name), Value::String(ret_type), Value::Array(params)) => {
                    // Parse return type
                    let return_type = FFIType::from_string(ret_type)
                        .map_err(|e| format!("Invalid return type: {}", e))?;
                    
                    // Parse parameter types
                    let mut param_types = Vec::new();
                    for param in params {
                        if let Value::String(type_str) = param {
                            let ffi_type = FFIType::from_string(type_str)
                                .map_err(|e| format!("Invalid param type: {}", e))?;
                            param_types.push(ffi_type);
                        } else {
                            return Err("Parameter types must be strings".to_string());
                        }
                    }
                    
                    let mut ctx = ffi_ctx.lock().unwrap();
                    ctx.register_signature(name, return_type, param_types);
                    Ok(Value::Boolean(true))
                }
                _ => Err("ffiRegisterFunction expects (string, string, array)".to_string()),
            }
        })),
    );

    // ffiCall(lib_name: string, func_name: string, args: array) -> any
    let ffi_ctx = Arc::clone(&self.ffi_context);
    self.env.define(
        "ffiCall".to_string(),
        Value::Native(Arc::new(move |args| {
            if args.len() != 3 {
                return Err("ffiCall expects 3 arguments".to_string());
            }
            
            match (&args[0], &args[1], &args[2]) {
                (Value::String(lib_name), Value::String(func_name), Value::Array(func_args)) => {
                    let ctx = ffi_ctx.lock().unwrap();
                    ctx.call_function(lib_name, func_name, func_args.clone())
                }
                _ => Err("ffiCall expects (string, string, array)".to_string()),
            }
        })),
    );
}
```

#### Task 1.3: Expand FFI Call Support
**File**: `src/stdlib/ffi.rs`

Expand `call_function` to support more signatures:

```rust
pub fn call_function(
    &self,
    lib_name: &str,
    func_name: &str,
    args: Vec<Value>,
) -> Result<Value, String> {
    let signature = self.signatures.get(func_name)
        .ok_or_else(|| format!("Function '{}' not registered", func_name))?;

    if args.len() != signature.param_types.len() {
        return Err(format!(
            "Function '{}' expects {} arguments, got {}",
            func_name, signature.param_types.len(), args.len()
        ));
    }

    let lib = self.libraries.get(lib_name)
        .ok_or_else(|| format!("Library '{}' not loaded", lib_name))?;

    #[cfg(unix)]
    {
        let lib = lib.lock().unwrap();
        
        // Generate function call based on signature
        match (&signature.return_type, signature.param_types.as_slice()) {
            // int func(int)
            (FFIType::Int, [FFIType::Int]) => {
                type FuncType = unsafe extern "C" fn(c_int) -> c_int;
                unsafe {
                    let func: libloading::Symbol<FuncType> = 
                        lib.lib.get(func_name.as_bytes())?;
                    let arg = value_to_c_int(&args[0])?;
                    Ok(c_int_to_value(func(arg)))
                }
            }
            
            // int func(int, int)
            (FFIType::Int, [FFIType::Int, FFIType::Int]) => {
                type FuncType = unsafe extern "C" fn(c_int, c_int) -> c_int;
                unsafe {
                    let func: libloading::Symbol<FuncType> = 
                        lib.lib.get(func_name.as_bytes())?;
                    let arg1 = value_to_c_int(&args[0])?;
                    let arg2 = value_to_c_int(&args[1])?;
                    Ok(c_int_to_value(func(arg1, arg2)))
                }
            }
            
            // double func(double)
            (FFIType::Double, [FFIType::Double]) => {
                type FuncType = unsafe extern "C" fn(c_double) -> c_double;
                unsafe {
                    let func: libloading::Symbol<FuncType> = 
                        lib.lib.get(func_name.as_bytes())?;
                    let arg = value_to_c_double(&args[0])?;
                    Ok(c_double_to_value(func(arg)))
                }
            }
            
            // int func(string)
            (FFIType::Int, [FFIType::String]) => {
                type FuncType = unsafe extern "C" fn(*const c_char) -> c_int;
                unsafe {
                    let func: libloading::Symbol<FuncType> = 
                        lib.lib.get(func_name.as_bytes())?;
                    let c_str = value_to_c_string(&args[0])?;
                    Ok(c_int_to_value(func(c_str.as_ptr())))
                }
            }
            
            // void func()
            (FFIType::Void, []) => {
                type FuncType = unsafe extern "C" fn();
                unsafe {
                    let func: libloading::Symbol<FuncType> = 
                        lib.lib.get(func_name.as_bytes())?;
                    func();
                    Ok(Value::Nil)
                }
            }
            
            // void func(int)
            (FFIType::Void, [FFIType::Int]) => {
                type FuncType = unsafe extern "C" fn(c_int);
                unsafe {
                    let func: libloading::Symbol<FuncType> = 
                        lib.lib.get(func_name.as_bytes())?;
                    let arg = value_to_c_int(&args[0])?;
                    func(arg);
                    Ok(Value::Nil)
                }
            }
            
            _ => Err(format!(
                "Unsupported function signature: {:?} {:?}",
                signature.return_type, signature.param_types
            )),
        }
    }

    #[cfg(not(unix))]
    {
        Err("FFI is only supported on Unix-like systems currently".to_string())
    }
}
```

---

### Phase 2: Examples and Testing (Week 2)

#### Task 2.1: Create FFI Examples
**File**: `examples/ffi_demo.al`

```javascript
// ========================================
// Example 1: Standard C Library Functions
// ========================================

print("=== FFI Demo: C Standard Library ===")
print("")

// Load libc
result = ffiLoadLibrary("libc", "/lib/x86_64-linux-gnu/libc.so.6")
if (!result) {
    print("Failed to load libc")
    exit(1)
}
print("✓ Loaded libc")

// Example 1: abs(int) -> int
ffiRegisterFunction("abs", "int", ["int"])
result = ffiCall("libc", "abs", [-42])
print("abs(-42) = " + str(result))  // 42

// Example 2: strlen(char*) -> int
ffiRegisterFunction("strlen", "int", ["string"])
len = ffiCall("libc", "strlen", ["Hello, World!"])
print("strlen('Hello, World!') = " + str(len))  // 13

// Example 3: sqrt(double) -> double
ffiRegisterFunction("sqrt", "double", ["double"])
root = ffiCall("libc", "sqrt", [16.0])
print("sqrt(16.0) = " + str(root))  // 4.0

// ========================================
// Example 2: Custom C Library
// ========================================

print("")
print("=== Custom Math Library ===")
print("")

// Compile with: gcc -shared -fPIC -o libcustom.so custom.c
// custom.c:
// int add(int a, int b) { return a + b; }
// int multiply(int a, int b) { return a * b; }
// double power(double base, double exp) { return pow(base, exp); }

result = ffiLoadLibrary("custom", "./libcustom.so")
if (result) {
    print("✓ Loaded custom library")
    
    ffiRegisterFunction("add", "int", ["int", "int"])
    sum = ffiCall("custom", "add", [10, 32])
    print("add(10, 32) = " + str(sum))  // 42
    
    ffiRegisterFunction("multiply", "int", ["int", "int"])
    product = ffiCall("custom", "multiply", [6, 7])
    print("multiply(6, 7) = " + str(product))  // 42
} else {
    print("⚠ Custom library not found (optional)")
}

print("")
print("✓ FFI Demo Complete!")
```

#### Task 2.2: Create Test Suite
**File**: `tests/ffi_test.rs`

```rust
#[cfg(test)]
mod ffi_tests {
    use super::*;

    #[test]
    fn test_ffi_load_library() {
        let mut interpreter = Interpreter::new();
        
        let code = r#"
            result = ffiLoadLibrary("libc", "/lib/x86_64-linux-gnu/libc.so.6")
            result
        "#;
        
        let result = interpreter.run(code).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_ffi_register_function() {
        let mut interpreter = Interpreter::new();
        
        let code = r#"
            ffiLoadLibrary("libc", "/lib/x86_64-linux-gnu/libc.so.6")
            result = ffiRegisterFunction("abs", "int", ["int"])
            result
        "#;
        
        let result = interpreter.run(code).unwrap();
        assert_eq!(result, Value::Boolean(true));
    }

    #[test]
    fn test_ffi_call_abs() {
        let mut interpreter = Interpreter::new();
        
        let code = r#"
            ffiLoadLibrary("libc", "/lib/x86_64-linux-gnu/libc.so.6")
            ffiRegisterFunction("abs", "int", ["int"])
            ffiCall("libc", "abs", [-42])
        "#;
        
        let result = interpreter.run(code).unwrap();
        assert_eq!(result, Value::Integer(42));
    }

    #[test]
    fn test_ffi_call_strlen() {
        let mut interpreter = Interpreter::new();
        
        let code = r#"
            ffiLoadLibrary("libc", "/lib/x86_64-linux-gnu/libc.so.6")
            ffiRegisterFunction("strlen", "int", ["string"])
            ffiCall("libc", "strlen", ["Hello"])
        "#;
        
        let result = interpreter.run(code).unwrap();
        assert_eq!(result, Value::Integer(5));
    }
}
```

---

### Phase 3: Documentation (Week 2)

#### Task 3.1: Update SYNTAX_REFERENCE.md

Add FFI section:

```markdown
## FFI - Foreign Function Interface

### Overview

A-lang provides FFI (Foreign Function Interface) to call C/C++ functions from shared libraries (.so, .dylib, .dll).

### Supported Platforms
- ✅ Linux (x86_64)
- ✅ macOS (x86_64, arm64)
- ❌ Windows (coming soon)

### Supported Types

| A-lang Type | C Type | Description |
|------------|--------|-------------|
| `"int"` | `int` | 32-bit signed integer |
| `"long"` | `long` | 64-bit signed integer |
| `"float"` | `float` | 32-bit floating point |
| `"double"` | `double` | 64-bit floating point |
| `"string"` | `char*` | Null-terminated string |
| `"bool"` | `bool` | Boolean value |
| `"void"` | `void` | No return value |
| `"pointer"` | `void*` | Generic pointer |

### Functions

#### ffiLoadLibrary(name, path)
Load a shared library.

**Parameters:**
- `name` (string): Library identifier
- `path` (string): Full path to .so/.dylib file

**Returns:** boolean (true on success)

**Example:**
```javascript
result = ffiLoadLibrary("libc", "/lib/x86_64-linux-gnu/libc.so.6")
if (!result) {
    print("Failed to load library")
}
```

#### ffiRegisterFunction(name, return_type, param_types)
Register a C function signature.

**Parameters:**
- `name` (string): Function name
- `return_type` (string): Return type
- `param_types` (array): Array of parameter types

**Returns:** boolean (true on success)

**Example:**
```javascript
ffiRegisterFunction("abs", "int", ["int"])
ffiRegisterFunction("strlen", "int", ["string"])
ffiRegisterFunction("pow", "double", ["double", "double"])
```

#### ffiCall(lib_name, func_name, args)
Call a registered C function.

**Parameters:**
- `lib_name` (string): Library identifier
- `func_name` (string): Function name
- `args` (array): Function arguments

**Returns:** any (depends on function)

**Example:**
```javascript
result = ffiCall("libc", "abs", [-42])  // 42
length = ffiCall("libc", "strlen", ["Hello"])  // 5
```

### Complete Example

```javascript
// Load library
ffiLoadLibrary("libc", "/lib/x86_64-linux-gnu/libc.so.6")

// Register functions
ffiRegisterFunction("abs", "int", ["int"])
ffiRegisterFunction("strlen", "int", ["string"])
ffiRegisterFunction("sqrt", "double", ["double"])

// Call functions
print(ffiCall("libc", "abs", [-42]))        // 42
print(ffiCall("libc", "strlen", ["test"]))  // 4
print(ffiCall("libc", "sqrt", [16.0]))      // 4.0
```

### Safety Considerations

⚠️ **Warning:** FFI is inherently unsafe. Incorrect types or invalid memory access can crash the program.

**Best Practices:**
1. Always verify library paths exist
2. Match function signatures exactly
3. Test with simple functions first
4. Use try/catch for error handling
5. Never pass invalid pointers

### Common Libraries

#### Linux - libc
```javascript
ffiLoadLibrary("libc", "/lib/x86_64-linux-gnu/libc.so.6")
// Available: abs, strlen, memcpy, printf, etc.
```

#### macOS - libc
```javascript
ffiLoadLibrary("libc", "/usr/lib/libSystem.dylib")
```

### Troubleshooting

**Library not found:**
- Check library path with: `ldd /path/to/library.so`
- Use `ldconfig -p` to list available libraries

**Function not found:**
- Verify function is exported: `nm -D /path/to/library.so | grep function_name`
- Check for C++ name mangling (use `extern "C"`)

**Type mismatch:**
- Ensure exact type matching
- Use `int` for `int`, `double` for `double`
- Strings must be null-terminated
```

---

### Phase 4: Advanced Features (Week 3-4) 🔮 FUTURE

#### Task 4.1: Windows Support
- Implement Windows DLL loading
- Use `LoadLibraryW` and `GetProcAddress`
- Handle different calling conventions (stdcall, cdecl)

#### Task 4.2: Complex Types
- Struct support with memory layout
- Array passing (pointer + length)
- Callback functions (Rust closure → C function pointer)
- Variadic functions (printf, etc.)

#### Task 4.3: Memory Management
- Automatic CString cleanup
- Reference counting for pointers
- Lifetime tracking for C objects

#### Task 4.4: Error Handling
- Better error messages
- Stack traces for FFI calls
- Validation before unsafe operations

---

## 🧪 Testing Strategy

### Unit Tests
- ✅ Type conversion (already exists)
- ✅ Signature parsing (already exists)
- 🚧 Library loading
- 🚧 Function calling
- 🚧 Error handling

### Integration Tests
- 🚧 Real C library calls (libc)
- 🚧 Custom library creation and usage
- 🚧 Error scenarios
- 🚧 Performance benchmarks

### Platform Tests
- 🚧 Linux (Ubuntu 20.04+, Debian 11+)
- 🚧 macOS (Intel + Apple Silicon)
- 🔮 Windows (future)

---

## 📝 Documentation Checklist

- [ ] Update SYNTAX_REFERENCE.md with FFI section
- [ ] Create FFI_TUTORIAL.md with step-by-step guide
- [ ] Add FFI examples to examples/
- [ ] Update README.md with FFI mention
- [ ] Add API reference to docs/
- [ ] Create troubleshooting guide
- [ ] Record demo video

---

## 🚀 Release Plan

### Version 1.1.0 - FFI Basic (Target: Q1 2025)
- ✅ Basic FFI infrastructure (done)
- 🚧 Integration with interpreter
- 🚧 Core functions (loadLibrary, registerFunction, call)
- 🚧 Unix/Linux support
- 🚧 Documentation
- 🚧 Examples

### Version 1.2.0 - FFI Enhanced (Target: Q2 2025)
- 🔮 Windows support
- 🔮 Complex types (structs, arrays)
- 🔮 Callback functions
- 🔮 Memory management improvements
- 🔮 Performance optimizations

### Version 2.0.0 - FFI Complete (Target: Q3 2025)
- 🔮 Full type system integration
- 🔮 Automatic bindings generation
- 🔮 C++ support
- 🔮 Cross-platform compatibility
- 🔮 Production-ready safety

---

## 💡 Use Cases

### 1. System Programming
```javascript
// Access system calls directly
ffiLoadLibrary("libc", "/lib/libc.so.6")
ffiRegisterFunction("getpid", "int", [])
pid = ffiCall("libc", "getpid", [])
```

### 2. Hardware Interaction
```javascript
// Use existing C drivers
ffiLoadLibrary("sensor", "/usr/lib/libsensor.so")
ffiRegisterFunction("read_temperature", "double", [])
temp = ffiCall("sensor", "read_temperature", [])
```

### 3. Legacy Code Integration
```javascript
// Call existing C codebase
ffiLoadLibrary("legacy", "./liblegacy.so")
ffiRegisterFunction("process_data", "int", ["string", "int"])
result = ffiCall("legacy", "process_data", [data, size])
```

### 4. Performance-Critical Sections
```javascript
// Offload heavy computation to C
ffiLoadLibrary("fastmath", "./libfastmath.so")
ffiRegisterFunction("matrix_multiply", "void", ["pointer", "pointer", "pointer", "int"])
ffiCall("fastmath", "matrix_multiply", [a, b, result, size])
```

---

## 🔒 Security Considerations

### Memory Safety
- All FFI calls are `unsafe` in Rust
- No automatic bounds checking
- Potential for buffer overflows
- Use-after-free risks

### Mitigation Strategies
1. Type validation before calls
2. Bounds checking in wrapper functions
3. Safe wrappers for common patterns
4. Documentation of unsafe operations
5. Runtime checks in debug mode

### Best Practices
- Minimize FFI surface area
- Prefer safe Rust implementations
- Wrap C libraries in safe interfaces
- Test extensively with sanitizers
- Document safety invariants

---

## 📚 References

- [Rust FFI Guide](https://doc.rust-lang.org/nomicon/ffi.html)
- [libloading documentation](https://docs.rs/libloading/)
- [C ABI Compatibility](https://rust-lang.github.io/unsafe-code-guidelines/layout/structs-and-tuples.html)
- [Foreign Function Interface Patterns](https://michael-f-bryan.github.io/rust-ffi-guide/)

---

## 🤝 Contributing

To contribute to FFI implementation:

1. Read this plan thoroughly
2. Pick a task from Phase 1 or 2
3. Write tests first (TDD)
4. Implement feature
5. Update documentation
6. Submit PR with examples

**Priority Areas:**
1. 🔥 Integration with interpreter (Phase 1)
2. 🔥 Basic examples and tests (Phase 2)
3. 📝 Documentation (Phase 3)
4. 🔮 Advanced features (Phase 4)

---

## 📞 Support

For questions about FFI implementation:
- Open an issue on GitHub
- Tag with `ffi` label
- Reference this document

---

**Next Steps:**
1. Review this plan
2. Start with Phase 1, Task 1.1
3. Test incrementally
4. Document as you go

**Estimated Timeline:**
- Phase 1: 1 week (basic integration)
- Phase 2: 1 week (examples + tests)
- Phase 3: 2-3 days (documentation)
- Phase 4: 2-3 weeks (advanced features)

**Total:** ~4-5 weeks for FFI v1.1.0 release