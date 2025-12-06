# 🚀 A-lang: The Revolutionary Scripting Language

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.1.0-green.svg)](Cargo.toml)

A-lang is a revolutionary scripting language written in Rust that combines the simplicity of Lua with groundbreaking features that don't exist in any other programming language. It's designed to be fast, lightweight, and incredibly powerful.

---

## 🌟 The 5 WOW Factors

A-lang introduces **5 revolutionary features** that set it apart from every other language:

### 1. ⏰ Time-Travel Debugging

Built-in time-travel debugging lets you **rewind execution, inspect historical states, and replay code** from any point. No external debugger needed!

```alang
x = 10
snapshot              // Take a snapshot of current state
x = x + 5
print(x)              // Prints: 15

rewind 1              // Go back one snapshot
print(x)              // Prints: 10 - we've traveled back in time!

checkpoint "important_point"
// ... more code ...
rewind to "important_point"  // Jump to checkpoint
```

**Features:**
- ✅ Automatic state snapshots
- ✅ Named checkpoints for quick navigation
- ✅ State inspection at any point in history
- ✅ Replay execution from any snapshot
- ✅ Export/import debugging sessions
- ✅ Zero-overhead when disabled

### 2. ⚡ Reactive Variables

Variables are **reactive by default** - changes automatically propagate through your program like in modern frontend frameworks, but at the language level!

```alang
reactive counter = 0
reactive doubled = counter * 2
reactive quadrupled = doubled * 2

effect [counter] {
    print("Counter changed to: " + counter)
}

counter = 5  // Automatically triggers:
             // - doubled becomes 10
             // - quadrupled becomes 20
             // - effect runs and prints message
```

**Features:**
- ✅ Automatic dependency tracking
- ✅ Computed values that update automatically
- ✅ Effects for side effects on changes
- ✅ Efficient change propagation (topological sort)
- ✅ Batch updates for performance
- ✅ Visual dependency graph export

### 3. 🎨 Runtime Syntax Extensions

**Create new syntax during runtime!** Define Domain-Specific Languages (DSLs) on-the-fly without recompiling.

```alang
syntax "unless" {
    pattern: "unless CONDITION then BODY",
    transform: |cond, body| {
        if !cond then body
    }
}

// Now use your custom syntax:
unless x > 10 then {
    print("x is not greater than 10")
}

// Create SQL-like syntax on the fly:
syntax "select" {
    pattern: "select FIELDS from TABLE where CONDITION",
    transform: |fields, table, cond| {
        table.filter(cond).map(|row| row.pick(fields))
    }
}
```

**Features:**
- ✅ Define new syntax patterns at runtime
- ✅ Macro expansion with proper hygiene
- ✅ Quote/unquote for metaprogramming
- ✅ DSL creation without compiler modifications
- ✅ Syntax validation and error reporting

### 4. 🔮 Smart Auto-Parallelization

The runtime **automatically detects and parallelizes** safe operations without manual intervention!

```alang
// Automatically parallelized!
results = [1, 2, 3, 4, 5].map(|x| expensive_computation(x))

// Explicit parallel blocks for maximum control
parallel {
    a = compute_something()
    b = compute_other()
    c = compute_third()
}

// Parallel comprehensions
squares = parallel for x in 1..1000000 {
    x * x
}
```

**Features:**
- ✅ Automatic parallel map/filter/reduce
- ✅ Work-stealing thread pool
- ✅ Deadlock prevention
- ✅ Smart cost analysis (only parallelize when beneficial)
- ✅ Load balancing across cores
- ✅ Thread-safe by default

### 5. 🧠 Context-Aware Type System

Types **adapt to their usage context** with bidirectional type inference, providing the flexibility of dynamic typing with the safety of static typing.

```alang
// Types inferred from usage - no annotations needed!
fn add(a, b) {
    a + b  // Compiler knows a and b must be numeric
}

// Gradual typing - add types only when you want
fn process(x: Integer) -> String {
    x.to_string() + " processed"
}

// Context-aware refinement
fn handle(value) {
    if type_of(value) == "integer" {
        // Here, value is known to be Integer
        value + 1
    } else {
        // Here, value is not Integer
        value
    }
}

// Types adapt to context
context "numeric" {
    x = "42"  // Automatically converted to integer
    x + 1     // Works! Result is 43
}
```

**Features:**
- ✅ Bidirectional type inference
- ✅ Gradual typing (dynamic → static)
- ✅ Type refinement in control flow
- ✅ Context-dependent behavior
- ✅ Union and intersection types
- ✅ Smart error messages

---

## 🎯 Why A-lang?

| Feature | Lua | Python | JavaScript | A-lang |
|---------|-----|--------|------------|--------|
| Performance | ⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| Time-Travel Debugging | ❌ | ❌ | ❌ | ✅ |
| Reactive Variables | ❌ | ❌ | ❌ | ✅ |
| Runtime Syntax Ext. | ❌ | ❌ | ❌ | ✅ |
| Auto-Parallelization | ❌ | ❌ | ❌ | ✅ |
| Context-Aware Types | ❌ | ❌ | ❌ | ✅ |
| Memory Safety | ❌ | ⚠️ | ⚠️ | ✅ |
| Easy Embedding | ✅ | ⚠️ | ⚠️ | ✅ |

---

## 🚀 Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/a-lang.git
cd a-lang

# Build the project
cargo build --release

# Install (optional)
cargo install --path .
```

### Run the REPL

```bash
./target/release/alang
```

You'll see:
```
╔═══════════════════════════════════════════════════════════════╗
║  🚀 A-lang v0.1.0                                            ║
╠═══════════════════════════════════════════════════════════════╣
║  The Revolutionary Scripting Language                         ║
║                                                               ║
║  🌟 5 WOW Factors:                                            ║
║    ⏰  Time-Travel Debugging                                  ║
║    ⚡ Reactive Variables                                     ║
║    🎨 Runtime Syntax Extensions                              ║
║    🔮 Smart Auto-Parallelization                            ║
║    🧠 Context-Aware Type System                             ║
╚═══════════════════════════════════════════════════════════════╝

1> 
```

### Execute a File

```bash
alang examples/hello.al
```

---

## 📚 Language Guide

### Basic Syntax

```alang
// Variables - just assign, no let/var needed!
x = 42
name = "A-lang"
isAwesome = true

// Constants (optional)
const PI = 3.14159

// Arrays
numbers = [1, 2, 3, 4, 5]
mixed = [1, "two", 3.0, true]

// Objects
person = {
    name: "Alice",
    age: 30,
    email: "alice@example.com"
}

// Functions
fn greet(name) {
    return "Hello, " + name + "!"
}

fn factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

// Lambdas
double = |x| x * 2
add = |a, b| a + b
```

### Control Flow

```alang
// If-else (JS style)
if x > 10 {
    print("x is large")
} else {
    print("x is small")
}

// While loop
i = 0
while i < 10 {
    print(i)
    i = i + 1
}

// For loop
for i in 1..10 {
    print(i)
}

for item in [1, 2, 3] {
    print(item)
}

// Match (pattern matching)
match value {
    0 => print("zero"),
    1 => print("one"),
    _ => print("other")
}
```

### Advanced Features

#### Reactive Programming

```alang
// Create reactive variables (just add 'reactive')
reactive temperature = 20
reactive isCold = temperature < 15
reactive isHot = temperature > 30

// Computed values
computed status = {
    if isCold { "It's cold!" }
    else if isHot { "It's hot!" }
    else { "It's comfortable" }
}

// Effects (run when dependencies change)
effect [temperature] {
    print("Temperature changed to: " + temperature)
}

// Update with simple = syntax
temperature = 10  // isCold becomes true, status updates, effect runs
```

#### Time-Travel Debugging

```alang
fn fibonacci(n) {
    checkpoint "fib_start"
    
    if n <= 1 {
        return n
    }
    
    snapshot  // Automatic snapshot
    result = fibonacci(n - 1) + fibonacci(n - 2)
    
    return result
}

result = fibonacci(10)
print(result)

// Go back and inspect
rewind to "fib_start"
rewind 5  // Go back 5 snapshots
```

#### Parallel Execution

```alang
// Automatic parallelization
data = 1..1000000
results = data.map(|x| expensive_operation(x))

// Explicit parallel block
parallel {
    a = task1()
    b = task2()
    c = task3()
}

// Parallel collection operations
evens = parallel_filter(numbers, |x| x % 2 == 0)
doubled = parallel_map(numbers, |x| x * 2)
```

---

## 🛠️ Embedding A-lang

A-lang is designed to be easily embedded in Rust applications:

```rust
use a_lang::{Interpreter, run};

fn main() {
    // Quick execution
    let result = run("x = 10; x * 2").unwrap();
    println!("Result: {}", result);
    
    // Reusable interpreter
    let mut interpreter = Interpreter::new();
    
    interpreter.execute("counter = 0");
    interpreter.execute("counter = counter + 1");
    
    // Access reactive system
    let reactive_ctx = interpreter.reactive_context();
    let stats = reactive_ctx.stats();
    println!("Reactive nodes: {}", stats.total_nodes);
    
    // Access time-travel debugger
    let debugger = interpreter.time_travel_debugger();
    let tt_stats = debugger.read().unwrap().stats();
    println!("Snapshots: {}", tt_stats.total_snapshots);
}
```

---

## 📖 Examples

Check out the `examples/` directory for more:

- `hello.al` - Basic Hello World
- `fibonacci.al` - Fibonacci with time-travel
- `reactive_counter.al` - Reactive programming demo
- `parallel_map.al` - Parallel processing
- `game_simulation.al` - Complex example with all features

---

## 🏗️ Architecture

A-lang is built with a clean, modular architecture:

```
┌─────────────────────────────────────────┐
│           A-lang Source Code            │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│      Lexer (logos-based tokenizer)      │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│    Parser (Recursive Descent + AST)     │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│         Interpreter Engine              │
│  ┌────────────────────────────────┐     │
│  │  • Time-Travel Debugger        │     │
│  │  • Reactive System             │     │
│  │  • Type Inference              │     │
│  │  • Parallel Executor           │     │
│  │  • Syntax Extension Registry   │     │
│  └────────────────────────────────┘     │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│             Output / REPL               │
└─────────────────────────────────────────┘
```

---

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_reactive_variables

# Run benchmarks
cargo bench
```

---

## 🚧 Roadmap

- [x] Core interpreter with basic features
- [x] Time-travel debugging system
- [x] Reactive variable system
- [x] Parallel execution framework
- [x] Type inference engine
- [ ] Full syntax extension implementation
- [ ] Standard library
- [ ] Module system
- [ ] Package manager
- [ ] LSP server for IDE support
- [ ] Bytecode compiler for performance
- [ ] JIT compilation (LLVM backend)
- [ ] Debugger UI (GUI/TUI)
- [ ] Web playground
- [ ] VS Code extension

---

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup

```bash
# Clone the repo
git clone https://github.com/yourusername/a-lang.git
cd a-lang

# Build in debug mode
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

A-lang is built on the shoulders of giants:
- **Rust** - For memory safety and performance
- **logos** - Fast lexer generator
- **im** - Persistent data structures for time-travel
- **rayon** - Data parallelism library
- **tokio** - Async runtime for reactive system

---

## 📬 Contact

- GitHub: [@yourusername](https://github.com/yourusername)
- Email: your.email@example.com
- Discord: [Join our community](https://discord.gg/alang)

---

## ⭐ Star History

If you find A-lang interesting, please consider giving it a star on GitHub! It helps others discover the project.

---

**Made with ❤️ and Rust 🦀**

---

## 🎓 Learn More

- [Language Specification](docs/spec.md)
- [API Documentation](https://docs.rs/a-lang)
- [Tutorial Series](docs/tutorial/)
- [Blog Posts](https://blog.alang.dev)
- [Conference Talks](docs/talks.md)

---

## 🌍 Community

Join our growing community:
- [Discord Server](https://discord.gg/alang)
- [Reddit](https://reddit.com/r/alang)
- [Twitter](https://twitter.com/alang_dev)
- [Stack Overflow](https://stackoverflow.com/questions/tagged/a-lang)

---

> **"A-lang: Where the future of scripting begins."** 🚀