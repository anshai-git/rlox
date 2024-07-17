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
        if self.match_token(For) {
            return self.for_statement();
        }

        if self.match_token(If) {
            return self.if_statement();
        }

        if self.match_token(Print) {
            return self.print_statement();
        }

        if self.match_token(While) {
            return self.while_statement();
        }

        if self.match_token(LeftBrace) {
            return Statement::Block {
                statements: self.block_statement(),
            };
        }

        self.expression_statement()
    }

    fn for_statement(&mut self) -> Statement {
        self.consume(LeftParen, "Expect '(' after 'for'.");

        let mut initializer: Option<Statement> = None;
        if self.match_token(Semicolon) {
            initializer = None;
        } else if self.match_token(Var) {
            initializer = Some(self.var_declaration());
        } else {
            initializer = Some(self.expression_statement());
        }

        let mut condition: Option<Expression> = None;
        if !self.check(Semicolon) {
            condition = Some(self.expression());
        }
        self.consume(Semicolon, "Expect ';' after loop condition.");

        let mut increment: Option<Expression> = None;
        if !self.check(RightParen) {
            increment = Some(self.expression());
        }
        self.consume(RightParen, "Expect ')' after for clauses.");

        let mut body: Statement = self.statement();

        if let Some(increment_expression) = increment {
            body = Statement::Block {
                statements: vec![
                    body,
                    Statement::Expression {
                        expression: Box::new(increment_expression),
                    },
                ],
            }
        }

        if let None = condition {
            condition = Some(Expression::Literal {
                value: Object::Boolean(true),
            })
        }
        body = Statement::While {
            condition: Box::new(condition.unwrap()),
            body: Box::new(body),
        };

        if let Some(initializer_expression) = initializer {
            body = Statement::Block {
                statements: vec![initializer_expression, body],
            }
        }

        return body;
    }

    fn while_statement(&mut self) -> Statement {
        self.consume(LeftParen, "Expect '(' after 'while'.");
        let condition: Expression = self.expression();
        self.consume(RightParen, "Expect ')' after condition.");
        let body: Statement = self.statement();

        Statement::While {
            condition: Box::new(condition),
            body: Box::new(body),
        }
    }

    fn if_statement(&mut self) -> Statement {
        self.consume(LeftParen, "Expect '(' after 'if'.");
        let condition: Expression = self.expression();
        self.consume(RightParen, "Expect ')' after condition.");

        let then_branch: Statement = self.statement();
        let mut else_branch: Option<Statement> = None;
        if self.match_token(Else) {
            else_branch = Some(self.statement());
        }

        Statement::If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
        }
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
        let expr: Expression = self.or();
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

    fn or(&mut self) -> Expression {
        let mut expr: Expression = self.and();

        while self.match_token(Or) {
            let operator = self.previous();
            let right = self.and();
            expr = Expression::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
        }

        expr
    }

    fn and(&mut self) -> Expression {
        let mut expr: Expression = self.equality();

        while self.match_token(And) {
            let operator: Token = self.previous();
            let right: Expression = self.equality();
            expr = Expression::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            }
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

        self.call()
    }

    fn call(&mut self) -> Expression {
        let mut expression: Expression = self.primary();

        loop {
            if self.match_token(LeftParen) {
                expression = self.finish_call(expression);
            } else {
                break;
            }
        }

        expression
    }

    fn finish_call(&mut self, callee: Expression) -> Expression {
        let mut arguments: Vec<Expression> = Vec::new();
        if !self.check(RightParen) {
            loop {
                if (arguments.len() >= 255) {
                    panic!("Can't have more than 255 arguments.");
                }
                arguments.push(self.expression());
                if !self.match_token(Comma) {
                    break;
                }
            }
        }
        let paren: Token = self.consume(RightParen, "Expect ')' after arguments.");
        Expression::Call {
            callee: Box::new(callee),
            paren,
            arguments,
        }
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
