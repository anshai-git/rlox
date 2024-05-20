use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use crate::stmt::Stmt;
use crate::token::Token;
use std::{fs, io, process};

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
        let mut scanner: Scanner = Scanner::new(source, self);
        let tokens: &Vec<Token> = scanner.scan_tokens();
        println!("{:?}", tokens);

        let mut parser: Parser = Parser::new(tokens.to_vec());
        let program: Vec<Stmt> = parser.parse();

        if self.had_error {
            panic!("HAD ERROR TRUE");
        }

        // println!("AST: {}", AstPrinter::new().run(&program));
        // println!("\n result: ");

        let mut interpreter: Interpreter = Interpreter::new(&program);
        interpreter.interpret();
    }

    // Error handling
    pub fn error(&mut self, line: u16, message: String) {
        self.report(line, String::new(), message);
    }

    pub fn report(&mut self, line: u16, location: String, message: String) {
        println!("[line {}] Error {}: {}", line, location, message);
        self.had_error = true;
    }
}
