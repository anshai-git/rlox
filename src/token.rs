use std::fmt::Debug;

use crate::object::Object;
use crate::token_type::TokenType;

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Object,
    pub line: u16,
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.token_type)
            .field(&self.lexeme)
            .field(&self.literal)
            .finish()
    }
}
