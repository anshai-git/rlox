use std::time::{SystemTime, UNIX_EPOCH};

use crate::lox_callable::LoxCallable;

#[derive(Clone)]
pub struct Clock;

impl Clock {
    pub fn new() -> Self {
        Clock {}
    }
}

impl LoxCallable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn run(
        &self,
        _interpreter: &mut crate::interpreter::Interpreter,
        _arguments: Vec<crate::object::Object>,
    ) -> crate::object::Object {
        let start = SystemTime::now();

        // Calculate the duration since the UNIX_EPOCH
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        // Convert the duration to milliseconds
        let in_ms = since_the_epoch.as_millis();

        crate::object::Object::RNumber(in_ms as f64)
    }

    fn name(&self) -> crate::object::Object {
        crate::object::Object::RString("<builtin fn clock>".to_string())
    }
}
