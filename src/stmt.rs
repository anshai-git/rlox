use crate::{expr::Expr, token::Token};

#[derive(Clone)]
pub enum Stmt<'a> {
    Expression {
        expr: Box<Expr<'a>>,
    },
    Print {
        expr: Box<Expr<'a>>,
    },
    Var {
        name: Token,
        initializer: Option<Expr<'a>>,
    },
    Block {
        statements: Vec<Self>
    },
    If {
        condition: Box<Expr<'a>>,
        then_branch: Box<Self>,
        else_branch: Box<Option<Self>>,
    }
}

impl Stmt<'_> {
    pub fn accept<R, T: Visitor<R>>(&self, visitor: &mut T) -> R {
        visitor.run(self)
    }
}

pub trait Visitor<R> {
    fn run(&mut self, stmt: &Stmt) -> R;
}
