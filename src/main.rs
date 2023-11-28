use std::fs;
use std::env;
use std::io;
use std::io::Write;

fn run_prompt() {
    loop {
        print!("> ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    break;
                }
                run(&input)
            }
            Err(error) => println!("error: {error}"),
        }
    }
}

fn run_file(file_path: &String) {
    match fs::read_to_string(file_path) {
        Ok(source) => run(&source),
        Err(error) => println!("error: {error}")
    }
}

fn run(source: &String) {
    println!("{source}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        run_prompt();
    } else {
        let file_path = &args[1];
        run_file(file_path);
    }
}
