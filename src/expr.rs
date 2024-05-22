use crate::{object::Object, token::Token};

#[derive(Clone, Debug)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Self>,
    },
    Variable {
        name: Token,
    },
    Unary {
        right: Box<Self>,
        operator: Token,
    },
    Binary {
        left: Box<Self>,
        operator: Token,
        right: Box<Self>,
    },
    Grouping {
        expr: Box<Self>,
    },
    Literal {
        value: Object,
    },
    Logical {
        left: Box<Self>,
        right: Box<Self>,
        operator: Token,
    },
    Call {
        callee: Box<Self>,
        paren: Token,
        arguments: Vec<Self>
    }
}

impl Expr {
    pub fn accept<R, T: Visitor<R>>(&self, visitor: &mut T) -> R {
        visitor.run(self)
    }
}

pub trait Visitor<R> {
    fn run(&mut self, expr: &Expr) -> R;
}
