use std::fs;
use std::env;
use std::io;
use std::io::Write;

use scanner::Scanner;

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
            Err(error) => println!("error: {error}")
        }
    }

    fn run(&mut self, source: &String) {
        let mut scanner = Scanner::new(source, self);
        let tokens = scanner.scan_tokens();
        for token in tokens {
            println!("{token}");
        }
    }

    fn error(&mut self, line: i32, message: &str) {
        self.report(line, message);
    }

    fn report(&mut self, line: i32, message: &str) {
        eprintln!("[line {line}] Error: {message}");
        self.had_error = true;
    }
}

fn main() {
    let mut lox = Lox {had_error: false };
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        lox.run_prompt();
    } else {
        let file_path = &args[1];
        lox.run_file(file_path);
    }
}
