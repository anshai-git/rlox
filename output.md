[Function 
    { name: (Identifier, "fib", RString("fib")),
      params: [(Identifier, "n", RString("n"))], 
      body: [
            If { condition: Binary { left: Variable { name: (Identifier, "n", RString("n")) }, operator: (LessEqual, "<=", RString("")), right: Literal { value: RNumber(1.0) } },
            then_branch: Return { keyword: (Return, "return", RString("return")), value: Some(Variable { name: (Identifier, "n", RString("n")) }) },
            else_branch: None }, 
            Return { keyword: (Return, "return", RString("return")),
                     value: Some( Binary {
                        left: Call { callee: Variable { name: (Identifier, "fib", RString("fib")) }, paren: (RightParen, ")", RString("")), arguments: [Binary { left: Variable { name: (Identifier, "n", RString("n")) }, 
                        operator: (Minus, "-", RString("")), 
                        right: Literal { value: RNumber(2.0) } }] }, 
                        operator: (Plus, "+", RString("")), 
                        right: Call { callee: Variable { name: (Identifier, "fib", RString("fib")) }, paren: (RightParen, ")", RString("")), 
                        arguments: [Binary { left: Variable { name: (Identifier, "n", RString("n")) }, operator: (Minus, "-", RString("")), right: Literal { value: RNumber(1.0) } }] } }) }] },

            Block { statements: [Var { name: (Identifier, "i", RString("i")), initializer: Some(Literal { value: RNumber(0.0) }) }, While { condition: Binary { left: Variable { name: (Identifier, "i", RString("i")) }, operator: (Less, "<", RString("")), right: Literal { value: RNumber(20.0) } },
            body: Block { statements: [Block { statements: [Print { expr: Call { callee: Variable { name: (Identifier, "fib", RString("fib")) },
            paren: (RightParen, ")", RString("")), arguments: [Variable { name: (Identifier, "i", RString("i")) }] } }] }, Expression { expr: Assign { name: (Identifier, "i", RString("i")), value: Binary { left: Variable { name: (Identifier, "i", RString("i")) }, operator: (Plus, "+", RString("")), right: Literal { value: RNumber(1.0) } } } }] } }] }]


Expr::Variable name: (Identifier, "i", RString("i")), environment: Environment { enclosing: Some(Environment { enclosing: None, values: {"fib": RCallable()} }), values: {"i": RNumber(0.0)} }

Expr::Variable name: (Identifier, "fib", RString("fib")), environment: Environment { enclosing: Some(Environment { enclosing: Some(Environment { enclosing: Some(Environment { enclosing: None, values: {"fib": RCallable()} }), values: {"i": RNumber(0.0)} }), values: {} }), values: {} }

Expr::Variable name: (Identifier, "i", RString("i")), environment: Environment { enclosing: Some(Environment { enclosing: Some(Environment { enclosing: Some(Environment { enclosing: None, values: {"fib": RCallable()} }), values: {"i": RNumber(0.0)} }), values: {} }), values: {} }

Expr::Variable name: (Identifier, "n", RString("n")), environment: Environment { enclosing: Some(Environment { enclosing: None, values: {} }), values: {"n": RNumber(0.0)} }

Expr::Variable name: (Identifier, "n", RString("n")), environment: Environment { enclosing: Some(Environment { enclosing: None, values: {} }), values: {"n": RNumber(0.0)} }

Expr::Variable name: (Identifier, "fib", RString("fib")), environment: Environment { enclosing: Some(Environment { enclosing: Some(Environment { enclosing: Some(Environment { enclosing: None, values: {"fib": RCallable()} }), values: {"i": RNumber(0.0)} }), values: {} }), values: {} }

Expr::Variable name: (Identifier, "n", RString("n")),
               environment: Environment { enclosing: Some(Environment { enclosing: Some(Environment { enclosing: Some(Environment { enclosing: None, values: {"fib": RCallable()} }), values: {"i": RNumber(0.0)} }), values: {} }), values: {} }


thread 'main' panicked at src/interpreter.rs:217:17:
Undefined variable (Identifier, "n", RString("n"))
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
