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

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::Eof, String::new(), Object::Null, self.line as usize));

        self.tokens.to_vec()
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
            let mut current_char = self.peek();
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
        self.current += 1;
        self.source.chars().nth((self.current - 1) as usize).unwrap()
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
        if (self.current + 1) as usize >= self.source.chars().count() {
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

#[cfg(test)]
mod scanner_tests {
    use crate::object::Object;
    use crate::scanner::Scanner;
    use crate::rlox::RLox;
    use crate::token::Token;
    use crate::token_type::TokenType;

    #[test]
    fn test_scanner_ignores_whitespace() {
        let source: String = "\t \r \n \t \n \n \n \n \r \t \n \n \r \r \t \t".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source, &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 1);
    }

    #[test]
    fn test_scanner_line_number() {
        //                               0  1      2           3  4  5  6  7  8  9  10 11
        let source: String = "identifier \n \"some \n string\" \n \n \n \n \n \n \n \n \n".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source, &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(scanner.line == 12);

        assert!(TokenType::Identifier == result.get(0).unwrap().token_type);
        assert!("identifier".to_string() == result.get(0).unwrap().lexeme);

        assert!(TokenType::String == result.get(1).unwrap().token_type);
        assert!(Object::String("some \n string".to_string()) == result.get(1).unwrap().literal);
        
        assert!(TokenType::Eof == result.get(2).unwrap().token_type);
    }

    #[test]
    fn test_scann_left_paren() {
        let source: String = "(".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source, &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::LeftParen == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);
    }

    #[test]
    fn test_scann_right_paren() {
        let source: String = ")".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source, &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::RightParen == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);
    }

    #[test]
    fn test_scann_left_brace() {
        let source: String = "{".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source, &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::LeftBrace == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);
    }

    #[test]
    fn test_scann_right_brace() {
        let source: String = "}".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source, &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::RightBrace == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);
    }

    #[test]
    fn test_scann_minus() {
        let source: String = "-".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source, &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::Minus == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);
    }

    #[test]
    fn test_scann_plus() {
        let source: String = "+".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source, &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::Plus == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);
    }

    #[test]
    fn test_scann_comma() {
        let source: String = ",".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source, &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::Comma == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);
    }

    #[test]
    fn test_scann_dot() {
        let source: String = ".".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source, &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::Dot == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);
    }

    #[test]
    fn test_scann_semicolon() {
        let source: String = ";".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source, &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::Semicolon == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);
    }

    #[test]
    fn test_scann_star() {
        let source: String = "*".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source, &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::Star == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);
    }

    #[test]
    fn test_scann_equal() {
        let source: String = "=".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source, &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();
        assert!(result.len() == 2);
        assert!(TokenType::Equal == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);
    }

    #[test]
    fn test_scann_less() {
        let source: String = "<".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source, &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::Less == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);
    }

    #[test]
    fn test_scann_greater() {
        let source: String = ">".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source, &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::Greater == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);
    }

    #[test]
    fn test_scann_equal_equal() {
        let source: String = "==".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source, &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::EqualEqual == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);
    }

    #[test]
    fn test_scann_greater_equal() {
        let source: String = ">=".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source, &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::GreaterEqual == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);
    }

    #[test]
    fn test_scann_less_equal() {
        let source: String = "<=".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source, &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::LessEqual == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);
    }

    #[test]
    fn test_scann_single_line_comment() {
        let source: String = "// this should be ignored.".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source, &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 1);
        assert!(TokenType::Eof == result.get(0).unwrap().token_type);
    }

    #[test]
    fn test_scann_string() {
        let source: String = "\"This should be parsed as a string.\"".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source.clone(), &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::String == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);

        assert!(source == result.get(0).unwrap().lexeme);
        // Trim the surrounding quotes
        let trimmed = source[1 .. source.len() - 1].to_string();
        assert!(Object::String(trimmed) == result.get(0).unwrap().literal);
    }

    #[test]
    fn test_scann_number() {
        let source: String = "15.28".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source.clone(), &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::Number == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);

        assert!(source == result.get(0).unwrap().lexeme);
        assert!(Object::Number(15.28) == result.get(0).unwrap().literal);
    }

    #[test]
    fn test_scann_and_keyword() {
        let source: String = "and".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source.clone(), &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::And == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);

        assert!(source == result.get(0).unwrap().lexeme);
    }

    #[test]
    fn test_scann_class_keyword() {
        let source: String = "class".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source.clone(), &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::Class == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);

        assert!(source == result.get(0).unwrap().lexeme);
    }

    #[test]
    fn test_scann_else_keyword() {
        let source: String = "else".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source.clone(), &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::Else == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);

        assert!(source == result.get(0).unwrap().lexeme);
    }

    #[test]
    fn test_scann_false_keyword() {
        let source: String = "false".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source.clone(), &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::False == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);

        assert!(source == result.get(0).unwrap().lexeme);
    }

    #[test]
    fn test_scann_for_keyword() {
        let source: String = "for".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source.clone(), &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::For == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);

        assert!(source == result.get(0).unwrap().lexeme);
    }

    #[test]
    fn test_scann_fun_keyword() {
        let source: String = "fun".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source.clone(), &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::Fun == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);

        assert!(source == result.get(0).unwrap().lexeme);
    }

    #[test]
    fn test_scann_if_keyword() {
        let source: String = "if".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source.clone(), &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::If == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);

        assert!(source == result.get(0).unwrap().lexeme);
    }

    #[test]
    fn test_scann_null_keyword() {
        let source: String = "null".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source.clone(), &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::Null == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);

        assert!(source == result.get(0).unwrap().lexeme);
    }

    #[test]
    fn test_scann_or_keyword() {
        let source: String = "or".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source.clone(), &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::Or == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);

        assert!(source == result.get(0).unwrap().lexeme);
    }

    #[test]
    fn test_scann_print_keyword() {
        let source: String = "print".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source.clone(), &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::Print == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);

        assert!(source == result.get(0).unwrap().lexeme);
    }

    #[test]
    fn test_scann_return_keyword() {
        let source: String = "return".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source.clone(), &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::Return == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);

        assert!(source == result.get(0).unwrap().lexeme);
    }

    #[test]
    fn test_scann_super_keyword() {
        let source: String = "super".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source.clone(), &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::Super == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);

        assert!(source == result.get(0).unwrap().lexeme);
    }

    #[test]
    fn test_scann_this_keyword() {
        let source: String = "this".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source.clone(), &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::This == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);

        assert!(source == result.get(0).unwrap().lexeme);
    }

    #[test]
    fn test_scann_true_keyword() {
        let source: String = "true".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source.clone(), &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::True == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);

        assert!(source == result.get(0).unwrap().lexeme);
    }

    #[test]
    fn test_scann_var_keyword() {
        let source: String = "var".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source.clone(), &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::Var == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);

        assert!(source == result.get(0).unwrap().lexeme);
    }

    #[test]
    fn test_scann_while_keyword() {
        let source: String = "while".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source.clone(), &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::While == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);

        assert!(source == result.get(0).unwrap().lexeme);
    }

    #[test]
    fn test_scann_identifier() {
        let source: String = "some_identifier_01".to_string();
        let mut rlox: RLox = RLox::new();
        let mut scanner: Scanner = Scanner::new(source.clone(), &mut rlox);
        let result: Vec<Token> = scanner.scan_tokens();

        assert!(result.len() == 2);
        assert!(TokenType::Identifier == result.get(0).unwrap().token_type);
        assert!(TokenType::Eof == result.get(1).unwrap().token_type);

        assert!(source == result.get(0).unwrap().lexeme);
    }
}
