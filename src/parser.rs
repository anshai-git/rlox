use crate::{expression::Expression, object::Object, token::Token, token_type::TokenType::{self, *}};

pub struct Parser {
    tokens: Vec<Token>,
    current: u64
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Expression {
        return self.expression();
    }

    // expression →  equality ;
    fn expression(&mut self) -> Expression {
        self.equality()
    }

    // equality →  comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Expression {
        let mut expression: Expression = self.comparison();

        while self.match_tokens(vec![BangEqual, EqualEqual]) {
            let operator: Token = self.previous();
            let right: Expression = self.comparison();
            expression = Expression::Binary { left: Box::new(expression), right: Box::new(right), operator }
        }

        expression
    }

    // comparison →  term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Expression {
        let mut expression: Expression = self.term();

        while (self.match_tokens(vec![Greater, GreaterEqual, Less, LessEqual])) {
            let operator: Token = self.previous();
            let right: Expression = self.term();
            expression = Expression::Binary { left: Box::new(expression), right: Box::new(right), operator }
        }

        expression
    }

    // term →  factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Expression {
        let mut expression: Expression = self.factor();

        while self.match_tokens(vec![Minus, Plus]) {
           let operator: Token = self.previous();
           let right: Expression = self.factor();
           expression = Expression::Binary { left: Box::new(expression), right: Box::new(right), operator }
        }

        expression
    }

    // factor →  unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Expression {
        let mut expression: Expression = self.unary();

        while self.match_tokens(vec![Slash, Star]) {
           let operator: Token = self.previous();
           let right: Expression = self.unary();
           expression = Expression::Binary { left: Box::new(expression), right: Box::new(right), operator }
        }

        expression
    }

    // unary →  ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> Expression {
        if self.match_tokens(vec![Bang, Minus]) {
            let operator: Token = self.previous();
            let right: Expression = self.unary();
            return Expression::Unary { operator, right: Box::new(right) };
        }

        self.primary()
    }


    // primary →  NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Expression {
        if self.match_token(False) { return Expression::Literal { value: Object::Boolean(false)}};
        if self.match_token(True) { return Expression::Literal { value: Object::Boolean(true)}};
        if self.match_token(Null) { return Expression::Literal { value: Object::Null}};

        if self.match_tokens(vec![Number, String]) { return Expression::Literal { value: self.previous().literal}};

        if self.match_token(LeftParen) {
            let expression: Expression = self.expression();
            self.consume(RightParen, "Expect ')' after expression");
            return Expression::Grouping { expression: Box::new(expression) }
        }

        panic!("Expect expression");
    }

    // Helpers
    fn match_token(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            return true;
        }
        false
    }

    fn match_tokens(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&mut self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().token_type == Eof
    }

    fn peek(&mut self) -> Token {
        self.tokens.get(self.current as usize).unwrap().clone()
    }

    fn previous(&mut self) -> Token {
        self.tokens.get((self.current - 1) as usize).unwrap().clone()
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Token {
        if self.check(token_type.clone()) {
            return self.advance();
        }

        panic!("Expect {:?}", token_type);
    }
}

#[cfg(test)]
mod parser_tests {
    use crate::{expression::Expression, token::Token, token_type::TokenType};

    use super::Parser;


    #[test]
    fn test_parse_primary_expression() {
        let tokens: Vec<Token> = vec![
            Token::new(TokenType::String, "some string".to_string(), crate::object::Object::String("some string".to_string()), 1),
            Token::new(TokenType::Eof, "".to_string(), crate::object::Object::Null, 1)
        ];
        let mut parser: Parser = Parser::new(tokens);
        let result: Expression = parser.parse();
        println!("[RESULT]: {:?}", result);
    }
}
