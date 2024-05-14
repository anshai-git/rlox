use core::panic;

use crate::{
    environment::Environment, expr::Expr, object::Object, stmt::Stmt, token_type::TokenType,
};

pub struct Interpreter<'a> {
    pub program: &'a Vec<Stmt<'a>>,
    pub environment: Box<Environment>,
}

impl<'a> Interpreter<'a> {
    pub fn new(program: &'a Vec<Stmt>) -> Self {
        Interpreter {
            program,
            environment: Box::new(Environment::new()),
        }
    }

    pub fn interpret(&mut self) {
        for statement in self.program {
            self.execute(statement);
        }
    }

    fn execute(&mut self, stmt: &Stmt) {
        stmt.accept(self);
    }

    fn evaluate(&mut self, expr: &Expr) -> Object {
        expr.accept(self)
    }

    fn is_truthy(&self, object: &Object) -> Object {
        match object {
            Object::RNull => Object::RBoolean(false),
            Object::RBoolean(v) => Object::RBoolean(*v),
            _ => Object::RBoolean(true),
        }
    }

    fn check_number_operand(operand: &Object) -> f64 {
        if let Object::RNumber(n) = operand {
            return *n;
        }
        panic!("Operand must be a number.");
    }

    fn check_number_operands(left: &Object, right: &Object) -> (f64, f64) {
        if let (Object::RNumber(l), Object::RNumber(r)) = (&left, &right) {
            return (*l, *r);
        }
        panic!("Operands must be numbers.");
    }

    fn is_equal(left: &Object, right: &Object) -> bool {
        if let (Object::RNull, Object::RNull) = (&left, &right) {
            return true;
        }

        if let Object::RNull = &left {
            return false;
        }

        if let (Object::RNumber(l), Object::RNumber(r)) = (&left, &right) {
            return l == r;
        }

        if let (Object::RString(l), Object::RString(r)) = (&left, &right) {
            return l == r;
        }

        false
    }

    fn enter_scope(&mut self) {
        let new_environment = self.environment.new_enclosed();
        self.environment = Box::new(new_environment);
    }

    fn execute_block(&mut self, statements: &Vec<Stmt>) {
        self.enter_scope();

        for statement in statements {
            self.execute(statement);
        }

        if let Some(e) = self.environment.enclosing.clone() {
            self.environment = e;
        }
    }
}

impl crate::expr::Visitor<Object> for Interpreter<'_> {
    fn run(&mut self, expr: &Expr) -> Object {
        match expr {
            Expr::Literal { value } => value.clone(),
            Expr::Grouping { expr } => self.evaluate(expr),
            Expr::Unary { right, operator } => {
                let right: Object = self.evaluate(right);

                match operator.token_type {
                    TokenType::Bang => self.is_truthy(&right),
                    TokenType::Minus => {
                        let n = Interpreter::check_number_operand(&right);
                        return Object::RNumber(-n);
                    }
                    _ => panic!("Unexpected Token."),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left: Object = self.evaluate(left);
                let right: Object = self.evaluate(right);

                match operator.token_type {
                    TokenType::Minus => {
                        let (l, r) = Interpreter::check_number_operands(&left, &right);
                        Object::RNumber(l - r)
                    }
                    TokenType::Slash => {
                        let (l, r) = Interpreter::check_number_operands(&left, &right);
                        Object::RNumber(l / r)
                    }
                    TokenType::Star => {
                        let (l, r) = Interpreter::check_number_operands(&left, &right);
                        Object::RNumber(l * r)
                    }
                    TokenType::Plus => {
                        if let (Object::RNumber(l), Object::RNumber(r)) = (&left, &right) {
                            return Object::RNumber(l + r);
                        }

                        if let (Object::RString(l), Object::RString(r)) = (left, right) {
                            return Object::RString(l + r.as_str());
                        }

                        panic!("Operands must be numbers or strings")
                    }
                    TokenType::Greater => {
                        let (l, r) = Interpreter::check_number_operands(&left, &right);
                        Object::RBoolean(l > r)
                    }
                    TokenType::GreaterEqual => {
                        let (l, r) = Interpreter::check_number_operands(&left, &right);
                        Object::RBoolean(l >= r)
                    }
                    TokenType::Less => {
                        let (l, r) = Interpreter::check_number_operands(&left, &right);
                        Object::RBoolean(l < r)
                    }
                    TokenType::LessEqual => {
                        let (l, r) = Interpreter::check_number_operands(&left, &right);
                        Object::RBoolean(l <= r)
                    }
                    TokenType::BangEqual => Object::RBoolean(!Interpreter::is_equal(&left, &right)),
                    TokenType::EqualEqual => Object::RBoolean(Interpreter::is_equal(&left, &right)),
                    _ => panic!("Unexpected token."),
                }
            }
            Expr::Variable { name } => return self.environment.get(name).clone(),
            Expr::Assign { name, value } => {
                let v: Object = self.evaluate(value);
                self.environment.assign(name, v.clone());
                v
            }
            Expr::Logical {
                left,
                right,
                operator,
            } => {
                let left_result: Object = self.evaluate(left);

                if let TokenType::Or = operator.token_type {
                    if let Object::RBoolean(v @ true) = self.is_truthy(&left_result) {
                        return left_result;
                    }
                } else {
                    if let Object::RBoolean(v @ false) = self.is_truthy(&left_result) {
                        return left_result;
                    }
                }

                self.evaluate(right)
            }
        }
    }
}

impl crate::stmt::Visitor<()> for Interpreter<'_> {
    fn run(&mut self, stmt: &crate::stmt::Stmt) -> () {
        match stmt {
            crate::stmt::Stmt::Print { expr } => {
                let value: Object = self.evaluate(expr);
                match value {
                    Object::RNull => {
                        println!("null");
                    }
                    Object::RBoolean(v) => {
                        println!("{v}");
                    }
                    Object::RNumber(n) => {
                        println!("{n}");
                    }
                    Object::RString(s) => {
                        println!("{s}");
                    }
                }
            }
            crate::stmt::Stmt::Expression { expr } => {
                self.evaluate(expr);
            }
            crate::stmt::Stmt::Var { name, initializer } => {
                let mut value: Option<Object> = None;
                if initializer.is_some() {
                    value = Some(self.evaluate(&initializer.as_ref().unwrap()));
                }
                self.environment
                    .define(name.lexeme.clone(), value.unwrap_or(Object::RNull));
            }
            crate::stmt::Stmt::Block { statements } => {
                self.execute_block(statements);
            }
            crate::stmt::Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition: Object = self.evaluate(condition);
                if let Object::RBoolean(_v @ true) = self.is_truthy(&condition) {
                    self.execute(then_branch);
                } else if let Some(else_b) = *else_branch.clone() {
                    self.execute(&else_b);
                }
            }
        }
    }
}
