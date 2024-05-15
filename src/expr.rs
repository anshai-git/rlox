use crate::{object::Object, token::Token};

#[derive(Clone, Debug)]
pub enum Expr<'a> {
    Assign {
        name: &'a Token,
        value: Box<Self>,
    },
    Variable {
        name: &'a Token,
    },
    Unary {
        right: Box<Self>,
        operator: &'a Token,
    },
    Binary {
        left: Box<Self>,
        operator: &'a Token,
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
        operator: &'a Token,
    },
}

impl Expr<'_> {
    pub fn accept<R, T: Visitor<R>>(&self, visitor: &mut T) -> R {
        visitor.run(self)
    }
}

pub trait Visitor<R> {
    fn run(&mut self, expr: &Expr) -> R;
}
