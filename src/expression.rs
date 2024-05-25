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
    fn visit_binary_expression(&self, expr: &Expression);
    fn visit_grouping_expression(&self, expr: &Expression);
    fn visit_literal_expression(&self, expr: &Expression);
    fn visit_unary_expression(&self, expr: &Expression);
}

impl Expression {
    pub fn accept<T: ExpressionVisitor>(&self, visitor: T) {
        match self {
            Binary => visitor.visit_binary_expression(self),
            Grouping => visitor.visit_grouping_expression(self),
            Unary => visitor.visit_unary_expression(self),
            Literal => visitor.visit_literal_expression(self),
            _ => println!(""),
        }
    }
}
