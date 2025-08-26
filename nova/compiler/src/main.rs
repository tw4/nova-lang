use nova_compiler::{Lexer, Parser, Interpreter, Repl};
use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        let mut repl = Repl::new();
        repl.run()
    } else if args.len() == 2 {
        let filename = &args[1];
        run_file(filename)
    } else {
        eprintln!("Usage: {} [script.nova]", args[0]);
        std::process::exit(1);
    }
}

fn run_file(filename: &str) -> io::Result<()> {
    let source = fs::read_to_string(filename)?;
    
    let mut lexer = Lexer::new(&source);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("Lexer Error: {}", e);
            std::process::exit(1);
        }
    };
    
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(program) => {
            let mut interpreter = Interpreter::new();
            match interpreter.interpret(&program) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Runtime Error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Parse Error: {}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}