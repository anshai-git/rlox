use crate::object::Object;

pub struct LoxReturn {
    pub value: Object,
}

impl LoxReturn {
    pub fn new(value: Object) -> Self {
        Self { value }
    }
}
