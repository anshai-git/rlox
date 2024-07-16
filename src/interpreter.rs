use std::borrow::BorrowMut;

use crate::{
    environment::Environment,
    expression::{Expression, ExpressionVisitor},
    object::Object,
    statement::{Statement, StatementVisitor},
    token_type::TokenType,
};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, mut statements: Vec<Statement>) {
        for statement in statements.iter_mut() {
            Interpreter::execute(statement, self);
        }
    }

    pub fn execute_block(&mut self, mut statements: &mut Vec<Statement>, environment: Environment) {
        let previous: Environment = self.environment.clone();
        self.environment = environment;
        for statement in statements.iter_mut() {
            Interpreter::execute(statement, self);
        }
        self.environment = previous;
    }

    pub fn execute<T: StatementVisitor>(stmt: &mut Statement, visitor: &mut T) -> () {
        stmt.accept(visitor);
    }

    pub fn evaluate<T: ExpressionVisitor>(expr: &Expression, visitor: &mut T) -> Object {
        expr.accept(visitor)
    }

    pub fn is_truthy(value: Object) -> bool {
        match value {
            Object::Null => false,
            Object::Boolean(b) => b,
            _ => true,
        }
    }

    pub fn is_equal(left: Object, right: Object) -> bool {
        match (left, right) {
            (Object::Null, Object::Null) => true,
            (Object::Null, _) => false,
            (Object::Boolean(l), Object::Boolean(r)) => l == r,
            (Object::Number(l), Object::Number(r)) => l == r,
            (Object::String(l), Object::String(r)) => l == r,
            _ => false,
        }
    }
}

impl StatementVisitor for Interpreter {
    fn visit_while_stmt(&mut self, stmt: &mut Statement) {
        if let Statement::While { condition, body } = stmt {
            while Interpreter::is_truthy(Interpreter::evaluate(condition, self)) {
                Interpreter::execute(body, self);
            }
        } else {
            panic!("Expected Statement::While")
        }
    }

    fn visit_if_stmt(&mut self, stmt: &mut Statement) {
        if let Statement::If {
            condition,
            then_branch,
            else_branch,
        } = stmt
        {
            let condition_expr_result: Object = Interpreter::evaluate(condition, self);
            if Interpreter::is_truthy(condition_expr_result) {
                Interpreter::execute(then_branch, self);
            } else if let Some(ref mut else_branch_stmt) = else_branch.as_mut() {
                Interpreter::execute(else_branch_stmt, self);
            }
        } else {
            panic!("Expected Statement::If")
        }
    }

    fn visit_block_stmt(&mut self, stmt: &mut Statement) {
        if let Statement::Block { ref mut statements } = stmt {
            self.execute_block(
                statements,
                Environment::with_enclosing(self.environment.clone()),
            );
        } else {
            panic!("Expected Statement::Block")
        }
    }

    fn visit_var_stmt(&mut self, stmt: &Statement) {
        if let Statement::Var { name, initializer } = stmt {
            let value: Object = match initializer.as_ref() {
                Some(ref expr) => Interpreter::evaluate(expr, self),
                None => Object::Null,
            };

            self.environment.define(name.lexeme.clone(), value);
        } else {
            panic!("Expected Statement::Var")
        }
    }

    fn visit_print_stmt(&mut self, stmt: &Statement) -> () {
        if let Statement::Print { value } = stmt {
            let value: Object = Interpreter::evaluate(value, self);
            println!("{:?}", value);
        } else {
            panic!("Expected Statement::Print")
        }
    }

    fn visit_expression_stmt(&mut self, stmt: &Statement) -> () {
        if let Statement::Expression { expression } = stmt {
            Interpreter::evaluate(expression, self);
        } else {
            panic!("Expected Statement::Expression")
        }
    }
}

impl ExpressionVisitor for Interpreter {
    fn visit_logical_expression(&mut self, expr: &Expression) -> Object {
        if let Expression::Logical {
            left,
            operator,
            right,
        } = expr
        {
            let left_expr_result: Object = Interpreter::evaluate(left, self);
            if operator.token_type == TokenType::Or {
                if Interpreter::is_truthy(left_expr_result.clone()) {
                    return left_expr_result;
                }
            } else {
                if !Interpreter::is_truthy(left_expr_result.clone()) {
                    return left_expr_result;
                }
            }

            Interpreter::evaluate(right, self)
        } else {
            panic!("Expected Expression::Logical");
        }
    }

    fn visit_assign_expression(&mut self, expr: &Expression) -> Object {
        if let Expression::Assign { name, value } = expr {
            let value_expr_result: Object = Interpreter::evaluate(value, self);
            self.environment.assign(name.clone(), value_expr_result.clone());
            return value_expr_result;
        } else {
            panic!("Expected Expression::Assign");
        }
    }

    fn visit_variable_expression(&mut self, expr: &Expression) -> Object {
        if let Expression::Variable { name } = expr {
            self.environment.get(name.clone())
        } else {
            panic!("Expected Expression::Variable")
        }
    }

    fn visit_literal_expression(&self, expr: &Expression) -> Object {
        // println!("visit literal >> {:?}", expr);
        if let Expression::Literal { value } = expr {
            value.clone()
        } else {
            panic!("Expected Expression::Literal")
        }
    }

    fn visit_grouping_expression(&mut self, expr: &Expression) -> Object {
        if let Expression::Grouping { expression } = expr {
            Interpreter::evaluate(expression, self)
        } else {
            panic!("Expected Expression::Grouping")
        }
    }

    fn visit_binary_expression(&mut self, expr: &Expression) -> Object {
        // println!("{:?}", expr);
        if let Expression::Binary {
            left,
            right,
            operator,
        } = expr
        {
            let left: Object = Interpreter::evaluate(left, self);
            let right: Object = Interpreter::evaluate(right, self);

            match operator.token_type {
                TokenType::Greater => match (left, right) {
                    (Object::Number(l), Object::Number(r)) => Object::Boolean(l > r),
                    _ => panic!("Object tpye not supported for operatrion."),
                },
                TokenType::GreaterEqual => match (left, right) {
                    (Object::Number(l), Object::Number(r)) => Object::Boolean(l >= r),
                    _ => panic!("Object tpye not supported for operatrion."),
                },
                TokenType::Less => match (left, right) {
                    (Object::Number(l), Object::Number(r)) => Object::Boolean(l < r),
                    _ => panic!("Object tpye not supported for operatrion."),
                },
                TokenType::LessEqual => match (left, right) {
                    (Object::Number(l), Object::Number(r)) => Object::Boolean(l <= r),
                    _ => panic!("Object tpye not supported for operatrion."),
                },
                TokenType::BangEqual => Object::Boolean(!Interpreter::is_equal(left, right)),
                TokenType::EqualEqual => Object::Boolean(Interpreter::is_equal(left, right)),
                TokenType::Minus => {
                    if let (Object::Number(l), Object::Number(r)) = (left, right) {
                        Object::Number(l - r)
                    } else {
                        panic!("Object tpye not supported for operatrion.")
                    }
                }
                TokenType::Plus => match (left, right) {
                    (Object::Number(l), Object::Number(r)) => Object::Number(l + r),
                    (Object::String(l), Object::String(r)) => Object::String(l + &r),
                    _ => panic!("Object tpye not supported for operatrion."),
                },
                TokenType::Slash => {
                    if let (Object::Number(l), Object::Number(r)) = (left, right) {
                        Object::Number(l / r)
                    } else {
                        panic!("Object tpye not supported for operatrion.")
                    }
                }
                TokenType::Star => {
                    if let (Object::Number(l), Object::Number(r)) = (left, right) {
                        Object::Number(l * r)
                    } else {
                        panic!("Object tpye not supported for operatrion.")
                    }
                }
                _ => panic!("Operator not supported."),
            }
        } else {
            panic!("Expected Expression::Binary")
        }
    }

    fn visit_unary_expression(&mut self, expr: &Expression) -> Object {
        if let Expression::Unary { operator, right } = expr {
            let right: Object = Interpreter::evaluate(right, self);
            match operator.token_type {
                TokenType::Minus => {
                    if let Object::Number(num) = right {
                        Object::Number(-num)
                    } else {
                        panic!("Operand not supported")
                    }
                }
                TokenType::Bang => Object::Boolean(Interpreter::is_truthy(right)),
                _ => panic!("Operator not supported"),
            }
        } else {
            panic!("Expected Expression::Grouping")
        }
    }
}
