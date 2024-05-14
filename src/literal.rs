use std::fmt::Debug;

#[derive(Clone)]
pub struct Literal {
    pub number_value: Option<f64>,
    pub string_value: Option<String>,
}

impl Literal {
    pub fn new_string(value: String) -> Self {
        Literal {
            string_value: Some(value),
            number_value: None,
        }
    }

    pub fn new_number(value: f64) -> Self {
        Literal {
            string_value: None,
            number_value: Some(value),
        }
    }

    pub fn new_empty() -> Self {
        Literal {
            string_value: None,
            number_value: None,
        }
    }
}

impl Debug for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = f.debug_tuple("");

        if let Some(val) = self.number_value {
            text.field(&val);
        }

        if let Some(val) = &self.string_value {
            text.field(val);
        }

        text.finish()
    }
}
