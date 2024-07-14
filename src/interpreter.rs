use crate::{
    expression::{Expression, ExpressionVisitor},
    object::Object,
    token_type::TokenType,
};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interpret(&self, expr: &Expression) {
        let val: Object = Interpreter::evaluate(expr, self);
        println!("{:?}", val);
    }

    pub fn evaluate<T: ExpressionVisitor>(expr: &Expression, visitor: &T) -> Object {
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

impl ExpressionVisitor for Interpreter {
    fn visit_literal_expression(&self, expr: &Expression) -> Object {
        println!("visit literal >> {:?}", expr);
        if let Expression::Literal { value } = expr {
            value.clone()
        } else {
            panic!("Expected Expression::Literal")
        }
    }

    fn visit_grouping_expression(&self, expr: &Expression) -> Object {
        if let Expression::Grouping { expression } = expr {
            Interpreter::evaluate(expression, self)
        } else {
            panic!("Expected Expression::Grouping")
        }
    }

    fn visit_binary_expression(&self, expr: &Expression) -> Object {
        println!("{:?}", expr);
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

    fn visit_unary_expression(&self, expr: &Expression) -> Object {
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
