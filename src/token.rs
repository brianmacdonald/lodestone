
use std::fmt;
use std::collections::HashMap;

#[derive(Hash, Clone, PartialEq, Eq)]
pub struct TokenType {
    pub name: &'static str,
}

impl fmt::Debug for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TokenType {{ name: {} }}", self.name)
    }
}

#[derive(Hash, Clone)]
pub struct Token {
    pub t_type: TokenType,
    pub literal: String,
}

pub fn create_start_token() -> Token {
    Token { t_type: EOF, literal: String::from("EOF") }
}

pub const ILLEGAL: TokenType = TokenType { name: "ILLEGAL" };
pub const EOF: TokenType = TokenType { name: "EOF" };
pub const IDENT: TokenType = TokenType { name: "IDENT" };
pub const UNDERSCORE: TokenType = TokenType { name: "UNDERSCORE" };
// Keywords
pub const FUNCTION: TokenType = TokenType { name: "FUNCTION" };
pub const OBJECT: TokenType = TokenType { name: "OBJECT" };
pub const IMPORT: TokenType = TokenType { name: "IMPORT" };
pub const LET: TokenType = TokenType { name: "LET" };
pub const WHILE: TokenType = TokenType { name: "WHILE" };
pub const TRUE: TokenType = TokenType { name: "TRUE" };
pub const FALSE: TokenType = TokenType { name: "FALSE" };
pub const IF: TokenType = TokenType { name: "IF" };
pub const ELSE: TokenType = TokenType { name: "ELSE" };
pub const RETURN: TokenType = TokenType { name: "RETURN" };
// Operators
pub const ASSIGN: TokenType = TokenType { name: ":=" };
pub const REASSIGN: TokenType = TokenType { name: "=" };
pub const COLON: TokenType = TokenType { name: ":" };
pub const PLUS: TokenType = TokenType { name: "+" };
pub const MINUS: TokenType = TokenType { name: "-" };
pub const BANG: TokenType = TokenType { name: "!" };
pub const ASTERISK: TokenType = TokenType { name: "*" };
pub const SLASH: TokenType = TokenType { name: "/" };
pub const MODULO: TokenType = TokenType { name: "%" };
pub const LT: TokenType = TokenType { name: "<" };
pub const GT: TokenType = TokenType { name: ">" };
pub const EQ: TokenType = TokenType { name: "==" };
pub const NOT_EQ: TokenType = TokenType { name: "!=" };
// Delimiters
pub const COMMA: TokenType = TokenType { name: "," };
pub const SEMICOLON: TokenType = TokenType { name: ";" };
pub const LPAREN: TokenType = TokenType { name: "(" };
pub const RPAREN: TokenType = TokenType { name: ")" };
pub const LBRACE: TokenType = TokenType { name: "{" };
pub const RBRACE: TokenType = TokenType { name: "}" };
pub const LBRACKET: TokenType = TokenType { name: "[" };
pub const RBRACKET: TokenType = TokenType { name: "]" };
// Slots
pub const SLOT: TokenType = TokenType { name: "." };
// Types
pub const STRING: TokenType = TokenType { name: "STRING" };
pub const INT: TokenType = TokenType { name: "INT" };

pub fn keywords(key: String) -> Option<TokenType> {
    let mut kw_map = HashMap::new();
    kw_map.insert(String::from("Object"), OBJECT);
    kw_map.insert(String::from("fun"), FUNCTION);
    kw_map.insert(String::from("let"), LET);
    kw_map.insert(String::from("while"), WHILE);
    kw_map.insert(String::from("true"), TRUE);
    kw_map.insert(String::from("false"), FALSE);
    kw_map.insert(String::from("if"), IF);
    kw_map.insert(String::from("else"), ELSE);
    kw_map.insert(String::from("return"), RETURN);
    kw_map.insert(String::from("import"), IMPORT);
    kw_map.get(&key).cloned()
}

pub fn lookup_ident(ident: String) -> TokenType {
    match keywords(ident) {
        Some(x) => x,
        _ => IDENT,
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
