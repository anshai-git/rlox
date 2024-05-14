trait Expr<R> {
    fn accept<T: Visitor<R>>(&mut self, visitor: &mut T);
}

trait Visitor<R> {
    fn visit_binary_expr(&mut self, expr: &mut Binary) -> R;
    fn visit_grouping_expr(&mut self, expr: &mut Grouping) -> R;
    fn visit_unary_expr(&mut self, expr: &mut Unary) -> R;
    fn visit_literal_expr(&mut self, expr: &mut Literal) -> R;
}

struct Unary {
    lexeme: String,
}

struct Binary {
    lexeme: String,
}

struct Grouping {
    lexeme: String,
}

struct Literal {
    lexeme: String,
}

impl Unary {
    fn get_lexeme(&self) -> &String {
        &self.lexeme
    }
}

impl Grouping {
    fn get_lexeme(&self) -> &String {
        &self.lexeme
    }
}

impl Binary {
    fn get_lexeme(&self) -> &String {
        &self.lexeme
    }
}

impl Literal {
    fn get_lexeme(&self) -> &String {
        &self.lexeme
    }
}


impl<R> Expr<R> for Unary {
    fn accept<T: Visitor<R>>(&mut self, visitor: &mut T) {
        visitor.visit_unary_expr(self);
    }
}

impl<R> Expr<R> for Binary {
    fn accept<T: Visitor<R>>(&mut self, visitor: &mut T) {
        visitor.visit_binary_expr(self);
    }
}

impl<R> Expr<R> for Grouping {
    fn accept<T: Visitor<R>>(&mut self, visitor: &mut T) {
        visitor.visit_grouping_expr(self);
    }
}

impl<R> Expr<R> for Literal {
    fn accept<T: Visitor<R>>(&mut self, visitor: &mut T) {
        visitor.visit_literal_expr(self);
    }
}














































struct Interpreter;

impl Visitor<String> for Interpreter {
    fn visit_unary_expr(&mut self, expr: &mut Unary) -> String {
        let lexeme = expr.get_lexeme();
        println!("unary lexeme: {}", lexeme);
        "not implemented".to_string()
    }

    fn visit_binary_expr(&mut self, expr: &mut Binary) -> String {
        let lexeme = expr.get_lexeme();
        println!("binary lexeme: {}", lexeme);
        "not implemented".to_string()
    }

    fn visit_grouping_expr(&mut self, expr: &mut Grouping) -> String {
        let lexeme = expr.get_lexeme();
        println!("grouping lexeme: {}", lexeme);
        "not implemented".to_string()
    }

    fn visit_literal_expr(&mut self, expr: &mut Literal) -> String {
        let lexeme = expr.get_lexeme();
        println!("literal lexeme: {}", lexeme);
        "not implemented".to_string()
    }
}

fn main() {
    let mut unary: Unary = Unary {
        lexeme: "test_unary_lexeme".to_string(),
    };

    let mut binary: Binary = Binary {
        lexeme: "test_binary_lexeme".to_string(),
    };

    let mut grouping: Grouping = Grouping {
        lexeme: "test_grouping_lexeme".to_string(),
    };

    let mut literal: Literal = Literal {
        lexeme: "test_literal_lexeme".to_string(),
    };

    let mut interpreter: Interpreter = Interpreter {};

    unary.accept(&mut interpreter);
    binary.accept(&mut interpreter);
    grouping.accept(&mut interpreter);
    literal.accept(&mut interpreter);
}
