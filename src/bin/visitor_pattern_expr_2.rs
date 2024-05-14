enum Expr<'a> {
    Unary {
        right: &'a Self,
        operator: String,
    },
    Binary {
        left: &'a Self,
        operator: String,
        right: &'a Self,
    },
    Grouping {
        expr: &'a Self,
    },
    Literal {
        value: &'a str,
    },
}

impl Expr<'_> {
    fn accept<R, T: Visitor<R>>(&self, visitor: &T) -> R {
        visitor.run(self)
    }
}

trait Visitor<R> {
    fn run(&self, expr: &Expr) -> R;
}

fn main() {
    
}
