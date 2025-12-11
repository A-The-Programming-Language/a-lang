//! # A-lang REPL (Read-Eval-Print Loop)
//!
//! Interactive shell for A-lang with support for all WOW features.

use a_lang::{run_with_interpreter, Interpreter, LANGUAGE_NAME, VERSION};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Execute file
        let filename = &args[1];
        execute_file(filename);
    } else {
        // Start REPL
        start_repl();
    }
}

fn execute_file(filename: &str) {
    let path = Path::new(filename);

    if !path.exists() {
        eprintln!("Error: File '{}' not found", filename);
        std::process::exit(1);
    }

    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            std::process::exit(1);
        }
    };

    let mut interpreter = Interpreter::new();

    match run_with_interpreter(&source, &mut interpreter) {
        Ok(result) => {
            // Only print result if it's not Nil
            if !matches!(result, a_lang::Value::Nil) {
                println!("{}", result);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn start_repl() {
    println!("╔══════════════════════════════════════╗");
    println!("║              A - L A N G             ║");
    println!("║     ___       _                      ║");
    println!("║    / _ \\     | |                     ║");
    println!("║   / /_\\ \\____| | __ _ _ __   __ _     ║");
    println!("║   |  _  |____| |/ _` | '_ \\ / _` |    ║");
    println!("║   | | | |    | | (_| | | | | (_| |    ║");
    println!("║   \\_| |_/    |_|\\__,_|_| |_|\\__, |    ║");
    println!("║                             __/ |    ║");
    println!("║                            |___/     ║");
    println!("╚══════════════════════════════════════╝");
    println!("{} v{}", LANGUAGE_NAME, VERSION);
    println!("Type \".help\" for more information.");

    let mut rl = match DefaultEditor::new() {
        Ok(editor) => editor,
        Err(e) => {
            eprintln!("Failed to initialize readline: {}", e);
            std::process::exit(1);
        }
    };

    let mut interpreter = Interpreter::new();
    let mut line_number = 1;

    loop {
        let prompt = format!("{}> ", line_number);
        match rl.readline(&prompt) {
            Ok(line) => {
                let trimmed = line.trim();

                // Skip empty lines
                if trimmed.is_empty() {
                    continue;
                }

                // Add to history
                let _ = rl.add_history_entry(&line);

                // Handle special commands
                if trimmed.starts_with('.') {
                    if handle_command(trimmed, &interpreter) {
                        break;
                    }
                    continue;
                }

                // Execute the line
                match run_with_interpreter(trimmed, &mut interpreter) {
                    Ok(result) => {
                        // Only print non-nil results
                        if !matches!(result, a_lang::Value::Nil) {
                            println!("=> {}", result);
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }

                line_number += 1;
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("Goodbye!");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn handle_command(command: &str, interpreter: &Interpreter) -> bool {
    match command {
        ".exit" | ".quit" => {
            println!("Goodbye!");
            return true;
        }
        ".help" => {
            print_help();
        }
        ".version" => {
            println!("{} v{}", LANGUAGE_NAME, VERSION);
        }
        ".features" => {
            print_features();
        }
        ".examples" => {
            print_examples();
        }
        ".reactive" => {
            print_reactive_stats(interpreter);
        }
        ".time-travel" => {
            print_time_travel_stats(interpreter);
        }
        ".clear" => {
            print!("\x1B[2J\x1B[1;1H");
        }
        _ => {
            println!("Unknown command: {}", command);
            println!("Type '.help' for available commands");
        }
    }
    false
}

fn print_help() {
    println!("Available commands:");
    println!("  .help          Show this help");
    println!("  .exit, .quit   Exit REPL");
    println!("  .version       Show version");
    println!("  .features      List features");
    println!("  .examples      Show examples");
    println!("  .reactive      Reactive stats");
    println!("  .time-travel   Time-travel stats");
    println!("  .clear         Clear screen");
}

fn print_features() {
    println!("\nA-lang Features:");
    println!("  [*] Time-Travel Debugging - Rewind and replay execution");
    println!("  [*] Reactive Variables - Auto-update on changes");
    println!("  [*] FFI - Call C functions directly");
    println!("  [*] Backend - HTTP, WebSocket, MySQL");
    println!("  [*] IoT - GPIO, I2C, SPI, UART");
    println!("\nDocumentation: https://github.com/A-The-Programming-Language/a-lang");
}

fn print_examples() {
    println!("\nExamples:");
    println!("  // Variables");
    println!("  x = 42");
    println!("  message = \"Hello, A-lang!\"");
    println!();
    println!("  // Functions");
    println!("  fn greet(name) {{ return \"Hello, \" + name }}");
    println!();
    println!("  // Input/Output");
    println!("  name = input(\"Your name: \")");
    println!("  print(\"Hello, \" + name)");
    println!();
    println!("  // FFI (Call C functions)");
    println!("  ffiLoadLibrary(\"libc\", \"/lib/libc.so.6\")");
    println!("  ffiRegisterFunction(\"abs\", \"int\", [\"int\"])");
    println!("  result = ffiCall(\"libc\", \"abs\", [-42])");
}

fn print_reactive_stats(interpreter: &Interpreter) {
    let ctx = interpreter.reactive_context();
    let stats = ctx.stats();

    println!("\n[*] Reactive System:");
    println!("  Total Nodes:     {}", stats.total_nodes);
    println!("  Signals:         {}", stats.signals);
    println!("  Computed Values: {}", stats.computed_values);
    println!("  Effects:         {}", stats.effects);
    println!("  Dependencies:    {}", stats.total_dependencies);
}

fn print_time_travel_stats(interpreter: &Interpreter) {
    let debugger = interpreter.time_travel_debugger();
    let stats = debugger.read().unwrap().stats();

    println!("\n[*] Time-Travel Debugger:");
    println!("  Total Snapshots:  {}", stats.total_snapshots);
    println!("  Current Position: {}", stats.current_position);
    println!("  Checkpoints:      {}", stats.checkpoints_count);
    println!("  Memory Usage:     {} bytes", stats.memory_usage);
}
