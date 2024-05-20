use std::collections::HashMap;

use crate::{
    environment::Environment,
    lox_callable::{LoxCallable, LoxCallableClone},
    object::Object,
    stmt::Stmt,
};

#[derive(Clone)]
pub struct LoxFunction<'a> {
    declaration: Stmt<'a>,
}

impl<'a> LoxFunction<'a> {
    pub fn new(declaration: Stmt<'a>) -> Self {
        Self { declaration }
    }
}

impl<'a> LoxCallable for LoxFunction<'static> {
    fn arity(&self) -> usize {
        if let Stmt::Function { params, .. } = self.declaration.clone() {
            return params.len();
        }
        panic!("Unexpected error.");
    }

    fn run(
        &self,
        interpreter: &mut crate::interpreter::Interpreter,
        arguments: Vec<Object>,
    ) -> Object {
        if let Stmt::Function { params, body, .. } = self.declaration.clone() {
            let mut environment = Environment {
                enclosing: Some(Box::new(Environment {
                    enclosing: None,
                    values: interpreter.globals.values.clone(),
                })),
                values: HashMap::new(),
            };
            for (index, param) in params.iter().enumerate() {
                environment.define(
                    params.get(index).unwrap().lexeme.clone(),
                    arguments.get(index).unwrap().clone(),
                );
            }
            interpreter.execute_block_2(&body, environment);
        }

        Object::RNull
    }

    fn name(&self) -> Object {
        if let Stmt::Function { name, .. } = self.declaration.clone() {
            return Object::RString(name.lexeme.clone());
        }
        Object::RString("unknown".to_string())
    }
}
