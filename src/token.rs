
use std::fmt;
use std::collections::HashMap;


#[derive(Hash, Clone, Eq)]
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
pub const FUN: TokenType = TokenType{ name: "FUN" };
pub const LET: TokenType = TokenType{ name: "LET" };
pub const WHILE: TokenType = TokenType{ name: "WHILE" };

pub fn keywords(key: String) -> Option<TokenType> {
    let mut kw_map = HashMap::new();
    kw_map.insert(String::from("fun"), FUN);
    kw_map.insert(String::from("let"), LET);
    kw_map.insert(String::from("while"), WHILE);
    kw_map.get(&key).cloned()
}


#[cfg(test)]
mod tests {
    use super::*;    

    #[test]
    fn test_keywords() {
        let output = keywords(String::from("let"));
        assert_eq!(output, Some(LET));
    }

}
