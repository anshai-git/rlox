use std::collections::HashMap;

use crate::{object::Object, rlox::RLox, token::Token, token_type::TokenType};

pub struct Scanner<'a> {
    keywords: HashMap<String, TokenType>,
    source: String,
    tokens: Vec<Token>,

    start: u64,
    current: u64,
    line: u64,

    rlox: &'a mut RLox
}

impl<'a> Scanner<'a> {
    pub fn new(source: String, rlox: &'a mut RLox) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(),      TokenType::And);
        keywords.insert("class".to_string(),    TokenType::Class);
        keywords.insert("else".to_string(),     TokenType::Else);
        keywords.insert("false".to_string(),    TokenType::False);
        keywords.insert("for".to_string(),      TokenType::For);
        keywords.insert("fun".to_string(),      TokenType::Fun);
        keywords.insert("if".to_string(),       TokenType::If);
        keywords.insert("null".to_string(),     TokenType::Null);
        keywords.insert("or".to_string(),       TokenType::Or);
        keywords.insert("print".to_string(),    TokenType::Print);
        keywords.insert("return".to_string(),   TokenType::Return);
        keywords.insert("super".to_string(),    TokenType::Super);
        keywords.insert("this".to_string(),     TokenType::This);
        keywords.insert("true".to_string(),     TokenType::True);
        keywords.insert("var".to_string(),      TokenType::Var);
        keywords.insert("while".to_string(),    TokenType::While);

        Self {
            keywords,
            source,
            tokens: Vec::new(),

            start: 0,
            current: 0,
            line: 1,

            rlox
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::Eof, String::new(), Object::Null, self.line as usize));
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen, Object::Null),
            ')' => self.add_token(TokenType::RightParen, Object::Null),
            '{' => self.add_token(TokenType::LeftBrace, Object::Null),
            '}' => self.add_token(TokenType::RightBrace, Object::Null),
            ',' => self.add_token(TokenType::Comma, Object::Null),
            '.' => self.add_token(TokenType::Dot, Object::Null),
            '-' => self.add_token(TokenType::Minus, Object::Null),
            '+' => self.add_token(TokenType::Plus, Object::Null),
            ';' => self.add_token(TokenType::Semicolon, Object::Null),
            '*' => self.add_token(TokenType::Star, Object::Null),
            '!' => {
                let token_type: TokenType = if self.match_next('=') { TokenType::BangEqual } else { TokenType::Bang };
                self.add_token(token_type, Object::Null);
            },
            '=' => {
                let token_type: TokenType = if self.match_next('=') { TokenType::EqualEqual } else { TokenType::Equal };
                self.add_token(token_type, Object::Null);
            },
            '<' => {
                let token_type: TokenType = if self.match_next('=') { TokenType::LessEqual } else { TokenType::Less };
                self.add_token(token_type, Object::Null);
            },
            '>' => {
                let token_type: TokenType = if self.match_next('=') { TokenType::GreaterEqual } else { TokenType::Greater };
                self.add_token(token_type, Object::Null);
            },
            '/' => {
                // Single line comment
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, Object::Null);
                }
            }
            ' '
            | '\r'
            | '\t' => {},
            '\n' => { self.line += 1 },
            '"' => self.string(),
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    self.rlox.error(self.line, "Unexpected character".to_string());
                }
            }
        }
    }

    // Identifier
    fn identifier(&mut self) { 
        let mut current_char: char = self.peek();
        while self.is_alpha_numeric(current_char) {
            self.advance();
            current_char = self.peek();
        }
        let text: String = self.source[self.start as usize .. self.current as usize].to_string();
        let token_type: TokenType = self.keywords.get(&text)
            .or(Some(&TokenType::Identifier))
            .unwrap()
            .to_owned();
        self.add_token(token_type, Object::Null);
    }

    // Number
    fn number(&mut self) {
        let mut current_char = self.peek();
        while self.is_digit(current_char) {
            self.advance();
            current_char = self.peek();
        }

        let next_char = self.peek_next();
        if current_char == '.' && self.is_digit(next_char) {
            self.advance();
            while self.is_digit(current_char) {
                self.advance();
                current_char = self.peek();
            }
        }

        let text = self.source[self.start as usize .. self.current as usize].to_string();
        self.add_token(TokenType::Number, Object::Number(text.parse::<f64>().unwrap()));
    }

    // String
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1; }
            self.advance();
        }

        if self.is_at_end() {
            self.rlox.error(self.line, "Unterminated String".to_string());
        }

        // The closing '"'
        self.advance();

        // Trim the surrounding quotes
        let literal: String = self.source[(self.start + 1) as usize .. (self.current - 1) as usize].to_string();
        self.add_token(TokenType::String, Object::String(literal));
    }

    // Helpers
    fn is_at_end(&mut self) -> bool {
        self.current as usize >= self.source.chars().count()
    }

    fn advance(&mut self) -> char {
        self.source.chars().nth(self.current as usize).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType, literal: Object) {
        let lexeme: String = self.source[(self.start as usize) .. (self.current as usize)].to_string();
        self.tokens.push(Token::new(token_type, lexeme, literal, self.line as usize));
    }

    // Consumes the next character if it matches the 'expected' value
    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current as usize).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current as usize).unwrap()
    }

    fn peek_next(&mut self) -> char {
        if (self.current + 1) as usize > self.source.chars().count() {
            return '\0';
        }
        self.source.chars().nth((self.current + 1) as usize).unwrap()
    }

    fn is_digit(&mut self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha(&mut self, c: char) -> bool {
        (c >= 'a' && c <= 'z') ||
        (c >= 'A' && c <= 'Z') ||
        c == '_'
    }

    fn is_alpha_numeric(&mut self, c:char) -> bool {
        self.is_digit(c) || self.is_alpha(c)
    }

}
