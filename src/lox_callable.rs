use crate::{interpreter::Interpreter, object::Object};

#[derive(Debug, Clone)]
pub enum LoxCallable {
    LoxFunction { arity: Box<dyn Fn() -> usize>, runner: Fn },
}

impl LoxCallable {
    pub fn run(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Object {
        match self {
            Self::LoxFunction { arity, runner } => self.run(interpreter, arguments),
        }
    }

    pub fn arity(&self) -> usize {
        match self {
            _ => 1,
        }
    }
}
