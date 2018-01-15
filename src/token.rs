
use std::fmt;

pub struct TokenType {
    pub name: &'static str
}

impl PartialEq for TokenType {
    fn eq(&self, other: &TokenType) -> bool {
        self.name == other.name
    }
}

impl fmt::Debug for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TokenType {{ name: {} }}", self.name )
    }
}

pub struct Token {
    pub t_type: TokenType,
    pub literal: String
}

pub const ILLEGAL: TokenType = TokenType{ name: "ILLEGAL" };
pub const UNDERSCORE: TokenType = TokenType{ name: "UNDERSCORE" };
pub const LET: TokenType = TokenType{ name: "LET" };


