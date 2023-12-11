use std::env;
use std::fs;
use std::io;
use std::io::Write;

use parser::Parser;
use scanner::Scanner;
use scanner::Token;
use scanner::TokenType;

use crate::ast_printer::print_ast;

mod ast;
mod ast_printer;
mod parser;
mod scanner;

pub struct Lox {
    had_error: bool,
}

impl Lox {
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
            Ok(source) => self.run(&source),
            Err(error) => println!("error: {error}"),
        }
    }

    fn run(&mut self, source: &String) {
        let mut scanner = Scanner::new(source, self);
        let tokens = scanner.scan_tokens().to_owned();
        let mut parser = Parser::new(&tokens, self);
        let expr = parser.parse();
        if self.had_error {
            return;
        }

        println!("{}", print_ast(&expr.unwrap()))
    }

    fn error(&mut self, token: Token, message: String) {
        if token.token_type == TokenType::Eof {
            self.report(token.line, format!("at end {message}"));
        } else {
            self.report(token.line, format!("at '{}' {}", token.lexeme, message));
        }
    }

    fn report(&mut self, line: i32, message: String) {
        eprintln!("[line {line}] Error: {message}");
        self.had_error = true;
    }
}

fn main() {
    let mut lox = Lox { had_error: false };
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        lox.run_prompt();
    } else {
        let file_path = &args[1];
        lox.run_file(file_path);
    }
}
