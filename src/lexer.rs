use super::token;

struct Lexer {
    input: String,
    position: u16,
    read_position: u16,
    ch: char
}

impl Lexer {

    fn new(input: String) -> Lexer {
        let mut l = Lexer { input: input, position: 0, read_position: 0, ch: 0 as char};
        l.read_char();
        return l;
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() as u16 {
            self.ch = 0 as char;
        } else {
            self.ch = self.input.chars().nth(self.read_position as usize).unwrap() as char;
        }
        self.position = self.read_position;
        self.read_position += 1;
        println!("p: {}, rp: {}, ch: {}", self.position, self.read_position, self.ch);
    }    

    fn next_token(&mut self) -> token::Token {
        let tok: token::Token;
        println!("about to test bws: {}", self.ch);
        self.skip_whitespace();
        println!("abount to test: {}", self.ch);
        match self.ch {
            ':' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let mut l_literal = String::from(ch.to_string());
                    l_literal.push(self.ch);
                    tok = token::Token { t_type: token::ASSIGN, literal: l_literal };
                } else {
                    tok = new_token(token::COLON, self.ch);
                }
            },
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let mut l_literal = String::from(ch.to_string());
                    l_literal.push(self.ch);
                    tok = token::Token { t_type: token::NOT_EQ, literal: l_literal };
                } else {
                    tok = new_token(token::BANG, self.ch);
                }
            },
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let mut l_literal = String::from(ch.to_string());
                    l_literal.push(self.ch);
                    tok = token::Token { t_type: token::EQ, literal: l_literal };
                } else {
                    tok = new_token(token::REASSIGN, self.ch);
                }
            },
            '-' => {
                tok = new_token(token::MINUS, self.ch);
            },
            '_' => {
               tok = new_token(token::UNDERSCORE, self.ch);
            },
            '/' => {
                 tok = new_token(token::SLASH, self.ch)
            },
            '*' => {                                                                                                 
               tok = new_token(token::ASTERISK, self.ch);                                                                 
            },
            '<' => {                                                                                                 
               tok = new_token(token::LT, self.ch);                                                                       
            },
            '>' => {                                                                                                 
               tok = new_token(token::GT, self.ch);                                                                       
            },
            ';' => {                                                                                                 
               tok = new_token(token::SEMICOLON, self.ch);                                                                
            },
            '(' => {                                                                                                 
               tok = new_token(token::LPAREN, self.ch);                                                                   
            },
            ')' => {                                                                                                 
               tok = new_token(token::RPAREN, self.ch);                                                                   
            },
            ',' => {                                                                                                 
               tok = new_token(token::COMMA, self.ch);                                                                    
            },
            '+' => {                                                                                                 
               tok = new_token(token::PLUS, self.ch);                                                                     
            },
            '{' => {                                                                                                 
               tok = new_token(token::LBRACE, self.ch);                                                                   
            },
            '}' => {                                                                                                 
               tok = new_token(token::RBRACE, self.ch);                                                                   
            },
            '.' => {                                                                                                 
               tok = new_token(token::SLOT, self.ch);                                                                     
            },
            '%' => {                                                                                                 
               tok = new_token(token::MODULO, self.ch);                                                                   
            },
            '[' => {
                tok = new_token(token::LBRACKET, self.ch);
            },
            ']' => {
                tok = new_token(token::RBRACKET, self.ch);
            },
            '"' => {
                tok = token::Token { t_type: token::STRING, literal: self.read_string()  };
            },
            _ => {
                if is_letter(self.ch) {
                    let l_literal = self.read_identifier();
                    let l_t_type = token::lookup_ident(l_literal.clone());
                    tok = token::Token { t_type: l_t_type, literal: l_literal }; 
                    return tok;
                } else if is_digit(self.ch) {
                    tok = token::Token { t_type: token::INT, literal:self.read_number() };
                    return tok;
                } else if self.ch == 0 as char {
                    tok = new_token(token::EOF, self.ch);
                } else {
                    tok = new_token(token::ILLEGAL, self.ch);
                }
            } 
        }
        println!("{}", tok.literal);
	    self.read_char();
        tok
    }

    fn skip_whitespace(&mut self) {
        let mut done = false;
        while !done {
            if self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
                self.read_char();
            } else {
                done = true;
            }
        }
    }

    fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() as u16 {
            return 0 as char;
        } else {
            return self.input.chars().nth(self.read_position as usize).unwrap() as char;
        }
    }

    fn read_string(&mut self) -> String {
        let position = self.position + 1;
        let mut done = false;
        while !done {
            self.read_char();
            if self.ch == '"' || self.ch == 0 as char {
                done = true;
            }
        }
        let take_size = self.position - position;
        return self.input.chars().skip(position as usize).take(take_size as usize).collect();
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        let mut done = false;
        while !done {
            if is_letter(self.ch) {
                self.read_char();
            } else {
                done = true;
            }
        }
        let take_size = self.position - position;
        return self.input.chars().skip(position as usize).take(take_size as usize).collect();
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        let mut done = false;
        while !done {
            if is_digit(self.ch) {
                self.read_char();
            } else {
                done = true;
            }
        }
        let take_size = self.position - position;
        return self.input.chars().skip(position as usize).take(take_size as usize).collect();
    }
}


fn new_token (token_type: token::TokenType, ch: char) -> token::Token {
    return token::Token { t_type: token_type, literal: ch.to_string() };   
}


fn is_letter(ch: char) -> bool {
    return 'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_';
}


fn is_digit(ch: char) -> bool {
    return '0' <= ch && ch <= '9';
}

#[cfg(test)]
mod tests {
    use super::*;    

    #[test]
    fn test_lexer_sets_input() {
        let lex = Lexer::new(String::from("foobar"));
        assert_eq!(lex.input, String::from("foobar"));
    }
 
    #[test]
    fn test_lexer_sets_char() {
        let lex = Lexer::new(String::from("foobar"));
        assert_eq!(lex.ch, 'f');
    }

    #[test]
    fn test_next_token() {
        let mut lex = Lexer::new(String::from("fn _call: foobar;"));
        assert_eq!(lex.next_token().t_type, token::IDENT);
        assert_eq!(lex.next_token().t_type, token::UNDERSCORE);
        assert_eq!(lex.next_token().literal, String::from("call"));
        assert_eq!(lex.next_token().literal, String::from(":"));
    }

    #[test]
    fn test_skip_whitespace() {
        let input = " ;_call";
        let mut lex = Lexer::new(String::from(input));
        lex.skip_whitespace();
        assert_eq!(lex.ch, ';');
    }

    #[test]
    fn test_skip_whitespace_newline() {
        let input = "
            _call
        ";
        let mut lex = Lexer::new(String::from(input));
        lex.skip_whitespace();
        assert_eq!(lex.ch, '_');
    }

    #[test]
    fn test_peek_char() {
        let mut lex = Lexer::new(String::from("fn _call"));
        assert_eq!(lex.peek_char(), 'n');
    }
   
    #[test]
    fn test_read_char() {
        let mut lex = Lexer::new(String::from("\"_call\";"));
        assert_eq!(lex.read_string(), String::from("_call"));
    }

    #[test]
    fn test_is_letter() {
        assert!(is_letter('a'));
        assert!(is_letter('M'));
        assert!(is_letter('_'));
        assert!(!is_letter(' '));
        assert!(!is_letter('\n'));
        assert!(!is_letter('%'));
        assert!(!is_letter(';'));
    }

    #[test]
    fn test_is_digit() {
        assert!(is_digit('1'));
    }

    #[test]
    fn test_is_not_digit() {
        assert!(!is_digit('a'));
    }

    #[test]
    fn test_next_tokens() {
        let eof = 0 as char;
        let input = r#"
let five := 5;
let ten := 10;
let add := fun(x, y) {
	x + y;
};
let result := add(five, ten);
!-/*5;

five: ten;
five = ten;

5 < 10 > 5;
if (5 < 10) {
   return true;
} else {
   return false;
}

10 == 10;
10 != 9;

10 % 3;

"foobar"
"foo bar"

while (true) {
	"spam eggs"
}

[1, 2];

import "/path/";

five.add := add;
        "#;
        let expected = [
            (token::LET, "let"),
            (token::IDENT, "five"),
            (token::ASSIGN, ":="),
            (token::INT, "5"),
            (token::SEMICOLON, ";" ),
            (token::LET, "let" ),
            (token::IDENT, "ten" ),
            (token::ASSIGN, ":=" ),
            (token::INT, "10" ),
            (token::SEMICOLON, ";" ),
            (token::LET, "let" ),
            (token::IDENT, "add" ),
            (token::ASSIGN, ":=" ),
            (token::FUNCTION, "fun" ),
            (token::LPAREN, "(" ),
            (token::IDENT, "x" ),
            (token::COMMA, "," ),
            (token::IDENT, "y" ),
            (token::RPAREN, ")" ),
            (token::LBRACE, "{" ),
            (token::IDENT, "x" ),
            (token::PLUS, "+" ),
            (token::IDENT, "y" ),
            (token::SEMICOLON, ";" ),
            (token::RBRACE, "}" ),
            (token::SEMICOLON, ";" ),
            (token::LET, "let" ),
            (token::IDENT, "result" ),
            (token::ASSIGN, ":=" ),
            (token::IDENT, "add" ),
            (token::LPAREN, "(" ),
            (token::IDENT, "five" ),
            (token::COMMA, "," ),
            (token::IDENT, "ten" ),
            (token::RPAREN, ")" ),
            (token::SEMICOLON, ";" ),
            (token::BANG, "!" ),
            (token::MINUS, "-" ),
            (token::SLASH, "/" ),
            (token::ASTERISK, "*" ),
            (token::INT, "5" ),
            (token::SEMICOLON, ";" ),
            (token::IDENT, "five" ),
            (token::COLON, ":" ),
            (token::IDENT, "ten" ),
            (token::SEMICOLON, ";" ),
            (token::IDENT, "five" ),
            (token::REASSIGN, "=" ),
            (token::IDENT, "ten" ),
            (token::SEMICOLON, ";" ),
            (token::INT, "5" ),
            (token::LT, "<" ),
            (token::INT, "10" ),
            (token::GT, ">" ),
            (token::INT, "5" ),
            (token::SEMICOLON, ";" ),
            (token::IF, "if" ),
            (token::LPAREN, "(" ),
            (token::INT, "5" ),
            (token::LT, "<" ),
            (token::INT, "10" ),
            (token::RPAREN, ")" ),
            (token::LBRACE, "{" ),
            (token::RETURN, "return" ),
            (token::TRUE, "true" ),
            (token::SEMICOLON, ";" ),
            (token::RBRACE, "}" ),
            (token::ELSE, "else" ),
            (token::LBRACE, "{" ),
            (token::RETURN, "return" ),
            (token::FALSE, "false" ),
            (token::SEMICOLON, ";" ),
            (token::RBRACE, "}" ),
            (token::INT, "10" ),
            (token::EQ, "==" ),
            (token::INT, "10" ),
            (token::SEMICOLON, ";" ),
            (token::INT, "10" ),
            (token::NOT_EQ, "!=" ),
            (token::INT, "9" ),
            (token::SEMICOLON, ";" ),
            (token::INT, "10" ),
            (token::MODULO, "%" ),
            (token::INT, "3" ),
            (token::SEMICOLON, ";" ),
            (token::STRING, "foobar" ),
            (token::STRING, "foo bar" ),
            (token::WHILE, "while" ),
            (token::LPAREN, "(" ),
            (token::TRUE, "true" ),
            (token::RPAREN, ")" ),
            (token::LBRACE, "{" ),
            (token::STRING, "spam eggs" ),
            (token::RBRACE, "}" ),
            (token::LBRACKET, "[" ),
            (token::INT, "1" ),
            (token::COMMA, "," ),
            (token::INT, "2" ),
            (token::RBRACKET, "]" ),
            (token::SEMICOLON, ";" ),
            (token::IMPORT, "import" ),
            (token::STRING, "/path/" ),
            (token::SEMICOLON, ";" ),
            (token::IDENT, "five" ),
            (token::SLOT, "." ),
            (token::IDENT, "add" ),
            (token::ASSIGN, ":=" ),
            (token::IDENT, "add" ),
            (token::SEMICOLON, ";" ),
            (token::EOF, &eof.to_string()),
        ];
        let mut lex = Lexer::new(String::from(input));
        for output in expected.into_iter() {
            let tok = lex.next_token();
            assert_eq!(tok.literal, output.1);
            assert_eq!(tok.t_type, output.0);
        }
    }

}
