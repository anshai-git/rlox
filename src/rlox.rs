use std::{fs, io, process};

use crate::{expression::Expression, parser::Parser, scanner::Scanner, token::Token};

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

                    self.run(line.clone());
                    self.had_error = false;
                }
                Err(error) => println!("Error: {error}"),
            };
            line = String::new();
        }
    }

    pub fn run(&mut self, source: String) {
        let mut scanner = Scanner::new(source, self);
        let tokens: Vec<Token> = scanner.scan_tokens();

        println!("\n\n[TOKENS]:\n\n{:?}\n\n", &tokens);

        let mut parser: Parser = Parser::new(tokens);
        let expression: Expression = parser.parse();

        println!("\n\n[AST]:\n\n{:?}\n\n", expression);
    }

    pub fn error(&mut self, line: u64, message: String) {
        self.report(line, String::new(), message);
    }

    pub fn report(&mut self, line: u64, location: String, message: String) {
        println!("[line {}] Error {}: {}", line, location, message);
        self.had_error = true;
    }
}
