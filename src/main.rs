#[macro_use]
extern crate lazy_static;

use std::env;
use std::io::Write;

mod ast;
mod common;
mod lexer;
mod token;

struct UvlInterpreter {
    had_error: bool,
}

impl UvlInterpreter {
    pub fn new() -> UvlInterpreter {
        UvlInterpreter { had_error: false }
    }

    pub fn run(&mut self, source: &str) {
        let mut lexer = crate::lexer::Lexer::new(source, self);
        let tokens = lexer.scan();
        println!("{:?}", tokens);
    }
}

impl common::ErrorReporter for UvlInterpreter {
    fn report(&mut self, line: usize, location: &str, message: &str) {
        println!("[line {}] Error {}: {}", line, location, message);
        self.had_error = true;
    }

    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }
}

fn run_file(file_path: &str) {
    let source_file = match std::fs::read_to_string(file_path) {
        Ok(source) => source,
        Err(err) => {
            println!("Failed to open file {}, error: {}", file_path, err);
            std::process::exit(65)
        }
    };

    let mut interp = UvlInterpreter::new();
    interp.run(&source_file);

    if interp.had_error {
        std::process::exit(65);
    }
}

fn run_prompt() {
    let mut line_buffer = String::new();
    let stdin = std::io::stdin();

    let mut interp = UvlInterpreter::new();
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        match stdin.read_line(&mut line_buffer) {
            Ok(_) => {
                interp.run(&line_buffer);
                line_buffer.clear();
                interp.had_error = false;
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
