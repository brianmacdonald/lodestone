
struct Lexer {
    input: &'static str,
    position:     u16,
    read_position: u16,
    ch: char
}

impl Lexer {

    fn new(input: &'static str) -> Lexer {
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

}

#[cfg(test)]
mod tests {
    use super::*;    

    #[test]
    fn lexer_sets_input() {
        let lex = Lexer::new("foobar");
        assert_eq!(lex.input, "foobar");
    }
 
    #[test]
    fn lexer_sets_char() {
        let lex = Lexer::new("foobar");
        assert_eq!(lex.ch, 'f');
    }
   

}
