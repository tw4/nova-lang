use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::Interpreter;
use std::io::{self, Write};

pub struct Repl {
    interpreter: Interpreter,
}

impl Repl {
    pub fn new() -> Self {
        Repl {
            interpreter: Interpreter::new(),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        println!("Nova Programming Language REPL v0.1.0");
        println!("Type 'exit' or 'quit' to exit, 'help' for help");
        println!();

        loop {
            print!("nova> ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input.is_empty() {
                continue;
            }

            match input {
                "exit" | "quit" => {
                    println!("Goodbye!");
                    break;
                }
                "help" => {
                    self.print_help();
                    continue;
                }
                _ => {}
            }

            self.evaluate_line(input);
        }

        Ok(())
    }

    fn evaluate_line(&mut self, input: &str) {
        let mut lexer = Lexer::new(input);
        let tokens = match lexer.tokenize() {
            Ok(tokens) => tokens,
            Err(e) => {
                println!("Lexer Error: {}", e);
                return;
            }
        };

        let mut parser = Parser::new(tokens);
        match parser.parse() {
            Ok(program) => {
                match self.interpreter.interpret(&program) {
                    Ok(value) => {
                        if !matches!(value, crate::value::Value::Null) {
                            println!("{}", value);
                        }
                    }
                    Err(e) => {
                        println!("Runtime Error: {:?}", e);
                    }
                }
            }
            Err(e) => {
                println!("Parse Error: {:?}", e);
            }
        }
    }

    fn print_help(&self) {
        println!("Nova Programming Language Help");
        println!("=============================");
        println!();
        println!("Basic syntax:");
        println!("  Variables:   let x = 42;");
        println!("  Functions:   fn add(a, b) {{ a + b }}");
        println!("  Conditions:  if (x > 0) {{ \"positive\" }} else {{ \"negative\" }}");
        println!("  Calls:       print(\"Hello, World!\");");
        println!();
        println!("Data types:");
        println!("  Numbers:     42, 3.14");
        println!("  Strings:     \"Hello\", \"World\"");
        println!("  Booleans:    true, false");
        println!("  Null:        null");
        println!();
        println!("Operators:");
        println!("  Arithmetic:  +, -, *, /");
        println!("  Comparison:  ==, !=, <, >, <=, >=");
        println!("  Logical:     and, or, !");
        println!();
        println!("Built-in functions:");
        println!("  print(value)  - Print a value to console");
        println!();
        println!("Commands:");
        println!("  help - Show this help");
        println!("  exit, quit - Exit REPL");
        println!();
    }
}