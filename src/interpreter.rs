use core::panic;
use std::error::Error;

use crate::{
    environment::{self, Environment},
    expr::Expr,
    lox_callable::{LoxCallable, LoxCallableClone},
    lox_function::LoxFunction,
    lox_return::LoxReturn,
    native_functions::Clock,
    object::Object,
    stmt::Stmt,
    token::Token,
    token_type::TokenType,
};

pub struct Interpreter<'a> {
    pub program: &'a Vec<Stmt>,
    pub environment: Box<Environment>,
    pub globals: Box<Environment>,
}

impl<'a> Interpreter<'a> {
    pub fn new(program: &'a Vec<Stmt>) -> Self {
        let mut globals = Box::new(Environment::new(None));
        globals.define(
            "clock".to_string(),
            Object::RCallable(Box::new(Clock::new())),
        );

        Interpreter {
            program,
            environment: Box::new(Environment::new(None)),
            globals,
        }
    }

    pub fn interpret(&mut self) {
        for statement in self.program {
            self.execute(statement);
        }
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), LoxReturn> {
        stmt.accept(self)
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
        let new_environment = Environment::new(Some(self.environment.clone()));
        self.environment = Box::new(new_environment);
    }

    pub fn execute_block(&mut self, statements: &Vec<Stmt>) {
        self.enter_scope();

        for statement in statements {
            self.execute(statement);
        }

        if let Some(e) = self.environment.enclosing.clone() {
            self.environment = e;
        }
    }

    pub fn execute_block_2(&mut self, statements: &Vec<Stmt>, env: Environment) -> Result<(), LoxReturn> {
        let previous = self.environment.clone();

        self.environment = Box::new(env);

        for statement in statements {
            if let Stmt::Return { keyword, value } = statement {
                self.environment = previous;
                return Err(LoxReturn::new(self.evaluate(&value.clone().unwrap())));
            }
            self.execute(statement);
        }

        self.environment = previous;
        Ok(())
    }
}

impl crate::expr::Visitor<Object> for Interpreter<'_> {
    fn run(&mut self, expr: &Expr) -> Object {
        match expr {
            Expr::Literal { value } => {
                println!("[Expr::Literal]: Value: {:?}, \n>> Environment: {:?}\n", value, self.environment);
                return value.clone();
            }
            Expr::Grouping { expr } => {
                println!("[Expr::Grouping]: Expr: {:?}, \n>> Environment: {:?}\n", expr, self.environment);
                return self.evaluate(expr);
            }
            Expr::Unary { right, operator } => {
                println!("[Expr::Unary]: Right: {:?}, Operator: {:?}, \n>> Environment: {:?}]\n", right, operator, self.environment);
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
            Expr::Binary { left, operator, right } => {
                println!("[Expr::Binary]: Left {:?}, Right: {:?}, Operator: {:?}, \n>> Environment: {:?}\n", left, right, operator, self.environment);
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
            Expr::Variable { name } => {
                println!("[Expr::Variable]: Name: {:?}, \n>> Environment: {:?}\n", name, self.environment);

                if let Some(variable) = self.environment.get(name).clone() {
                    return variable.clone();
                }

                if let Some(global_variable) = self.globals.get(name).clone() {
                    return global_variable.clone();
                }

                panic!("Undefined variable {:?}", name);
            }
            Expr::Assign { name, value } => {
                println!("[Expr::Assign]: Name: {:?}, Value: {:?},\n>> Environment: {:?}\n", name, value, self.environment);
                let v: Object = self.evaluate(value);
                self.environment.assign(name, v.clone());
                v
            }
            Expr::Logical { left, right, operator } => {
                println!("[Expr::Logical]: Left: {:?}, Right: {:?}, Operator: {:?},\n>> Environment: {:?}\n", left, right, operator, self.environment);
                let left_result: Object = self.evaluate(left);

                if let TokenType::Or = operator.token_type {
                    if let Object::RBoolean(_v @ true) = self.is_truthy(&left_result) {
                        return left_result;
                    }
                } else {
                    if let Object::RBoolean(_v @ false) = self.is_truthy(&left_result) {
                        return left_result;
                    }
                }

                self.evaluate(right)
            }
            Expr::Call { callee, arguments, .. } => {
                println!("[Expr::Call]: Callee: {:?}, Arguments: {:?}, \n>> Environment: {:?}\n", callee, arguments, self.environment);
                let callee_object: Object = self.evaluate(callee);

                let mut argument_values = Vec::new();
                for arg in arguments {
                    argument_values.push(self.evaluate(arg));
                }

                if let Object::RCallable(function) = callee_object {
                    if function.arity() != arguments.len() {
                        panic!(
                            "Expected {} arguments, but got {}.",
                            function.arity(),
                            arguments.len()
                        );
                    }

                    function.run(self, argument_values)
                } else {
                    panic!("Can only call functions and classes");
                }
            }
        }
    }
}

impl crate::stmt::Visitor<LoxReturn> for Interpreter<'_> {
    fn run(&mut self, stmt: &crate::stmt::Stmt) -> Result<(), LoxReturn> {
        match stmt {
            crate::stmt::Stmt::Print { expr } => {
                println!("[Stmt::Print]: Expr: {:?},\n>> Environment: {:?}\n", expr, self.environment);
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
                    Object::RCallable(f) => {
                        // if let Some Lox
                        println!("print not implemented for functions");
                    }
                }
                Ok(())
            }
            crate::stmt::Stmt::Expression { expr } => {
                println!("[Stmt::Expression]: Expr: {:?},\n>> Environment: {:?}\n", expr, self.environment);
                self.evaluate(expr);
                Ok(())
            }
            crate::stmt::Stmt::Var { name, initializer } => {
                println!("[Stmt::Var]: Name: {:?}, Initializer: {:?},\n>> Environment: {:?}\n", name, initializer, self.environment);
                let mut value: Option<Object> = None;
                if initializer.is_some() {
                    value = Some(self.evaluate(&initializer.as_ref().unwrap()));
                }
                self.environment.define(name.lexeme.clone(), value.unwrap_or(Object::RNull));
                Ok(())
            }
            crate::stmt::Stmt::Block { statements } => {
                println!("[Stmt::Block]: Statements: {:?},\n>> Environment: {:?}\n", statements, self.environment);
                let env = Environment::new(Some(self.environment.clone()));
                self.execute_block_2(statements, env);
                Ok(())
            }
            crate::stmt::Stmt::If { condition, then_branch, else_branch } => {
                println!("[Stmt::If]: Condition: {:?}, ThenBranch: {:?}, ElseBranch: {:?},\n>> Environment: {:?}\n", condition, then_branch, else_branch, self.environment);
                let condition: Object = self.evaluate(condition);

                if let Object::RBoolean(_v @ true) = self.is_truthy(&condition) {
                    self.execute(then_branch);
                } else if let Some(else_b) = *else_branch.clone() {
                    self.execute(&else_b);
                }

                Ok(())
            }
            crate::stmt::Stmt::While { condition, body } => {
                println!("[Stmt::While]: Condition: {:?}, Body: {:?},\n>> Environment: {:?}\n", condition, body, self.environment);
                let mut condition_result: Object = self.evaluate(&condition);

                while let Object::RBoolean(_v @ true) = self.is_truthy(&condition_result) {
                    println!("Condition: {:?}, Result: {:?}, Env: {:?}", condition, condition_result, self.environment);
                    self.execute(body);
                    condition_result = self.evaluate(&condition);
                }

                Ok(())
            }
            crate::stmt::Stmt::Function { name, params, body } => {
                println!("[Stmt::Function]: Name: {:?}, Params: {:?}, Body: {:?},\n>> Environment: {:?}\n", name, params, body, self.environment);
                let function_stmt = Stmt::Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                };

                let function = Box::new(LoxFunction::new(function_stmt, *self.environment.clone()));
                self.environment
                    .define(name.lexeme.clone(), Object::RCallable(function));

                Ok(())
            }
            crate::stmt::Stmt::Return { keyword, value } => {
                println!("[Stmt::Return]: Keyword: {:?}, Value: {:?},\n>> Environment: {:?}\n", keyword, value, self.environment);
                if let Some(value_expr) = *value.clone() {
                    return Err(LoxReturn::new(self.evaluate(&value_expr)));
                }
                Err(LoxReturn::new(Object::RNull))
            }
        }
    }
}
