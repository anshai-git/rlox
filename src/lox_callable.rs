use std::fmt::Debug;

use crate::{interpreter::Interpreter, object::Object};

pub trait LoxCallable: LoxCallableClone {
    fn run(&self, interpreter: &mut Interpreter, arguments: Vec<Object>) -> Object;
    fn arity(&self) -> usize;
    fn name(&self) -> Object;
}

pub trait LoxCallableClone {
    fn clone_box(&self) -> Box<dyn LoxCallable>;
}

impl<T> LoxCallableClone for T
where
    T: 'static + LoxCallable + Clone,
{
    fn clone_box(&self) -> Box<dyn LoxCallable> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn LoxCallable> {
    fn clone(&self) -> Box<dyn LoxCallable> {
        self.clone_box()
    }
}

impl Debug for Box<dyn LoxCallable> {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
