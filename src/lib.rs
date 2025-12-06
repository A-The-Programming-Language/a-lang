//! # A-lang: A Revolutionary Scripting Language
//!
//! A-lang is a modern scripting language written in Rust that combines the simplicity
//! of Lua with groundbreaking features that don't exist in any other language.
//!
//! ## 🚀 The 5 WOW Factors
//!
//! ### 1. ⏰ Time-Travel Debugging
//! Built-in time-travel debugging lets you rewind execution, inspect historical states,
//! and replay code execution from any point. No external debugger needed!
//!
//! ```alang
//! let x = 10;
//! snapshot;           // Take a snapshot
//! x = x + 5;
//! rewind 1;           // Go back one snapshot
//! print(x);           // Prints: 10
//! ```
//!
//! ### 2. ⚡ Reactive Variables
//! Variables are reactive by default - changes automatically propagate through your program
//! like in modern frontend frameworks, but at the language level.
//!
//! ```alang
//! reactive counter <- 0;
//! computed double <- counter * 2;
//! counter <- 5;       // double automatically becomes 10
//! ```
//!
//! ### 3. 🎨 Runtime Syntax Extensions
//! Create new syntax during runtime! Define DSLs on-the-fly without recompiling.
//!
//! ```alang
//! syntax "unless" {
//!     pattern: "unless COND then BODY",
//!     transform: |cond, body| if !cond then body
//! }
//! ```
//!
//! ### 4. 🔮 Smart Auto-Parallelization
//! The runtime automatically detects and parallelizes safe operations without manual intervention.
//!
//! ```alang
//! parallel {
//!     let results = [1, 2, 3, 4, 5].map(|x| heavy_computation(x));
//! }
//! // Automatically runs on multiple threads!
//! ```
//!
//! ### 5. 🧠 Context-Aware Type System
//! Types adapt to their usage context with bidirectional type inference, providing
//! the flexibility of dynamic typing with the safety of static typing.
//!
//! ```alang
//! fn process(x) {  // Type inferred from usage
//!     x + 1        // x must be numeric
//! }
//! ```
//!
//! ## Architecture
//!
//! A-lang is built with a modular architecture:
//! - **Lexer**: Fast tokenization using the `logos` crate
//! - **Parser**: Recursive descent parser with error recovery
//! - **AST**: Rich abstract syntax tree supporting all language features
//! - **Interpreter**: Tree-walking interpreter with optimizations
//! - **Reactive System**: Graph-based reactive propagation
//! - **Time-Travel**: Persistent data structures for efficient state snapshots
//! - **Type System**: Context-aware bidirectional type inference
//!
//! ## Performance
//!
//! While A-lang is an interpreted language, it's designed for performance:
//! - Written in Rust for memory safety and speed
//! - Uses persistent data structures (from the `im` crate) for efficient state management
//! - Automatic parallelization for CPU-intensive operations
//! - Lazy evaluation where beneficial
//!
//! ## Example Programs
//!
//! ### Fibonacci with Time-Travel
//! ```alang
//! fn fib(n) {
//!     checkpoint "fib_start";
//!     if n <= 1 {
//!         return n;
//!     }
//!     return fib(n - 1) + fib(n - 2);
//! }
//!
//! let result = fib(10);
//! rewind to "fib_start";  // Go back and try again!
//! ```
//!
//! ### Reactive Counter
//! ```alang
//! reactive count <- 0;
//! reactive doubled <- count * 2;
//! reactive quadrupled <- doubled * 2;
//!
//! effect [count] {
//!     print("Count changed to: " + count);
//! }
//!
//! count <- 5;  // Triggers effect, updates doubled and quadrupled
//! ```

pub mod ast;
pub mod interpreter;
pub mod lexer;
pub mod parallel;
pub mod parser;
pub mod reactive;
pub mod stdlib;
pub mod syntax_ext;
pub mod time_travel;
pub mod types;

// Re-export commonly used types
pub use ast::{Expression, Literal, Program, Span, Statement};
pub use interpreter::value::Value;
pub use interpreter::{Interpreter, InterpreterError};
pub use lexer::{tokenize, Token};
pub use reactive::{ReactiveContext, ReactiveError};
pub use stdlib::{
    ByteBuffer, ByteOrder, FFIContext, GpioController, HardwareManager, HttpClient, SystemUtils,
};
pub use time_travel::{TimeTravelDebugger, TimeTravelError};

/// A-lang version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// A-lang language name
pub const LANGUAGE_NAME: &str = "A-lang";

/// Language file extension
pub const FILE_EXTENSION: &str = ".al";

/// Convenience function to run A-lang code from a string
///
/// # Example
/// ```rust
/// use a_lang;
///
/// let code = r#"
///     let x = 10;
///     let y = 20;
///     x + y
/// "#;
///
/// match a_lang::run(code) {
///     Ok(result) => println!("Result: {}", result),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub fn run(source: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let tokens = tokenize(source)?;
    let program = parser::parse(tokens)?;
    let mut interpreter = Interpreter::new();
    let result = interpreter.execute(&program)?;
    Ok(result)
}

/// Run A-lang code with a custom interpreter
///
/// This allows you to reuse an interpreter instance to maintain state
/// between multiple executions.
pub fn run_with_interpreter(
    source: &str,
    interpreter: &mut Interpreter,
) -> Result<Value, Box<dyn std::error::Error>> {
    let tokens = tokenize(source)?;
    let program = parser::parse(tokens)?;
    let result = interpreter.execute(&program)?;
    Ok(result)
}

/// Evaluate a single expression
pub fn eval(source: &str) -> Result<Value, Box<dyn std::error::Error>> {
    run(source)
}

/// Create a new A-lang interpreter
pub fn new_interpreter() -> Interpreter {
    Interpreter::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        let result = run("5 + 3").unwrap();
        assert_eq!(result, Value::Integer(8));
    }

    #[test]
    fn test_variables() {
        let result = run("let x = 42; x").unwrap();
        assert_eq!(result, Value::Integer(42));
    }

    #[test]
    fn test_string_concatenation() {
        let result = run(r#"let hello = "Hello, "; let world = "World!"; hello + world"#).unwrap();
        assert_eq!(result, Value::String("Hello, World!".to_string()));
    }

    #[test]
    fn test_function_definition_and_call() {
        let code = r#"
            fn add(a, b) {
                return a + b;
            }
            add(10, 20)
        "#;
        let result = run(code).unwrap();
        assert_eq!(result, Value::Integer(30));
    }

    #[test]
    fn test_array_operations() {
        let result = run("let arr = [1, 2, 3]; arr").unwrap();
        match result {
            Value::Array(arr) => assert_eq!(arr.len(), 3),
            _ => panic!("Expected array"),
        }
    }

    #[test]
    fn test_if_statement() {
        let result = run("if true { 42 } else { 0 }").unwrap();
        assert_eq!(result, Value::Integer(42));
    }

    #[test]
    fn test_for_loop() {
        let code = r#"
            let sum = 0;
            for i in 1..6 {
                sum = sum + i;
            }
            sum
        "#;
        let result = run(code).unwrap();
        assert_eq!(result, Value::Integer(15)); // 1+2+3+4+5
    }

    #[test]
    fn test_builtin_len() {
        let result = run(r#"len("hello")"#).unwrap();
        assert_eq!(result, Value::Integer(5));
    }

    #[test]
    fn test_builtin_type_of() {
        let result = run("type_of(42)").unwrap();
        assert_eq!(result, Value::String("integer".to_string()));
    }

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_language_name() {
        assert_eq!(LANGUAGE_NAME, "A-lang");
    }
}
