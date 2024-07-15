use std::collections::HashMap;

use crate::{object::Object, token::Token};

#[derive(Clone)]
pub struct Environment {
    enclosing: Box<Option<Self>>,
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            enclosing: Box::new(None),
            values: HashMap::new(),
        }
    }

    pub fn with_enclosing(enclosing: Environment) -> Self {
        Self {
            enclosing: Box::new(Some(enclosing)),
            values: HashMap::new(),
        }
    }

    pub fn assign(&mut self, name: Token, value: Object) -> () {
        if self.values.contains_key(name.lexeme.as_str()) {
            self.values.insert(name.lexeme.clone(), value.clone());
        }

        if let Some(ref mut enclosing) = *self.enclosing {
            return enclosing.assign(name, value);
        }

        panic!("Undefined variable");
    }

    // We don't check if a variable exists when we define it
    // It means we enabled to redefine variables
    pub fn define(&mut self, name: String, value: Object) -> () {
        self.values.insert(name, value);
    }

    pub fn get(&mut self, name: Token) -> Object {
        // TODO: An Object Exception type shou;ld be returned here for synchronization
        if self.values.contains_key(name.lexeme.as_str()) {
            return self.values.get(name.lexeme.as_str()).unwrap().to_owned();
        }

        if let Some(ref mut enclosing) = *self.enclosing {
            return enclosing.get(name);
        }

        panic!("Undefined Variable {:?}", name);
    }
}
