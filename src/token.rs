use crate::{object::Object, token_type::TokenType};

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Object,
    line: usize
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Object, line: usize) ->Self {
        Self {
            token_type,
            lexeme,
            literal,
            line
        }
    }
}
