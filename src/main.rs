use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process::exit;

use interpreter::interpret;
use interpreter::RuntimeError;
use parser::Parser;
use scanner::Scanner;
use scanner::Token;
use scanner::TokenType;

mod ast;
mod interpreter;
mod parser;
mod scanner;

pub struct Lox {
    had_error: bool,
    had_runtime_error: bool,
}

impl Lox {
    fn new() -> Lox {
        Lox {
            had_error: false,
            had_runtime_error: false,
        }
    }

    fn run_prompt(&mut self) {
        loop {
            print!("> ");
            let _ = io::stdout().flush();
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    }
                    self.run(&input)
                }
                Err(error) => println!("error: {error}"),
            }
        }
    }

    fn run_file(&mut self, file_path: &String) {
        match fs::read_to_string(file_path) {
            Ok(source) => {
                self.run(&source);
                if self.had_error {
                    exit(65)
                }
                if self.had_runtime_error {
                    exit(70)
                }
            }
            Err(error) => eprintln!("error: {error}"),
        }
    }

    fn run(&mut self, source: &String) {
        let mut scanner = Scanner::new(source, self);
        let tokens = scanner.scan_tokens().to_owned();
        let mut parser = Parser::new(&tokens, self);
        let statements = parser.parse();

        match statements {
            Ok(stmts) => interpret(stmts, self),
            Err(_) => (),
        }
    }

    fn error(&mut self, token: Token, message: String) {
        if token.token_type == TokenType::Eof {
            self.report(token.line, format!("at end {message}"));
        } else {
            self.report(token.line, format!("at '{}' {}", token.lexeme, message));
        }
    }

    fn runtime_error(&mut self, error: RuntimeError) {
        eprintln!("{}\n[line {}]", error.message, error.token.line);
        self.had_runtime_error = true;
    }

    fn report(&mut self, line: i32, message: String) {
        eprintln!("[line {line}] Error: {message}");
        self.had_error = true;
    }
}

fn main() {
    let mut lox = Lox::new();
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        lox.run_prompt();
    } else {
        let file_path = &args[1];
        lox.run_file(file_path);
    }
}
