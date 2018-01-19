
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
pub const EOF: TokenType = TokenType{ name: "EOF" };
pub const IDENT: TokenType = TokenType{ name: "IDENT" };
pub const UNDERSCORE: TokenType = TokenType{ name: "UNDERSCORE" };
pub const FUN: TokenType = TokenType{ name: "FUN" };
pub const LET: TokenType = TokenType{ name: "LET" };
pub const WHILE: TokenType = TokenType{ name: "WHILE" };
pub const TRUE: TokenType = TokenType{ name: "TRUE" };
pub const FALSE: TokenType = TokenType{ name: "FALSE" };
pub const IF: TokenType = TokenType{ name: "IF" };
pub const ELSE: TokenType = TokenType{ name: "ELSE" };
pub const RETURN: TokenType = TokenType{ name: "RETURN" };

pub fn keywords(key: String) -> Option<TokenType> {
    let mut kw_map = HashMap::new();
    kw_map.insert(String::from("fun"), FUN);
    kw_map.insert(String::from("let"), LET);
    kw_map.insert(String::from("while"), WHILE);
    kw_map.insert(String::from("true"), TRUE);
    kw_map.insert(String::from("false"), FALSE);
    kw_map.insert(String::from("if"), IF);
    kw_map.insert(String::from("ELSE"), ELSE);
    kw_map.insert(String::from("RETURN"), RETURN);
    kw_map.get(&key).cloned()
}

pub fn lookup_ident(ident: String) -> TokenType {
    match keywords(ident) {
        Some(x) => {
            x
        },
        _ => {
            IDENT
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;    

    #[test]
    fn test_keywords() {
        let output = keywords(String::from("let"));
        assert_eq!(output, Some(LET));
    }

    #[test]
    fn test_lookup_ident() {
        let output = lookup_ident(String::from("let"));
        assert_eq!(output, LET);
    }

    #[test]
    fn test_lookup_ident_not_found() {
        let output = lookup_ident(String::from("foobar"));
        assert_eq!(output, IDENT);
    }

}

