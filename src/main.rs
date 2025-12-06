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
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!(
        "║  🚀 {} v{}                                            ║",
        LANGUAGE_NAME, VERSION
    );
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║  The Revolutionary Scripting Language                         ║");
    println!("║                                                               ║");
    println!("║  🌟 5 WOW Factors:                                            ║");
    println!("║    ⏰  Time-Travel Debugging                                  ║");
    println!("║    ⚡ Reactive Variables                                     ║");
    println!("║    🎨 Runtime Syntax Extensions                              ║");
    println!("║    🔮 Smart Auto-Parallelization                            ║");
    println!("║    🧠 Context-Aware Type System                             ║");
    println!("║                                                               ║");
    println!("║  Type '.help' for help, '.exit' to quit                      ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

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
                        eprintln!("❌ Error: {}", e);
                    }
                }

                line_number += 1;
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("Goodbye! 👋");
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
            println!("Goodbye! 👋");
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
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  A-lang REPL Commands                                         ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║  .help          Show this help message                        ║");
    println!("║  .exit          Exit the REPL                                 ║");
    println!("║  .quit          Same as .exit                                 ║");
    println!("║  .version       Show version information                      ║");
    println!("║  .features      List all WOW features                         ║");
    println!("║  .examples      Show example code                             ║");
    println!("║  .reactive      Show reactive system statistics               ║");
    println!("║  .time-travel   Show time-travel debugger stats               ║");
    println!("║  .clear         Clear the screen                              ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
}

fn print_features() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  🌟 A-lang's 5 WOW Factors                                    ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║                                                               ║");
    println!("║  1. ⏰ Time-Travel Debugging                                  ║");
    println!("║     • Take snapshots of program state                         ║");
    println!("║     • Rewind execution to any point                           ║");
    println!("║     • Replay from checkpoints                                 ║");
    println!("║     • Inspect historical states                               ║");
    println!("║                                                               ║");
    println!("║  2. ⚡ Reactive Variables                                     ║");
    println!("║     • Variables that auto-update on change                    ║");
    println!("║     • Computed values from dependencies                       ║");
    println!("║     • Effects triggered by changes                            ║");
    println!("║     • Automatic dependency tracking                           ║");
    println!("║                                                               ║");
    println!("║  3. 🎨 Runtime Syntax Extensions                              ║");
    println!("║     • Define new syntax during runtime                        ║");
    println!("║     • Create custom DSLs on-the-fly                           ║");
    println!("║     • Macro system with hygiene                               ║");
    println!("║     • Quote/unquote for metaprogramming                       ║");
    println!("║                                                               ║");
    println!("║  4. 🔮 Smart Auto-Parallelization                            ║");
    println!("║     • Automatic parallel execution                            ║");
    println!("║     • Safe concurrent operations                              ║");
    println!("║     • Work-stealing thread pool                               ║");
    println!("║     • Load balancing                                          ║");
    println!("║                                                               ║");
    println!("║  5. 🧠 Context-Aware Type System                             ║");
    println!("║     • Types adapt to usage context                            ║");
    println!("║     • Bidirectional type inference                            ║");
    println!("║     • Gradual typing support                                  ║");
    println!("║     • Type refinement in control flow                         ║");
    println!("║                                                               ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
}

fn print_examples() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  📚 A-lang Examples                                           ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!();
    println!("  // Basic Variables");
    println!("  let x = 42;");
    println!("  let message = \"Hello, A-lang!\";");
    println!();
    println!("  // Reactive Variables");
    println!("  reactive counter <- 0;");
    println!("  computed doubled <- counter * 2;");
    println!("  counter <- 5;  // doubled automatically becomes 10");
    println!();
    println!("  // Time-Travel Debugging");
    println!("  let x = 10;");
    println!("  snapshot;");
    println!("  x = x + 5;");
    println!("  rewind 1;  // Go back to snapshot");
    println!();
    println!("  // Functions");
    println!("  fn factorial(n) {{");
    println!("    if n <= 1 {{");
    println!("      return 1;");
    println!("    }}");
    println!("    return n * factorial(n - 1);");
    println!("  }}");
    println!();
    println!("  // Arrays and Loops");
    println!("  let numbers = [1, 2, 3, 4, 5];");
    println!("  for num in numbers {{");
    println!("    print(num);");
    println!("  }}");
    println!();
    println!("╚═══════════════════════════════════════════════════════════════╝");
}

fn print_reactive_stats(interpreter: &Interpreter) {
    let ctx = interpreter.reactive_context();
    let stats = ctx.stats();

    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  ⚡ Reactive System Statistics                               ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!(
        "║  Total Nodes:       {:>5}                                   ║",
        stats.total_nodes
    );
    println!(
        "║  Signals:           {:>5}                                   ║",
        stats.signals
    );
    println!(
        "║  Computed Values:   {:>5}                                   ║",
        stats.computed_values
    );
    println!(
        "║  Effects:           {:>5}                                   ║",
        stats.effects
    );
    println!(
        "║  Dependencies:      {:>5}                                   ║",
        stats.total_dependencies
    );
    println!("╚═══════════════════════════════════════════════════════════════╝");
}

fn print_time_travel_stats(interpreter: &Interpreter) {
    let debugger = interpreter.time_travel_debugger();
    let stats = debugger.read().unwrap().stats();

    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  ⏰ Time-Travel Debugger Statistics                          ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!(
        "║  Total Snapshots:   {:>5}                                   ║",
        stats.total_snapshots
    );
    println!(
        "║  Current Position:  {:>5}                                   ║",
        stats.current_position
    );
    println!(
        "║  Checkpoints:       {:>5}                                   ║",
        stats.checkpoints_count
    );
    println!(
        "║  Memory Usage:      {:>5} bytes                            ║",
        stats.memory_usage
    );
    println!("╚═══════════════════════════════════════════════════════════════╝");
}
