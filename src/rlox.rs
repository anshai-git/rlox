use std::{fs, io, process};

use crate::scanner::Scanner;

pub struct RLox {
    had_error: bool,
}

impl RLox {
    pub fn new() -> Self {
        RLox { had_error: false }
    }

    pub fn run_file(&mut self, file_path: &String) {
        let source: String = fs::read_to_string(file_path).expect("Source file not found.");
        self.run(source);

        if self.had_error {
            process::exit(65);
        }
    }

    pub fn run_prompt(&mut self) {
        let mut line: String = String::new();

        'repl: loop {
            match io::stdin().read_line(&mut line) {
                Ok(_) => {
                    if line.is_empty() {
                        break 'repl;
                    }

                    // TODO: solve without cloning here
                    self.run(line.clone());
                    self.had_error = false;
                }
                Err(error) => println!("Error: {error}"),
            };
            line = String::new();
        }
    }

    pub fn run(&mut self, source: String) {
        let scanner = Scanner::new(source, self);
    }

    // Error handling
    pub fn error(&mut self, line: u64, message: String) {
        self.report(line, String::new(), message);
    }

    pub fn report(&mut self, line: u64, location: String, message: String) {
        println!("[line {}] Error {}: {}", line, location, message);
        self.had_error = true;
    }
}
