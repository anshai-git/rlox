use crate::{expr::Expr, token::Token};

#[derive(Clone, Debug)]
pub enum Stmt {
    Expression {
        expr: Box<Expr>,
    },
    Print {
        expr: Box<Expr>,
    },
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
    Block {
        statements: Vec<Self>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Self>,
        else_branch: Box<Option<Self>>,
    },
    While {
        condition: Box<Expr>,
        body: Box<Self>,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Self>,
    },
    Return {
        keyword: Token,
        value: Box<Option<Expr>>
    }
}

impl Stmt {
    pub fn accept<R, T: Visitor<R>>(&self, visitor: &mut T) -> Result<(), R> {
        visitor.run(self)
    }
}

pub trait Visitor<R> {
    fn run(&mut self, stmt: &Stmt) -> Result<(), R>;
}
