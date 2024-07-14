use std::fmt::Binary;

use crate::{object::Object, token::Token};

#[derive(Debug)]
pub enum Expression {
    Binary {
        left: Box<Self>,
        right: Box<Self>,
        operator: Token,
    },
    Grouping {
        expression: Box<Self>,
    },
    Literal {
        value: Object,
    },
    Unary {
        operator: Token,
        right: Box<Self>,
    },
}

pub trait ExpressionVisitor {
    fn visit_binary_expression(&self, expr: &Expression) -> Object;
    fn visit_grouping_expression(&self, expr: &Expression) -> Object;
    fn visit_literal_expression(&self, expr: &Expression) -> Object;
    fn visit_unary_expression(&self, expr: &Expression) -> Object;
}

impl Expression {
    pub fn accept<T: ExpressionVisitor>(&self, visitor: &T) -> Object {
        println!("accept >> {:?}", self);
        match self {
            Expression::Binary { .. } => visitor.visit_binary_expression(self),
            Expression::Grouping { .. } => visitor.visit_grouping_expression(self),
            Expression::Unary { .. } => visitor.visit_unary_expression(self),
            Expression::Literal { .. } => visitor.visit_literal_expression(self),
        }
    }
}
