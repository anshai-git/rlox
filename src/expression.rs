use std::fmt::Binary;

use crate::{object::Object, token::Token};

#[derive(Debug)]
pub enum Expression {
    Assign {
        name: Token,
        value: Box<Self>,
    },
    Binary {
        left: Box<Self>,
        right: Box<Self>,
        operator: Token,
    },
    Call {
        callee: Box<Self>,
        paren: Token,
        arguments: Vec<Self>,
    },
    Grouping {
        expression: Box<Self>,
    },
    Literal {
        value: Object,
    },
    Logical {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Unary {
        operator: Token,
        right: Box<Self>,
    },
    Variable {
        name: Token,
    },
}

pub trait ExpressionVisitor {
    fn visit_binary_expression(&mut self, expr: &Expression) -> Object;
    fn visit_grouping_expression(&mut self, expr: &Expression) -> Object;
    fn visit_literal_expression(&self, expr: &Expression) -> Object;
    fn visit_unary_expression(&mut self, expr: &Expression) -> Object;
    fn visit_variable_expression(&mut self, expr: &Expression) -> Object;
    fn visit_assign_expression(&mut self, expr: &Expression) -> Object;
    fn visit_logical_expression(&mut self, expr: &Expression) -> Object;
    fn visit_call_expression(&mut self, expre: &Expression) -> Object;
}

impl Expression {
    pub fn accept<T: ExpressionVisitor>(&self, visitor: &mut T) -> Object {
        match self {
            Expression::Binary { .. } => visitor.visit_binary_expression(self),
            Expression::Grouping { .. } => visitor.visit_grouping_expression(self),
            Expression::Unary { .. } => visitor.visit_unary_expression(self),
            Expression::Literal { .. } => visitor.visit_literal_expression(self),
            Expression::Variable { .. } => visitor.visit_variable_expression(self),
            Expression::Assign { .. } => visitor.visit_assign_expression(self),
            Expression::Logical { .. } => visitor.visit_logical_expression(self),
            Expression::Call { .. } => visitor.visit_call_expression(self),
        }
    }
}
