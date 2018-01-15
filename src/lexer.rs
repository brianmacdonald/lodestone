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
    }    


    fn next_token(&mut self) -> token::Token {
        let mut tok: token::Token;
        self.skip_whitespace();
        match self.ch {
            '_' => {
               tok = new_token(token::UNDERSCORE, self.ch);
            },
             _ => {
               tok = new_token(token::ILLEGAL, self.ch);
            } 
        }
        tok
    }

    fn skip_whitespace(&mut self) {
        if self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
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
        let mut position = self.position + 1;
        let mut done = false;
        while !done {
            self.read_char();
            if self.ch == '"' || self.ch == 0 as char {
                done = true;
            }
        }
        return self.input.chars().skip(position as usize).take(self.position as usize).collect();
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
        let mut lex = Lexer::new(String::from("fn _call"));
        assert_eq!(lex.next_token().t_type, token::ILLEGAL);
        lex.read_char();
        assert_eq!(lex.next_token().t_type, token::ILLEGAL);
        lex.read_char();
        assert_eq!(lex.next_token().t_type, token::UNDERSCORE);
        lex.read_char();
        assert_eq!(lex.next_token().t_type, token::ILLEGAL);
    }

    #[test]
    fn test_peek_char() {
        let mut lex = Lexer::new(String::from("fn _call"));
        assert_eq!(lex.peek_char(), 'n');
    }
   
    #[test]
    fn test_read_char() {
        let mut lex = Lexer::new(String::from("\"_call\";"));
        assert_eq!(lex.read_string(), String::from("_call\""));
    }

    #[test]
    fn test_is_letter() {
        assert!(is_letter('a'));
        assert!(is_letter('M'));
        assert!(is_letter('_'));
        assert!(!is_letter('%'));
    }


    #[test]
    fn test_is_digit() {
        assert!(is_digit('1'));
    }

    #[test]
    fn test_is_not_digit() {
        assert!(!is_digit('a'));
    }

    fn test_next_tokens() {
        let input = "
           let five := 5;
        ";
        let mut lex = Lexer::new(String::from(input));
        let tok = lex.next_token();
        assert_eq!(tok.t_type, token::LET);
    }

}
