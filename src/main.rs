#[macro_use]
extern crate lazy_static;

use std::env;
use std::io::Write;

mod ast;
mod common;
mod interp;
mod lexer;
mod parser;
mod token;
mod value;

use interp::UvlInterpreter;
use value::UvlError;

fn run_file(file_path: &str) {
    let source_file = match std::fs::read_to_string(file_path) {
        Ok(source) => source,
        Err(err) => {
            println!("Failed to open file {}, error: {}", file_path, err);
            std::process::exit(65)
        }
    };

    let mut interp = UvlInterpreter::new();
    if let Err(err) = interp.run(&source_file) {
        println!("{}", err);

        match err {
            UvlError::RuntimeError(_) => std::process::exit(70),
            _ => std::process::exit(65),
        }
    }
}

fn run_prompt() {
    let mut line_buffer = String::new();
    let stdin = std::io::stdin();

    let mut interp = UvlInterpreter::new();
    loop {
        print!("::> ");
        std::io::stdout().flush().unwrap();

        match stdin.read_line(&mut line_buffer) {
            Ok(_) => {
                match interp.run(&line_buffer) {
                    Ok(value) => println!("{}", value),
                    Err(err) => println!("{}", err),
                }

                line_buffer.clear();
                interp.reset();
            }
            Err(_) => {
                std::process::exit(65);
            }
        }
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() > 2 {
        println!("Usage: uvl [file]");
        std::process::exit(65);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}
