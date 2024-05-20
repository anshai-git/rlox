use crate::expr::{Expr, Visitor};

pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter {}
    }

    fn paranthesize(&mut self, name: &str, exprs: Vec<&Expr>) -> String {
        let mut builder = String::new();

        builder.push_str("(");
        builder.push_str(name);
        for expr in exprs {
            builder.push_str(" ");
            builder.push_str(expr.accept(self).as_str());
        }
        builder.push_str(")");

        builder.to_string()
    }
}

impl Visitor<String> for AstPrinter {
    fn run(&mut self, _expr: &Expr) -> String {
        "not implemented".to_string()
        // match expr {
        //     Expr::Unary { right, operator } => self.paranthesize(&operator.lexeme, vec![right]),
        //     Expr::Binary {
        //         operator,
        //         left,
        //         right,
        //     } => self.paranthesize(&operator.lexeme, vec![left, right]),
        //     Expr::Grouping { expr } => self.paranthesize("group", vec![expr]),
        //     Expr::Literal { value } => match value  {
        //        Object::RString(val) => val.clone(),
        //        Object::RNumber(val) => val.to_string(),
        //        Object::RNull => "null".to_string(),
        //        Object::RBoolean(v) => v.to_string()
        //     }
        // }
    }
}
