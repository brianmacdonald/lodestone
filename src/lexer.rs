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
            println!("{}", self.ch);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }    

    fn next_token(&mut self) -> token::Token {
        let tok: token::Token;
        self.skip_whitespace();
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
            '_' => {
               tok = new_token(token::UNDERSCORE, self.ch);
            },
            _ => {
                if is_letter(self.ch) {
                    let l_literal = self.read_identifier();
                    let l_t_type = token::lookup_ident(l_literal.clone());
                    tok = token::Token { t_type: l_t_type, literal: l_literal }; 
                } else if is_digit(self.ch) {
                    tok = token::Token { t_type: token::INT, literal:self.read_number() };
                } else if self.ch == 0 as char {
                    tok = new_token(token::EOF, self.ch);
                } else {
                    tok = new_token(token::ILLEGAL, self.ch);
                }
            } 
        }
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
        let mut lex = Lexer::new(String::from("fn _call"));
        assert_eq!(lex.next_token().t_type, token::IDENT);
        assert_eq!(lex.next_token().t_type, token::UNDERSCORE);
        assert_eq!(lex.next_token().t_type, token::IDENT);
    }

    #[test]
    fn test_skip_whitespace() {
        let input = " _call";
        let mut lex = Lexer::new(String::from(input));
        lex.skip_whitespace();
        assert_eq!(lex.ch, '_');
    }

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
        let input = "
            let five := 5; 
        ";
        let expected = [
            (token::LET, "let"),
            (token::IDENT, "five"),
            (token::ASSIGN, ":="),
            (token::INT, "5"),
        ];
        println!("{}", input);
        let mut lex = Lexer::new(String::from(input));
        for output in expected.into_iter() {
            let tok = lex.next_token();
            assert_eq!(tok.literal, output.1);
            assert_eq!(tok.t_type, output.0);
        }
    }

}
