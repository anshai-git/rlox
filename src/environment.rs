use core::panic;
use std::collections::HashMap;

use crate::{object::Object, token::Token};

#[derive(Clone, Debug)]
pub struct Environment {
    pub enclosing: Option<Box<Self>>,
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn new_enclosed(&mut self) -> Self {
        Environment {
            enclosing: Some(Box::new(self.clone())),
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, key: String, value: Object) {
        self.values.insert(key, value);
    }

    pub fn get(&self, name: &Token) -> &Object {
        if let Some(v) = self.values.get(&name.lexeme) {
            return v;
        }

        if let Some(enclosing) = &self.enclosing {
            return enclosing.get(name);
        }

        panic!("Undefined variable");
    }

    pub fn assign(&mut self, name: &Token, value: Object) {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return;
        }

        if let Some(enclosing) = self.enclosing.as_mut() {
            enclosing.assign(name, value);
            return;
        }

        panic!("Assign to undefined variable");
    }
}
