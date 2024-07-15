use std::thread::panicking;

use crate::{
    expression::Expression,
    object::Object,
    statement::Statement,
    token::Token,
    token_type::TokenType::{self, *},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: u64,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements: Vec<Statement> = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration());
        }
        statements
    }

    fn declaration(&mut self) -> Statement {
        // Synchronization should happen here
        return if self.match_token(TokenType::Var) {
            self.var_declaration()
        } else {
            self.statement()
        };
    }

    fn var_declaration(&mut self) -> Statement {
        let name: Token = self.consume(TokenType::Identifier, "Expect variable name.");

        let mut initializer = None;
        if self.match_token(TokenType::Equal) {
            initializer = Some(self.expression());
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        );
        Statement::Var {
            name,
            initializer: Box::new(initializer),
        }
    }

    fn statement(&mut self) -> Statement {
        if self.match_token(Print) {
            return self.print_statement();
        }

        if self.match_token(LeftBrace) {
            return Statement::Block {
                statements: self.block_statement(),
            };
        }

        self.expression_statement()
    }

    fn block_statement(&mut self) -> Vec<Statement> {
        let mut statements: Vec<Statement> = Vec::new();

        while !self.check(RightBrace) && !self.is_at_end() {
            statements.push(self.declaration());
        }

        self.consume(RightBrace, "Expect '}' after a block.");
        statements
    }

    fn print_statement(&mut self) -> Statement {
        let value: Expression = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after value.");
        Statement::Print {
            value: Box::new(value),
        }
    }

    fn expression_statement(&mut self) -> Statement {
        let expression: Expression = self.expression();
        self.consume(TokenType::Semicolon, "Expect ';' after expression.");
        Statement::Expression {
            expression: Box::new(expression),
        }
    }

    fn expression(&mut self) -> Expression {
        self.assignment()
    }

    fn assignment(&mut self) -> Expression {
        let expr: Expression = self.equality();
        if self.match_token(Equal) {
            let equals: Token = self.previous();
            let value: Expression = self.assignment();
            if let Expression::Variable { name } = expr {
                return Expression::Assign {
                    name,
                    value: Box::new(value),
                };
            }

            panic!("Invalid assignment target.");
        }
        expr
    }

    fn equality(&mut self) -> Expression {
        let mut expression: Expression = self.comparison();

        while self.match_tokens(vec![BangEqual, EqualEqual]) {
            let operator: Token = self.previous();
            let right: Expression = self.comparison();
            expression = Expression::Binary {
                left: Box::new(expression),
                right: Box::new(right),
                operator,
            }
        }

        expression
    }

    fn comparison(&mut self) -> Expression {
        let mut expression: Expression = self.term();

        while (self.match_tokens(vec![Greater, GreaterEqual, Less, LessEqual])) {
            let operator: Token = self.previous();
            let right: Expression = self.term();
            expression = Expression::Binary {
                left: Box::new(expression),
                right: Box::new(right),
                operator,
            }
        }

        expression
    }

    fn term(&mut self) -> Expression {
        let mut expression: Expression = self.factor();

        while self.match_tokens(vec![Minus, Plus]) {
            let operator: Token = self.previous();
            let right: Expression = self.factor();
            expression = Expression::Binary {
                left: Box::new(expression),
                right: Box::new(right),
                operator,
            }
        }

        expression
    }

    fn factor(&mut self) -> Expression {
        let mut expression: Expression = self.unary();

        while self.match_tokens(vec![Slash, Star]) {
            let operator: Token = self.previous();
            let right: Expression = self.unary();
            expression = Expression::Binary {
                left: Box::new(expression),
                right: Box::new(right),
                operator,
            }
        }

        expression
    }

    fn unary(&mut self) -> Expression {
        if self.match_tokens(vec![Bang, Minus]) {
            let operator: Token = self.previous();
            let right: Expression = self.unary();
            return Expression::Unary {
                operator,
                right: Box::new(right),
            };
        }

        self.primary()
    }

    fn primary(&mut self) -> Expression {
        if self.match_token(False) {
            return Expression::Literal {
                value: Object::Boolean(false),
            };
        };
        if self.match_token(Identifier) {
            return Expression::Variable {
                name: self.previous(),
            };
        }
        if self.match_token(True) {
            return Expression::Literal {
                value: Object::Boolean(true),
            };
        };
        if self.match_token(Null) {
            return Expression::Literal {
                value: Object::Null,
            };
        };

        if self.match_tokens(vec![Number, String]) {
            return Expression::Literal {
                value: self.previous().literal,
            };
        };

        if self.match_token(LeftParen) {
            let expression: Expression = self.expression();
            self.consume(RightParen, "Expect ')' after expression");
            return Expression::Grouping {
                expression: Box::new(expression),
            };
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
        self.tokens
            .get((self.current - 1) as usize)
            .unwrap()
            .clone()
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Token {
        if self.check(token_type.clone()) {
            return self.advance();
        }

        panic!("Expect {:?}", token_type);
    }
}
