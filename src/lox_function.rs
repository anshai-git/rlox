use std::collections::HashMap;

use crate::{
    environment::Environment,
    lox_callable::{LoxCallable, LoxCallableClone},
    object::Object,
    stmt::Stmt,
};

#[derive(Clone)]
pub struct LoxFunction {
    declaration: Stmt,
    closure: Environment,
}

impl LoxFunction {
    pub fn new(declaration: Stmt, closure: Environment) -> Self {
        Self {
            declaration,
            closure,
        }
    }
}

impl<'a> LoxCallable for LoxFunction {
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
            let mut environment = Environment::new(Some(Box::new(self.closure.clone())));

            for (index, param) in params.iter().enumerate() {
                environment.define(
                    params.get(index).unwrap().lexeme.clone(),
                    arguments.get(index).unwrap().clone(),
                );
            }

            if let Err(return_value) = interpreter.execute_block_2(&body, environment) {
                return return_value.value;
            }
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
