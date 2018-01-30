

struct Parser {
    lexer: lexer::Lexer,
    errors: [String; 126],
    cur_token: token::Token,
    peek_token: token::Token,
    prefix_parse_fns: HashMap<TokenType>,
    infix_parse_fns: HashMap<TokenType>
}

impl Parser {

    fn errors(&mut self) -> [String] {

    }

    fn next_token(&mut self) {
        self.cur_token = &self.peek_token;
        self.peek_token = &self.lexer.next_token();
    }

    fn parse_program(&mut self) -> ast::Program {
        let mut program: ast::Program { statements: vec:new() };
        while self.cur_token.t_type != token::EOF {
            let stmt = self.parse_statement();
            program.statements.push(stmt);
            self.next_token();
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Box<ast::Statement>> {
        match self.cur_token.t_type {
            token::LET => {
                return self.parse_let_statement();
            },
            token::RETURN => {
                return self.parse_return_statement();
            },
            _ => {
                return self.parser_expression_statement();
            }
        }
    }

    fn cur_token_is(&mut self, t: token::TokenType) -> bool {
        self.cur_token == t
    }

    fn peek_token_is(&mut self, t: token::TokenType) -> bool {
        self.peek_token == t
    }

    fn peek_error(&mut self, t: token::TokenType) {
        let msg = String::from(format!("expected next token to be {}, got {} instead",
        t, self.peek_token.type));
        self.errors.push(msg);
    }

    fn expect_peek(&mut self, t: token::TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            return true;
        } else {
            self.peek_error(t);
            return false;
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<ast:Statement>> {
        let token = &self.cur_token;
        if !self.expect_peek(token::IDENT) {
            return None;
        }
        let name = ast::Identifier {token: self.cur_token, value: self.cur_token.literal };
        if !self.expect_peek(token::IDENT) {
            return None;
        }
        self.next_token();
        let value = self.parse_expression(LOWEST);
        if self.peek_token_is(token::SEMICOLON) {
            self.next_token();
        }
        return Some(Box::new(ast::LetStatement { token: token, name: name, value: value }));
    }

    fn parse_return_statement(&mut self) -> Option<Box<ast::Statement>> {
        let token = self.cur_token.clone();
        self.next_token();
        return_value = self.parse_expression(LOWEST);
        if self.peek_token_is(token::SEMICOLON) {
            self.next_token()
        }
        return Some(Box::new(ast::ReturnStatement { token: token, return_value: return_value } ))
    }

    fn parser_expression_statement(&mut self) -> Option<Box<Statement>> {
        let token = self.cur_token.clone();
        let expression = self.parse_expression(LOWEST);
        if self.peek_token_is(token::SEMICOLON) {
            self.next_token();
        }
        return Some(Box::new(ast::ExpressionStatement { token: token, expression: expression }));
    }


}
