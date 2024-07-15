use crate::{expression::Expression, token::Token};

pub enum Statement {
    Print {
        value: Box<Expression>,
    },
    Block {
        statements: Vec<Self>,
    },
    Expression {
        expression: Box<Expression>,
    },
    Var {
        name: Token,
        initializer: Box<Option<Expression>>,
    },
}

pub trait StatementVisitor {
    fn visit_print_stmt(&mut self, stmt: &Statement);
    fn visit_expression_stmt(&mut self, stmt: &Statement);
    fn visit_var_stmt(&mut self, stmt: &Statement);
    fn visit_block_stmt(&mut self, stmt: &mut Statement);
}

impl Statement {
    pub fn accept<T: StatementVisitor>(&mut self, visitor: &mut T) -> () {
        match self {
            Statement::Print { .. } => visitor.visit_print_stmt(self),
            Statement::Expression { .. } => visitor.visit_expression_stmt(self),
            Statement::Var { .. } => visitor.visit_var_stmt(self),
            Statement::Block { .. } => visitor.visit_block_stmt(self),
        }
    }
}
