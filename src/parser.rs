use core::panic;

use crate::expr::Expr;
use crate::object::Object;
use crate::stmt::Stmt;
use crate::token::Token;
use crate::token_type::TokenType;

pub struct Parser {
    tokens: Vec<Token>,
    current: i16,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        while let Some(s) = Parser::declaration(&self.tokens, &mut self.current) {
            statements.push(s);
        }

        statements
    }

    fn declaration<'a>(tokens: &'a Vec<Token>, current: &mut i16) -> Option<Stmt<'a>> {
        if Parser::match_tokens(tokens, current, vec![TokenType::Var]) {
            return Parser::var_declaration(tokens, current);
        }
        Parser::statement(tokens, current)
        // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
        // TODO: synchronization point should be here
        // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    }

    fn var_declaration<'a>(tokens: &'a Vec<Token>, current: &mut i16) -> Option<Stmt<'a>> {
        if !Parser::is_at_end(tokens, current) {
            let name: &Token = Parser::consume(
                tokens,
                current,
                TokenType::Identifier,
                "Expect variable name.",
            );
            let mut initializer: Option<Expr> = None;
            if Parser::match_tokens(tokens, current, vec![TokenType::Equal]) {
                initializer = Some(Parser::expression(tokens, current));
            }
            Parser::consume(
                tokens,
                current,
                TokenType::Semicolon,
                "Expect ';' after variable declaration.",
            );
            return Some(Stmt::Var {
                name: name.clone(),
                initializer,
            });
        }
        None
    }

    fn statement<'a>(tokens: &'a Vec<Token>, current: &mut i16) -> Option<Stmt<'a>> {
        if !Parser::is_at_end(tokens, current) {
            if Parser::match_tokens(tokens, current, vec![TokenType::If]) {
                return Some(Parser::if_statement(tokens, current));
            }

            if Parser::match_tokens(tokens, current, vec![TokenType::Print]) {
                return Some(Parser::print_statement(tokens, current));
            }

            if Parser::match_tokens(tokens, current, vec![TokenType::LeftBrace]) {
                let statements: Vec<Stmt> = Parser::block(tokens, current);
                return Some(Stmt::Block {
                    statements: statements,
                });
            }

            return Some(Parser::expression_statement(tokens, current));
        }
        None
    }

    fn if_statement<'a>(tokens: &'a Vec<Token>, current: &mut i16) -> Stmt<'a> {
        Parser::consume(
            tokens,
            current,
            TokenType::LeftParen,
            "Expect '(' after 'if' keyword.",
        );
        let condition: Expr = Parser::expression(tokens, current);
        Parser::consume(
            tokens,
            current,
            TokenType::LeftParen,
            "Expect ')' after 'if' condition.",
        );

        let then_branch = Parser::statement(tokens, current);
        let mut else_branch = None;
        if Parser::match_tokens(tokens, current, vec![TokenType::Else]) {
            else_branch = Parser::statement(tokens, current);
        }

        Stmt::If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch.unwrap()),
            else_branch: Box::new(else_branch),
        }
    }

    fn block<'a>(tokens: &'a Vec<Token>, current: &mut i16) -> Vec<Stmt<'a>> {
        let mut statements: Vec<Stmt> = Vec::new();
        while !Parser::check(tokens, current, TokenType::RightBrace)
            && !Parser::is_at_end(tokens, current)
        {
            if let Some(stmt) = Parser::declaration(tokens, current) {
                statements.push(stmt);
            }
        }
        Parser::consume(
            tokens,
            current,
            TokenType::RightBrace,
            "Expect '}' after block",
        );
        statements
    }

    fn print_statement<'a>(tokens: &'a Vec<Token>, current: &mut i16) -> Stmt<'a> {
        let value: Expr = Parser::expression(tokens, current);
        Parser::consume(
            tokens,
            current,
            TokenType::Semicolon,
            "Expect ';' after value.",
        );
        Stmt::Print {
            expr: Box::new(value),
        }
    }

    fn expression_statement<'a>(tokens: &'a Vec<Token>, current: &mut i16) -> Stmt<'a> {
        let expr: Expr = Parser::expression(tokens, current);
        Parser::consume(
            tokens,
            current,
            TokenType::Super,
            "Expect ';' after expression.",
        );
        Stmt::Expression {
            expr: Box::new(expr),
        }
    }

    // expression →  equality ;
    fn expression<'a>(tokens: &'a Vec<Token>, current: &mut i16) -> Expr<'a> {
        Parser::assignment(tokens, current)
        // Parser::equality(tokens, current)
    }

    fn assignment<'a>(tokens: &'a Vec<Token>, current: &mut i16) -> Expr<'a> {
        let expr = Parser::or(tokens, current);

        if Parser::match_tokens(tokens, current, vec![TokenType::Equal]) {
            let equals = Parser::get_previous(tokens, current);
            let value = Parser::assignment(tokens, current);

            if let Expr::Variable { name } = expr {
                return Expr::Assign {
                    name,
                    value: Box::new(value),
                };
            }

            panic!("{:?}, Invalid assignment target", equals);
        }
        expr
    }

    fn or<'a>(tokens: &'a Vec<Token>, current: &mut i16) -> Expr<'a> {
        let mut expr: Expr = Parser::and(tokens, current);
        while Parser::match_tokens(tokens, current, vec![TokenType::Or]) {
            let operator: &Token = Parser::get_previous(tokens, current);
            let right: Expr = Parser::and(tokens, current);
            expr = Expr::Logical {
                left: Box::new(expr),
                right: Box::new(right),
                operator,
            };
        }
        expr
    }

    fn and<'a>(tokens: &'a Vec<Token>, current: &mut i16) -> Expr<'a> {
        let mut expr: Expr = Parser::equality(tokens, current);
        while Parser::match_tokens(tokens, current, vec![TokenType::And]) {
            let operator: &Token = Parser::get_previous(tokens, current);
            let right: Expr = Parser::equality(tokens, current);
            expr = Expr::Logical {
                left: Box::new(expr),
                right: Box::new(right),
                operator,
            };
        }
        expr
    }

    // equality →  comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality<'a>(tokens: &'a Vec<Token>, current: &mut i16) -> Expr<'a> {
        let mut expr = Parser::comparison(tokens, current);

        while Parser::match_tokens(
            tokens,
            current,
            vec![TokenType::BangEqual, TokenType::EqualEqual],
        ) {
            let operator: &Token = Parser::get_previous(tokens, current);
            let right: Expr = Parser::comparison(tokens, current);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    // comparison →  term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison<'a>(tokens: &'a Vec<Token>, current: &mut i16) -> Expr<'a> {
        let mut expr = Parser::term(tokens, current);

        while Parser::match_tokens(
            tokens,
            current,
            vec![TokenType::BangEqual, TokenType::EqualEqual],
        ) {
            let operator: &Token = Parser::get_previous(tokens, current);
            let right: Expr = Parser::term(tokens, current);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    // term →  factor ( ( "-" | "+" ) factor )* ;
    fn term<'a>(tokens: &'a Vec<Token>, current: &mut i16) -> Expr<'a> {
        let mut expr = Parser::factor(tokens, current);

        while Parser::match_tokens(tokens, current, vec![TokenType::Minus, TokenType::Plus]) {
            let operator: &Token = Parser::get_previous(tokens, current);
            let right: Expr = Parser::factor(tokens, current);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    // factor →  unary ( ( "/" | "*" ) unary )* ;
    fn factor<'a>(tokens: &'a Vec<Token>, current: &mut i16) -> Expr<'a> {
        let mut expr = Parser::unary(tokens, current);

        while Parser::match_tokens(tokens, current, vec![TokenType::Slash, TokenType::Star]) {
            let operator: &Token = Parser::get_previous(tokens, current);
            let right: Expr = Parser::unary(tokens, current);
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    // unary →  ( "!" | "-" ) unary | primary ;
    fn unary<'a>(tokens: &'a Vec<Token>, current: &mut i16) -> Expr<'a> {
        if Parser::match_tokens(tokens, current, vec![TokenType::Bang, TokenType::Minus]) {
            let operator: &Token = Parser::get_previous(tokens, current);
            let right: Expr = Parser::unary(tokens, current);
            Expr::Unary {
                right: Box::new(right),
                operator,
            }
        } else {
            Parser::primary(tokens, current)
        }
    }

    // primary →  NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary<'a>(tokens: &'a Vec<Token>, current: &mut i16) -> Expr<'a> {
        if Parser::match_tokens(tokens, current, vec![TokenType::True]) {
            return Expr::Literal {
                value: Object::RBoolean(true),
            };
        }

        if Parser::match_tokens(tokens, current, vec![TokenType::False]) {
            return Expr::Literal {
                value: Object::RBoolean(false),
            };
        }

        if Parser::match_tokens(tokens, current, vec![TokenType::Nil]) {
            return Expr::Literal {
                value: Object::RNull,
            };
        }

        if Parser::match_tokens(tokens, current, vec![TokenType::String, TokenType::Number]) {
            let literal: &Object = &Parser::get_previous(tokens, current).literal;
            return Expr::Literal {
                value: literal.clone(),
            };
        }

        if Parser::match_tokens(tokens, current, vec![TokenType::Identifier]) {
            let name: &Token = &Parser::get_previous(tokens, current);
            return Expr::Variable { name };
        }

        if Parser::match_tokens(tokens, current, vec![TokenType::LeftParen]) {
            let expr: Expr = Parser::expression(tokens, current);

            Parser::consume(
                tokens,
                current,
                TokenType::RightParen,
                "Expect ')' after expression.",
            );

            return Expr::Grouping {
                expr: Box::new(expr),
            };
        }

        panic!("Expect expression");
    }

    // Helpers
    fn match_tokens(tokens: &Vec<Token>, current: &mut i16, target: Vec<TokenType>) -> bool {
        for t_type in target {
            if Parser::check(tokens, current, t_type) {
                Parser::advance(tokens, current);
                return true;
            }
        }
        false
    }

    fn check(tokens: &Vec<Token>, current: &mut i16, t_type: TokenType) -> bool {
        if Parser::is_at_end(tokens, current) {
            return false;
        }
        Parser::peek(tokens, current).token_type == t_type
    }

    fn advance<'a>(tokens: &'a Vec<Token>, current: &mut i16) -> &'a Token {
        if !Parser::is_at_end(tokens, current) {
            *current += 1;
        }
        Parser::get_previous(tokens, current)
    }

    fn is_at_end(tokens: &Vec<Token>, current: &mut i16) -> bool {
        Parser::peek(tokens, current).token_type == TokenType::Eof
    }

    fn peek<'a>(tokens: &'a Vec<Token>, current: &mut i16) -> &'a Token {
        tokens.get(*current as usize).unwrap()
    }

    fn get_previous<'a>(tokens: &'a Vec<Token>, current: &i16) -> &'a Token {
        let prev: usize = (current - 1) as usize;
        tokens.get(prev).unwrap()
    }

    fn consume<'a>(
        tokens: &'a Vec<Token>,
        current: &mut i16,
        t_type: TokenType,
        message: &str,
    ) -> &'a Token {
        if Parser::check(tokens, current, t_type) {
            return Parser::advance(tokens, current);
        }

        panic!("{}", message);
    }
}
