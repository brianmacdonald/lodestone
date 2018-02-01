

struct Parser {
    lexer: lexer::Lexer,
    errors: [String; 126],
    cur_token: token::Token,
    peek_token: token::Token,
    prefix_parse_fns: HashMap<Option<TokenType>>,
    infix_parse_fns: HashMap<Option<TokenType>>
}

impl Parser {

    fn errors(&mut self) -> [String] {
        self.errors
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
        Some(Box::new(ast::LetStatement { token: token, name: name, value: value }))
    }

    fn parse_return_statement(&mut self) -> Option<Box<ast::Statement>> {
        let token = self.cur_token.clone();
        self.next_token();
        return_value = self.parse_expression(LOWEST);
        if self.peek_token_is(token::SEMICOLON) {
            self.next_token()
        }
        Some(Box::new(ast::ReturnStatement { token: token, return_value: return_value } ))
    }

    fn parser_expression_statement(&mut self) -> Option<Box<Statement>> {
        let token = self.cur_token.clone();
        let expression = self.parse_expression(LOWEST);
        if self.peek_token_is(token::SEMICOLON) {
            self.next_token();
        }
        Some(Box::new(ast::ExpressionStatement { token: token, expression: expression }))
    }

    fn parse_prefix_expression(&mut self) -> Option<Box<Statement>> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();
        p.next_token();
        let right = self.parse_expression(PREFIX);
        Some(Box::new(Expression { token: token, operator: operator, right: right }))
    }

    fn no_prefix_parse_fn_error(&mut self, t: token::TokenType) {
        let msg = format!("no prefix parse function for %s found", t);
        self.errors.push(msg);
    }

    fn parse_expression(&mut self, precedence: u16) -> Option<Box<ast::Expression>> {
        let prefix = self.prefix_parse_fns[precedence];
        match prefix {
            None => {
                self.no_prefix_parse_fn_error(self.cur_token.t_type);
                return None;
            }
        }
        let mut left_exp = prefix();
        while !self.peek_token_is(token::SEMICOLON) && precedence < self.peek_precedence() {
            let infix = self.infix_parse_fns[self.peek_token.t_type];
            match infix {
                Some(x) => {
                    return left_exp;
                },
                None => {}
            }
            self.next_token();
            left_exp = infix(left_exp);
        }
        left_exp;
    }

    fn register_prefix(&mut self, token_type: token:TokenType, func: prefix_parse_fn) {
        self.prefixParseFns.insert(token_type, func);
    }

    fn register_infix(&mut self, token_type: token:TokenType, func: infix_parse_fn) {
        self.infixParseFns.insert(token_type, func);
    }

    fn parse_identifier(&mut self) -> Option<Box<ast::Expression>> {
        Some(Box::new(ast::Identifier { token: self.cur_token.clone(), value: self.cur_token.literal.clone()}))
    }

    fn parser_integer_literal(&mut self) -> Option<Box<ast::Expression>> {
        let cur_token = self.cur_token.clone();
        value = self.cur_token.literal.parse::<u32>;
        if value.is_err() {
            let msg = format!("could not parse {} as integer", self.cur_token.literal);
            self.errors(msg);
            return None;
        }
        let value = value.unwrap();
        IntegerLiteral { token: cur_token, value: value }
    }

    fn peek_precedence(&mut self) -> u16 {
        match precedences(self.peek_token.type) {
            Some(x) => {
                return x;
            },
            None => {
                return LOWEST;
            }
        }
    }

    fn cur_precedence(&mut self) -> u16 {
        match precedences(self.cur_token.type) {
            Some(x) => {
                return x;
            },
            None => {
                return LOWEST;
            }
        }
    }

    fn parse_infix_expression(&mut self, left: ast::Expression) -> Option<Box<ast:Expression>> {
        let cur_token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();
        let precedence = self.cur_precedence();
        self.next_token();
        let right = self.parse_expression(precedence);
        Some(Box::new(InfixExpression { token: cur_token, operator: operator, left: left, right: right }))
    }

    fn parse_boolean(&mut self) -> Option<Box<Expression>> {
        Some(Box::new(Boolean {token: self.cur_token, value: self.cur_token_is(token::TRUE)}))
    }

    fn parse_grouped_expression(&mut self) -> Option<Box<Expression>> {
        self.next_token();
        let exp = self.parse_expression(LOWEST);
        if self.expect_peek(token::RPAREN) {
            return None;
        }
        return exp;
    }

    fn parse_if_expression(&mut self) -> Option<Box<Expression>> {
        let cur_token = self.cur_token.clone();
        if !self.expect_peek(token::LPAREN) {
            return None;
        }
        self.next_token();
        let condition = self.parse_expression(LOWEST);
        if !self.expect_peek(token::RPAREN) {
            return None;
        }
        if !self.expect_peek(token::LBRACE) {
            return None;
        }
        consequence = self.parse_block_statement();
        if self.expect_token_is(token::ELSE) {
            self.next_token();
            if !self.expect_peek(token::LBRACE) {
                return None;
            }
            alternative = self.parse_block_statement();
        }
        Some(Box::new(IfExpression { token: cur_token, condition: condition, consequence, consequence, alternative: alternative } ))
    }

    fn parse_block_statement(&mut self) -> Option<Box<Statement>> {
        let cur_token = self.cur_token.clone();
        let statements = Vec::new();
        self.next_token();
        for !self.cur_token_is(token::RBRACE) && !self.cur_token_is(token::EOF) {
            let stmt = self.parse_statement();
            match stmt {
                Some(x) => {
                    statements.push(stmt);
                },
                None => {}
            }
            self.next_token();
        }
        Some(Box::new(token: cur_token, statements: statements))
    }

    fn parse_function_literal(&mut self) -> Option<Box<Expression>> {
        let cur_token = self.cur_token.clone();
        if !self.expect_peek(token::LPAREN) {
            return None;
        }
        let parameters = self.parse_function_parameters();
        if !self.expect_peek(token::LBRACE) {
            return None;
        }
        let body = self.parse_block_statement();
        Some(Box::new(ast::FunctionLiteral { token: cur_token, parameters: parameters, body: body }))
    }

    func parse_function_parameters(&mut self) -> Vec<Option<Box<Expression>>> {
        let indentifiers = Vec::new();
        if self.peek_token_is(token::RPAREN) {
            self.next_token();
            return identifiers;
        }
        self.next_token();
        let mut ident = Identifier { token: self.cur_token.clone(), value: self.cur_token.literal.clone() };
        indentifiers.push(Some(Box::new(indent.clone()));
        while self.peek_token_is(token::COMMA) {
            self.next_token();
            self.next_token();
            ident = ast::Identifier {token: self.cur_token.clone(), value: self.cur_token.literal.clone() };
            identifiers.push(Some(Box::new(ident.clone())));
        }
        if !self.expect_peek(token::RPAREN) {
            return None;
        }
        indentifiers
    }

    fn parse_call_expression(&mut self, func: ast::Expression) -> Option<Box<Expression>> {
        let args = self.parse_expression_list(token::RPARAM);
        Some(Box::new(ast::CallExpression { token: self.cur_token, function, function, arguments: args }))
    }

    fn parse_call_arguments(&mut self) -> Option<Vec<Option<Box<Expression>>>> {
        let args = Vec::new();
        if self.peek_token_is(token::RPAREN) {
            self.next_token();
            return args;
        }
        self.next_token();
        args.push(self.parse_expression(LOWEST));
        while self.peek_token_is(token::COMMA) {
            self.next_token();
            self.next_token();
            args.push(self.parse_expression(LOWEST));
        }
        if !self.expect_peek(token::RPAREN) {
            return None
        }
        Some(args)
    }

    fn parse_string_literal() Option<Box<Expression>> {
        Some(Box::new(ast::StringLiteral { token: self.cur_token.clone(), value: self.cur_token.literal.clone() }))
    }

    fn parse_while_literal(&mut self) -> Option<Box<Expression>> {
        let cur_token = p.cur_token.clone();
        if !self.expect_peek(token::RPAREN) {
            return None;
        }
        if !self.expect_peek(token::LBRACE) {
            return None;
        }
        Some(Box::new(ast::WhileLiteral { token: self.cur_token, consequence: self.parse_block_statement() }))
    }

    fn parse_import_literal(&mut self) -> Option<Box<Expression>> {
        let cur_token = self.cur_token.clone();
        if !self.expect_peek(token::STRING) {
            return None;
        }
        Some(Box::new(ast::ImportLiteral { token: cur_token, path: self.parse_string_literal() }))
    }

    fn parse_array_literal(&mut self) -> Option<Box<Expression>> {
        let cur_token = self.cur_token.clone();
        Some(Box::new(ast::ArrayLiteral { token: cur_token, elements: self.parse_expression_list(token::RBRACKET) }))
    }

    fn prase_index_expression(&mut self, left: Box<Expression>) -> Option<Box<Expression>> {
        let cur_token = self.cur_token.clone();
        let left = left.clone();
        self.next_token();
        let index = self.parse_expression(LOWEST);
        if !self.expect_peek(token::RBRACKET) {
            return None;
        }
        Some(Box::new( ast::IndexExpression { token: cur_token, left: left, index: index } ))
    }

    fn parse_expression_list(&mut self, end: token::TokenType) Option<Vec<Box<Expression>>> {
        let mut list = Vec::new();
        if self.peek_token_is(end) {
            self.next_token();
            return Some(list);
        }

        self.next_token();
        list.push(self.parse_expression(LOWEST));

        while (self.peek_token_is(token::COMMA)) {
            self.next_token();
            self.next_token();
            list.push(self.parse_expression(LOWEST));
        }

        if !self.expect_peek(end) {
            return None;
        }

        Some(list)
    }

}
