use crate::lox_callable::LoxCallable;

#[derive(Debug, Clone)]
pub enum Object {
    RString(String),
    RNumber(f64),
    RBoolean(bool),
    RNull,
    RCallable(Box<dyn LoxCallable>),
}

impl Object {
    pub fn empty() -> Self {
        Object::RString(String::new())
    }
}
