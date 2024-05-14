use crate::{object::Object, rlox::RLox, token::Token, token_type::TokenType};
use std::{collections::HashMap, process};

pub struct Scanner<'a> {
    source: String,
    tokens: Vec<Token>,

    start: u16,
    current: u16,
    line: u16,

    keywords: HashMap<String, TokenType>,

    rlox: &'a mut RLox,
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

fn is_alpha_numeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

impl<'a> Scanner<'a> {
    pub fn new(source: String, rlox: &'a mut RLox) -> Self {
        let keywords: HashMap<String, TokenType> = HashMap::from([
            ("and".to_string(), TokenType::And),
            ("class".to_string(), TokenType::Class),
            ("else".to_string(), TokenType::Else),
            ("false".to_string(), TokenType::False),
            ("for".to_string(), TokenType::For),
            ("fun".to_string(), TokenType::Fun),
            ("if".to_string(), TokenType::If),
            ("nil".to_string(), TokenType::Nil),
            ("or".to_string(), TokenType::Or),
            ("print".to_string(), TokenType::Print),
            ("return".to_string(), TokenType::Return),
            ("super".to_string(), TokenType::Super),
            ("this".to_string(), TokenType::This),
            ("true".to_string(), TokenType::True),
            ("var".to_string(), TokenType::Var),
            ("while".to_string(), TokenType::While),
        ]);
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            rlox,
            keywords,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: Object::RString("".to_string()),
            line: self.line,
        });

        &self.tokens
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();

        if let Some(token_type) = self.map_char_to_token_type(c) {
            self.add_token(token_type, Object::empty());
        }
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if let Some(current_char) = self.source.chars().nth(self.current as usize) {
            if current_char == expected {
                self.current += 1;
                return true;
            }
        }

        false
    }

    fn map_char_to_token_type(&mut self, c: char) -> Option<TokenType> {
        match c {
            '(' => Some(TokenType::LeftParen),
            ')' => Some(TokenType::RightParen),
            '{' => Some(TokenType::LeftBrace),
            '}' => Some(TokenType::RightBrace),
            ',' => Some(TokenType::Comma),
            '.' => Some(TokenType::Dot),
            '-' => Some(TokenType::Minus),
            '+' => Some(TokenType::Plus),
            ';' => Some(TokenType::Semicolon),
            '*' => Some(TokenType::Star),
            '!' => {
                if self.match_next('=') {
                    Some(TokenType::BangEqual)
                } else {
                    Some(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_next('=') {
                    Some(TokenType::EqualEqual)
                } else {
                    Some(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_next('=') {
                    Some(TokenType::LessEqual)
                } else {
                    Some(TokenType::Less)
                }
            }
            '>' => {
                if self.match_next('=') {
                    Some(TokenType::GreaterEqual)
                } else {
                    Some(TokenType::Greater)
                }
            }
            '\\' => {
                if self.match_next('\\') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    None
                } else {
                    Some(TokenType::Slash)
                }
            }
            ' ' | '\r' | '\t' => None,
            '\n' => {
                self.line += 1;
                None
            }
            '"' => {
                self.string();
                None
            }
            _ => {
                if is_digit(c) {
                    self.number();
                } else if is_alpha(c) {
                    self.identifier();
                } else {
                    self.rlox
                        .error(self.line, "Unexpected character".to_string());
                }
                None
            }
        }
    }

    fn identifier(&mut self) {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text: String = self.source[self.start as usize..self.current as usize].to_string();

        let token_type: TokenType = match self.keywords.get(&text) {
            Some(token_type) => token_type.clone(),
            None => TokenType::Identifier,
        };

        self.add_token(token_type, Object::RString(text));
    }

    fn number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }

        // Look for a fractional part
        if self.peek() == '.' && is_digit(self.peek_next()) {
            // Consume the '.'
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }

        let string_value: String =
            self.source[(self.start) as usize..(self.current) as usize].to_string();
        let number_value: f64 = string_value.parse::<f64>().unwrap();
        let literal: Object = Object::RNumber(number_value);
        self.add_token(TokenType::Number, literal);
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.chars().count() as u16 {
            '\0'
        } else {
            self.source
                .chars()
                .nth((self.current + 1) as usize)
                .unwrap()
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.rlox
                .error(self.line, "Unterminated sstring.".to_string());
            return;
        }

        // Consume the closing '"'
        self.advance();

        // Trim the surrounding quotes
        let literal: String =
            self.source[(self.start + 1) as usize..(self.current - 1) as usize].to_string();
        self.add_token(TokenType::String, Object::RString(literal));
    }

    fn add_token(&mut self, token_type: TokenType, literal: Object) {
        let lexeme: String = self.source[self.start as usize..self.current as usize].to_string();

        self.tokens.push(Token {
            token_type,
            line: self.line,
            lexeme,
            literal,
        });
    }

    fn advance(&mut self) -> char {
        match self.source.chars().nth(self.current as usize) {
            Some(c) => {
                self.current += 1;
                c
            }
            None => {
                println!("Invalid index in source code.");
                self.rlox
                    .error(self.line, "Invalid index in source code.".to_string());
                process::exit(1);
            }
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current as usize).unwrap()
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.chars().count() as u16
    }
}
