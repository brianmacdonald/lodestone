use std::collections::HashMap;

use super::ast;
use super::token;
use super::lexer;

const LOWEST: u8 = 0;
const EQUALS: u8 = 1;
const LESSGREATER: u8 = 2;
const SUM: u8 = 3;
const PRODUCT: u8 = 4;
const MODULUS: u8 = 5;
const PREFIX: u8 = 6;
const CALL: u8 = 7;
const INDEX: u8 = 8;

struct Parser {
    lexer: lexer::Lexer,
    pub errors: Vec<String>,
    cur_token: token::Token,
    peek_token: token::Token,
}

impl Parser {

    fn new(lexer: lexer::Lexer) -> Parser {
        let mut parser = Parser {
            lexer: lexer,
            errors: vec![],
            cur_token: token::create_start_token(),
            peek_token: token::create_start_token()
        };
        parser.next_token();
        parser.next_token();
        return parser;
    }

    fn prefix_parse_call(&mut self, token: token::Token) -> Option<Box<ast::Expression>> {
        match token.t_type {
            token::BANG => {
                return self.parse_prefix_expression();
            },
            token::MINUS => {
                return self.parse_prefix_expression();
            },
            token::INT => {
                return self.parser_integer_literal();
            },
            token::IDENT => {
                return self.parse_identifier();
            },
            token::TRUE => {
                return self.parse_boolean();
            },
            token::FALSE => {
                return self.parse_boolean();
            },
            token::LPAREN => {
                return self.parse_grouped_expression();
            },
            token::IF => {
                return self.parse_if_expression();
            },
            token::FUNCTION => {
                return self.parse_function_literal();
            },
            token::STRING => {
                return self.parse_string_literal();
            },
            /*token::WHILE => {
                return self.parse_while_literal();
            },
            token::IMPORT => {
                return self.parse_import_literal();
            },
            */
            token::LBRACKET => {
                return self.parse_array_literal();
            }
            _ => {
                return None;
            }
        }
    }

    fn infix_parse_call(&mut self, token: token::Token) -> Option<Box<ast::Expression>> {
        match token.t_type {
            token::BANG => {
                return self.parse_prefix_expression();
            },
            _ => {
                return None;
            }
        }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program { statements: Vec::new() };
        while self.cur_token.t_type != token::EOF {
            let stmt = self.parse_statement();
            match stmt {
                Some(x) => {
                    program.statements.push(x);
                },
                None => {}
            }
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
        self.cur_token.t_type == t
    }

    fn peek_token_is(&mut self, t: token::TokenType) -> bool {
        self.peek_token.t_type == t
    }

    fn peek_error(&mut self, t: token::TokenType) {
        let token = self.peek_token.clone();
        let msg = String::from(format!("expected next token to be {}, got {} instead",
        t.name, token.t_type.name));
        self.errors.push(msg);
    }

    fn expect_peek(&mut self, t: token::TokenType) -> bool {
        if self.peek_token_is(t.clone()) {
            self.next_token();
            return true;
        } else {
            self.peek_error(t);
            return false;
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<ast::Statement>> {
        let token = self.cur_token.clone();
        if !self.expect_peek(token::IDENT) {
            return None;
        }
        let name = ast::Identifier {token: self.cur_token.clone(), value: self.cur_token.clone().literal };
        if !self.expect_peek(token::ASSIGN) {
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
        let return_value = self.parse_expression(LOWEST);
        if self.peek_token_is(token::SEMICOLON) {
            self.next_token()
        }
        Some(Box::new(ast::ReturnStatement { token: token, return_value: return_value } ))
    }

    fn parser_expression_statement(&mut self) -> Option<Box<ast::Statement>> {
        let token = self.cur_token.clone();
        let expression = self.parse_expression(LOWEST);
        if self.peek_token_is(token::SEMICOLON) {
            self.next_token();
        }
        Some(Box::new(ast::ExpressionStatement { token: token, expression: expression } ))
    }

    fn parse_prefix_expression(&mut self) -> Option<Box<ast::Expression>> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();
        self.next_token();
        let right = self.parse_expression(PREFIX);
        Some(Box::new(ast::PrefixExpression { token: token, operator: operator, right: right}))
    }

    fn no_prefix_parse_fn_error(&mut self, t: token::TokenType) {
        let msg = format!("no prefix parse function for {} found", t.name);
        self.errors.push(msg);
    }

    fn parse_expression(&mut self, precedence: u8) -> Option<Box<ast::Expression>> {
        let cur_token = self.cur_token.clone();
        let peek_token = self.peek_token.clone();
        let prefix = self.prefix_parse_call(cur_token.clone());
        match prefix {
            Some(p) => {
                while !self.peek_token_is(token::SEMICOLON) && precedence < self.peek_precedence() {
                    let infix = self.infix_parse_call(peek_token);
                    match infix {
                        Some(i) => {
                            return Some(i);
                        },
                        None => {}
                    }
                    self.next_token();
                    return infix;
                }
            },
            None => {
                //let error_token = cur_token.clone();
                self.no_prefix_parse_fn_error(cur_token.t_type);
                println!("No prefix!");
                return None;
            }
        }
        self.prefix_parse_call(cur_token.clone())
    }

    fn parse_identifier(&mut self) -> Option<Box<ast::Expression>> {
        Some(Box::new(ast::Identifier { token: self.cur_token.clone(), value: self.cur_token.literal.clone()}))
    }

    fn parser_integer_literal(&mut self) -> Option<Box<ast::Expression>> {
        let cur_token = self.cur_token.clone();
        let wrapped_value = self.cur_token.literal.parse::<u32>();
        if wrapped_value.is_err() {
            let msg = format!("could not parse {} as integer", self.cur_token.literal);
            self.errors.push(msg);
            return None;
        }
        let value = wrapped_value.unwrap();
        Some(Box::new(ast::IntegerLiteral { token: cur_token, value: value }))
    }

    pub fn precedences(&mut self, key: token::TokenType) -> Option<u8> {
        let mut kw_map = HashMap::new();
        kw_map.insert(token::EQ, EQUALS);
        kw_map.insert(token::NOT_EQ, EQUALS);
        kw_map.insert(token::LT, LESSGREATER);
        kw_map.insert(token::GT, LESSGREATER);
        kw_map.insert(token::PLUS, SUM);
        kw_map.insert(token::MINUS, SUM);
        kw_map.insert(token::MODULO, MODULUS);
        kw_map.insert(token::SLASH, PRODUCT);
        kw_map.insert(token::ASTERISK, PRODUCT);
        kw_map.insert(token::LPAREN, CALL);
        kw_map.insert(token::LBRACKET, INDEX);
        kw_map.get(&key).cloned()
    }

    fn peek_precedence(&mut self) -> u8 {
        let peek_token = self.peek_token.clone();
        let prec = self.precedences(peek_token.t_type);
        match prec {
            Some(x) => x,
            None => LOWEST
        }
    }

    fn cur_precedence(&mut self) -> u8 {
        let cur_token = self.cur_token.clone();
        let prec = self.precedences(cur_token.t_type);
        match prec {
            Some(x) => x,
            None => LOWEST
        }
    }

    fn parse_infix_expression(&mut self, left: Option<Box<ast::Expression>>) -> Option<Box<ast::Expression>> {
        let cur_token = self.cur_token.clone();
        let operator = cur_token.literal.clone();
        let precedence = self.cur_precedence();
        self.next_token();
        let right = self.parse_expression(precedence);
        Some(Box::new(ast::InfixExpression { token: cur_token, operator: operator, left: left, right: right }))
    }

    fn parse_boolean(&mut self) -> Option<Box<ast::Expression>> {
        let cur_token = self.cur_token.clone();
        Some(Box::new(ast::Boolean {token: cur_token, value: self.cur_token_is(token::TRUE)}))
    }

    fn parse_grouped_expression(&mut self) -> Option<Box<ast::Expression>> {
        self.next_token();
        let exp = self.parse_expression(LOWEST);
        if self.expect_peek(token::RPAREN) {
            return None;
        }
        return exp;
    }

    fn parse_if_expression(&mut self) -> Option<Box<ast::Expression>> {
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
        let consequence = self.parse_block_statement();
        let mut alternative = None;
        if self.peek_token_is(token::ELSE) {
            self.next_token();
            if !self.expect_peek(token::LBRACE) {
                return None;
            }
            alternative = self.parse_block_statement();
        }
        Some(Box::new(ast::IfExpression { token: cur_token, condition: condition, consequence: consequence, alternative: alternative } ))
    }

    fn parse_block_statement(&mut self) -> Option<ast::BlockStatement> {
        let cur_token = self.cur_token.clone();
        let mut statements = Vec::new();
        self.next_token();
        while !self.cur_token_is(token::RBRACE) && !self.cur_token_is(token::EOF) {
            let stmt = self.parse_statement();
            match stmt {
                Some(x) => {
                    statements.push(x);
                },
                None => {}
            }
            self.next_token();
        }
        Some(ast::BlockStatement { token: cur_token, statements: statements})
    }

    fn parse_function_literal(&mut self) -> Option<Box<ast::Expression>> {
        let cur_token = self.cur_token.clone();
        if !self.expect_peek(token::LPAREN) {
            return None;
        }
        let parameters = self.parse_function_parameters();
        if !self.expect_peek(token::LBRACE) {
            return None;
        }
        let body = self.parse_block_statement();
        Some(Box::new(ast::FunctionLiteral { token: cur_token, parameters: parameters, body: body.unwrap() }))
    }

    fn parse_function_parameters(&mut self) -> Vec<ast::Identifier> {
        let mut identifiers = Vec::new();
        if self.peek_token_is(token::RPAREN) {
            self.next_token();
            return identifiers;
        }
        self.next_token();
        let cur_token = self.cur_token.clone();
        let mut ident = ast::Identifier { token: cur_token, value: self.cur_token.literal.clone() };
        identifiers.push(ident);
        while self.peek_token_is(token::COMMA) {
            self.next_token();
            self.next_token();
            let cur_token = self.cur_token.clone();
            ident = ast::Identifier {token: cur_token, value: self.cur_token.literal.clone() };
            identifiers.push(ident);
        }
        if !self.expect_peek(token::RPAREN) {
            return vec![];
        }
        return identifiers;
    }

    fn parse_call_expression(&mut self, func: Box<ast::Expression>) -> Option<Box<ast::Expression>> {
        let args = self.parse_expression_list(token::RPAREN);
        let cur_token = self.cur_token.clone();
        Some(Box::new(ast::CallExpression { token: cur_token, function: func, arguments: args.unwrap() }))
    }

    fn parse_call_arguments(&mut self) -> Option<Vec<Option<Box<ast::Expression>>>> {
        let mut args = Vec::new();
        if self.peek_token_is(token::RPAREN) {
            self.next_token();
            return Some(args);
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

    fn parse_string_literal(&mut self) -> Option<Box<ast::Expression>> {
        let cur_token = self.cur_token.clone();
        Some(Box::new(ast::StringLiteral { token: cur_token, value: self.cur_token.literal.clone() }))
    }

    /*fn parse_while_literal(&mut self) -> Option<Box<ast::Expression>> {
        let cur_token = self.cur_token;
        if !self.expect_peek(token::RPAREN) {
            return None;
        }
        if !self.expect_peek(token::LBRACE) {
            return None;
        }
        Some(Box::new(ast::WhileLiteral { token: self.cur_token, consequence: self.parse_block_statement().unwrap() }))
    }

    fn parse_import_literal(&mut self) -> Option<Box<ast::Expression>> {
        let cur_token = self.cur_token;
        if !self.expect_peek(token::STRING) {
            return None;
        }
        Some(Box::new(ast::ImportLiteral { token: cur_token, path: self.parse_string_literal() }))
    }
        */

    fn parse_array_literal(&mut self) -> Option<Box<ast::Expression>> {
        let cur_token = self.cur_token.clone();
        Some(Box::new(ast::ArrayLiteral { token: cur_token, elements: self.parse_expression_list(token::RBRACKET).unwrap() }))
    }

    fn prase_index_expression(&mut self, left: Box<ast::Expression>) -> Option<Box<ast::Expression>> {
        let cur_token = self.cur_token.clone();
        let left = left;
        self.next_token();
        let index = self.parse_expression(LOWEST);
        if !self.expect_peek(token::RBRACKET) {
            return None;
        }
        Some(Box::new( ast::IndexExpression { token: cur_token, left: left, index: index } ))
    }

    fn parse_expression_list(&mut self, end: token::TokenType) -> Option<Vec<Box<ast::Expression>>> {
        let mut list = Vec::new();
        if self.peek_token_is(end.clone()) {
            self.next_token();
            return Some(list);
        }

        self.next_token();
        list.push(self.parse_expression(LOWEST).unwrap());

        while self.peek_token_is(token::COMMA) {
            self.next_token();
            self.next_token();
            list.push(self.parse_expression(LOWEST).unwrap());
        }

        if !self.expect_peek(end) {
            return None;
        }

        Some(list)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_let_statement() {
        let tests = vec![
            ("let x := 5;", ("x", 5.to_string(), "INT")),
            ("let y := true;", ("y", true.to_string(), "BOOL")),
            ("let foobar := y;", ("foobar", String::from("y"), "IDENT")),
            ("let foobar := \"spam\";", ("foobar", String::from("spam"), "STRING")),
        ];
        for test in tests.into_iter() {
            let lexer = lexer::Lexer::new( String::from(test.0) );
            let mut p = Parser::new(lexer);
            let mut program = p.parse_program();
            assert_eq!(program.statements.len(), 1);
            let ref mut let_stmt = program.statements[0];
            match let_stmt.as_any().downcast_ref::<ast::LetStatement>() {
                Some(ref mut stmt) => {
                    let output = test.1;
                    let ref name = stmt.name;
                    assert_eq!(name.value, output.0);
                    match stmt.value {
                        Some(ref v) => {
                            expression_test(output.2, &v, output.1);
                        },
                        None => panic!("no value found.")
                    }
                },
                None => panic!("not a let statement")
            };
        }
    }

    fn expression_test(type_t: &str, exp: &Box<ast::Expression>, expected: String) {
        match type_t {
            "INT" => {
                match exp.as_any().downcast_ref::<ast::IntegerLiteral>() {
                    Some(il) => {
                        assert_eq!(il.value.to_string(), expected);
                    },
                    None => panic!("Not a IntegerLiteral.")
                }
            },
            "BOOL" => {
                match exp.as_any().downcast_ref::<ast::Boolean>() {
                    Some(il) => {
                        assert_eq!(il.value.to_string(), expected);
                    },
                    None => panic!("Not a Boolean.")
                }
            },
            "IDENT" => {
                match exp.as_any().downcast_ref::<ast::Identifier>() {
                    Some(il) => {
                        assert_eq!(il.value.to_string(), expected);
                    },
                    None => panic!("Not a Identifier.")
                }
            },
            "STRING" => {
                match exp.as_any().downcast_ref::<ast::StringLiteral>() {
                    Some(il) => {
                        assert_eq!(il.value.to_string(), expected);
                    },
                    None => panic!("Not a StringLiteral.")
                }
            },
            _ => {
                panic!("Type not found.")
            }
        }
    }

    #[test]
    fn test_string_literal_expression() {
        let input = "\"hello world\";";
        let lexer = lexer::Lexer::new(String::from(input));
        let mut p = Parser::new(lexer);
        let mut program = p.parse_program();
        let ref mut stmt = program.statements[0];
        match stmt.as_any().downcast_ref::<ast::ExpressionStatement>() {
            Some(ref mut st) => {
                match st.expression {
                    Some(ref expression) => {
                        expression_test("STRING", &expression, String::from("hello world"));
                    },
                    None => panic!("expression not found.")
                }
            },
            None => panic!("not a let statement")
        }
    }

}
