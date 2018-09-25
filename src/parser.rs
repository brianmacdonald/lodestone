use std::collections::HashMap;

use super::ast::NodeKind;
use super::ast::ExpressionKind;
use super::ast::StatementKind;
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

pub struct Parser {
    lexer: lexer::Lexer,
    pub errors: Vec<String>,
    cur_token: token::Token,
    peek_token: token::Token,
}

impl Parser {

    pub fn new(lexer: lexer::Lexer) -> Parser {
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

    fn prefix_parse_call(&mut self, token: token::Token) -> Option<Box<ExpressionKind>> {
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
            token::OBJECT => {
                return self.parse_object_literal();
            },
            token::LBRACE => {
                if !self.peek_token_is(token::RBRACE) {
                    return None;
                }
                return self.parse_object_literal();
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

    fn infix_parse_call(&mut self, token: token::Token, expression: Option<Box<ExpressionKind>>) -> Option<Box<ExpressionKind>> {
        match token.t_type {
            token::PLUS => {
                return self.parse_infix_expression(expression);
            },
            token::MINUS => {
                return self.parse_infix_expression(expression);
            },
            token::MODULO => {
                return self.parse_infix_expression(expression);;
            },
            token::SLASH => {
                return self.parse_infix_expression(expression);;
            },
            token::ASTERISK => {
                return self.parse_infix_expression(expression);
            },
            token::EQ => {
                return self.parse_infix_expression(expression);
            },
            token::NOT_EQ => {
                return self.parse_infix_expression(expression);
            },
            token::LT => {
                return self.parse_infix_expression(expression);
            },
            token::GT => {
                return self.parse_infix_expression(expression);
            },
            token::LPAREN => {
                return self.parse_call_expression(expression);
            },
            token::LBRACKET => {
                return self.parse_index_expression(expression);
            },
            _ => {
                return None;
            }
        }
    }

    fn has_infix(&mut self,  token: token::Token) -> bool {
        match token.t_type {
            token::PLUS => {
                return true;
            },
            token::MINUS => {
                return true;
            },
            token::MODULO => {
                return true;
            },
            token::SLASH => {
                return true;
            },
            token::ASTERISK => {
                return true;
            },
            token::EQ => {
                return true;
            },
            token::NOT_EQ => {
                return true;
            },
            token::LT => {
                return true;
            },
            token::GT => {
                return true;
            },
            token::LPAREN => {
                return true;
            },
            token::LBRACKET => {
                return true;
            },
            _ => {
                return false;
            }
        }
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> NodeKind {
        let mut stmt_vec: Vec<StatementKind> = Vec::new();
        while self.cur_token.t_type != token::EOF {
            let stmt = self.parse_statement();
            match stmt {
                Some(x) => {
                    stmt_vec.push(*x);
                },
                None => {}
            }
            self.next_token();
        }
        return NodeKind::ProgramNode { statements: stmt_vec };
    }

    fn parse_statement(&mut self) -> Option<Box<StatementKind>> {
        match self.cur_token.t_type {
            token::LET => {
                return self.parse_let_statement();
            },
            token::RETURN => {
                return self.parse_return_statement();
            },
            token::IDENT => {
                return self.parse_expression_statement();
            },
            _ => {
                println!("other parse statement: {}", self.cur_token.literal);
                return self.parse_expression_statement();
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

    fn parse_let_statement(&mut self) -> Option<Box<StatementKind>> {
        let token = self.cur_token.clone();
        if !self.expect_peek(token::IDENT) {
            return None;
        }
        let name = ExpressionKind::Identifier {token: self.cur_token.clone(), value: self.cur_token.clone().literal };
        if !self.expect_peek(token::ASSIGN) {
            if self.expect_peek(token::CLONE) {
                self.next_token();
                let value = self.parse_expression(LOWEST);
                if self.peek_token_is(token::SEMICOLON) {
                    self.next_token();
                }
                return Some(Box::new(StatementKind::LetCloneStatement { token: token, name: name, value: value }));
            } 
            return None;
        }
        self.next_token();
        let value = self.parse_expression(LOWEST);
        if self.peek_token_is(token::SEMICOLON) {
            self.next_token();
        }
        Some(Box::new(StatementKind::LetStatement { token: token, name: name, value: value }))
    }

    fn parse_return_statement(&mut self) -> Option<Box<StatementKind>> {
        let token = self.cur_token.clone();
        self.next_token();
        let return_value = self.parse_expression(LOWEST);
        if self.peek_token_is(token::SEMICOLON) {
            self.next_token();
        }
        let es = StatementKind::ExpressionStatement {token: self.cur_token.clone(), expression: return_value};
        Some(Box::new(StatementKind::ReturnStatement { token: token, return_value: Some(Box::new(es)) } ))
    }

    fn parse_expression_statement(&mut self) -> Option<Box<StatementKind>> {
        let token = self.cur_token.clone();
        let expression = self.parse_expression(LOWEST);
        let slot_assignment_statement = self.parse_slot_assignment_statement(expression.clone());
        match slot_assignment_statement {
            None => {},
            _ => {
                if self.peek_token_is(token::SEMICOLON) {
                    self.next_token();
                }
                return slot_assignment_statement;
            }
        }
        if self.peek_token_is(token::SEMICOLON) {
            self.next_token();
        }
        Some(Box::new(StatementKind::ExpressionStatement{ token: token, expression: expression } ))
    }

    fn parse_slot_assignment_statement(&mut self, expression: Option<Box<ExpressionKind>>) -> Option<Box<StatementKind>> {
        match expression {
            Some(exp) => {
                let token = self.cur_token.clone();
                let e = exp.clone();
                match *exp {
                    ExpressionKind::SlotIdentiferExpression{..} => {
                        if self.peek_token_is(token::ASSIGN) {
                            self.next_token();
                            self.next_token();
                            let value = self.parse_expression(LOWEST);
                            return Some(Box::new(StatementKind::SlotAssignmentStatement{token: token, slot: Some(e), value}));
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }
        return None;
    }

    fn parse_prefix_expression(&mut self) -> Option<Box<ExpressionKind>> {
        let token = self.cur_token.clone();
        let operator = self.cur_token.literal.clone();
        self.next_token();
        let right = self.parse_expression(PREFIX);
        Some(Box::new(ExpressionKind::PrefixExpression { token: token, operator: operator, right: right}))
    }

    fn no_prefix_parse_fn_error(&mut self, t: token::TokenType) {
        let msg = format!("no prefix parse function for {} found", t.name);
        self.errors.push(msg);
    }

    fn parse_expression(&mut self, precedence: u8) -> Option<Box<ExpressionKind>> {
        let cur_token = self.cur_token.clone();
        let prefix = self.prefix_parse_call(cur_token.clone());
        match prefix {
            Some(p) => {
                let mut left_exp = Some(p);
                while !self.peek_token_is(token::SEMICOLON) && precedence < self.peek_precedence() {
                    let peek = self.peek_token.clone();
                    let infix_check = self.has_infix(peek.clone());
                    if !infix_check {
                        return left_exp;
                    }
                    self.next_token();
                    left_exp = self.infix_parse_call(peek, left_exp);
                }
                return left_exp;
            },
            None => {
                //let error_token = cur_token.clone();
                self.no_prefix_parse_fn_error(cur_token.t_type);
                println!("No prefix!");
                return None;
            }
        }
        //self.prefix_parse_call(cur_token.clone())
    }

    fn parse_identifier(&mut self) -> Option<Box<ExpressionKind>> {
        if self.peek_token_is(token::SLOT) {
            let parent = self.cur_token.literal.clone();
            self.next_token();
            let mut children = Vec::new();
            while self.peek_token_is(token::IDENT) {
                self.next_token();
                children.push(self.cur_token.literal.clone());
                if self.peek_token_is(token::SLOT) {
                    self.next_token();
                } 
            }
            return Some(Box::new(ExpressionKind::SlotIdentiferExpression{token: self.cur_token.clone(), parent, children}));
        } 
        Some(Box::new(ExpressionKind::Identifier { token: self.cur_token.clone(), value: self.cur_token.literal.clone()}))
    }

    fn parser_integer_literal(&mut self) -> Option<Box<ExpressionKind>> {
        let cur_token = self.cur_token.clone();
        let wrapped_value = self.cur_token.literal.parse::<u32>();
        if wrapped_value.is_err() {
            let msg = format!("could not parse {} as integer", self.cur_token.literal);
            self.errors.push(msg);
            return None;
        }
        let value = wrapped_value.unwrap();
        Some(Box::new(ExpressionKind::IntegerLiteral { token: cur_token, value: value }))
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

    fn parse_infix_expression(&mut self, left: Option<Box<ExpressionKind>>) -> Option<Box<ExpressionKind>> {
        let cur_token = self.cur_token.clone();
        let operator = cur_token.literal.clone();
        let precedence = self.cur_precedence();
        self.next_token();
        let right = self.parse_expression(precedence);
        Some(Box::new(ExpressionKind::InfixExpression { token: cur_token, operator: operator, left: left, right: right }))
    }

    fn parse_boolean(&mut self) -> Option<Box<ExpressionKind>> {
        let cur_token = self.cur_token.clone();
        Some(Box::new(ExpressionKind::BooleanExpression {token: cur_token, value: self.cur_token_is(token::TRUE)}))
    }

    fn parse_grouped_expression(&mut self) -> Option<Box<ExpressionKind>> {
        self.next_token();
        let exp = self.parse_expression(LOWEST);
        if !self.expect_peek(token::RPAREN) {
            return None;
        }
        return exp;
    }

    fn parse_if_expression(&mut self) -> Option<Box<ExpressionKind>> {
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
        Some(Box::new(ExpressionKind::IfExpression { token: cur_token, condition: condition, consequence: consequence, alternative: alternative } ))
    }

    fn parse_block_statement(&mut self) -> Option<Box<StatementKind>> {
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
        Some(Box::new(StatementKind::BlockStatement { token: cur_token, statements: statements}))
    }

    fn parse_function_literal(&mut self) -> Option<Box<ExpressionKind>> {
        let cur_token = self.cur_token.clone();
        if !self.expect_peek(token::LPAREN) {
            return None;
        }
        let parameters = self.parse_function_parameters();
        if !self.expect_peek(token::LBRACE) {
            return None;
        }
        let body = self.parse_block_statement();
        match body {
            Some(b) => {
                Some(Box::new(ExpressionKind::FunctionLiteral { token: cur_token, parameters: parameters, body: b }))
            },
            _ => {
                panic!("Required function body");
            }
        }
    }

    fn parse_object_literal(&mut self) -> Option<Box<ExpressionKind>> {
        let cur_token = self.cur_token.clone();
        if !self.expect_peek(token::RBRACE) {
            return None;
        }
        if self.peek_token_is(token::SEMICOLON) {
            self.next_token();
        }
        Some(Box::new(ExpressionKind::ObjectLiteral { token: cur_token }))
    }

    fn parse_function_parameters(&mut self) -> Vec<ExpressionKind> {
        let mut identifiers = Vec::new();
        if self.peek_token_is(token::RPAREN) {
            self.next_token();
            return identifiers;
        }
        self.next_token();
        let cur_token = self.cur_token.clone();
        let mut ident = ExpressionKind::Identifier{ token: cur_token, value: self.cur_token.literal.clone() };
        identifiers.push(ident);
        while self.peek_token_is(token::COMMA) {
            self.next_token();
            self.next_token();
            let cur_token = self.cur_token.clone();
            ident = ExpressionKind::Identifier{token: cur_token, value: self.cur_token.literal.clone() };
            identifiers.push(ident);
        }
        if !self.expect_peek(token::RPAREN) {
            return vec![];
        }
        return identifiers;
    }

    fn parse_call_expression(&mut self, func: Option<Box<ExpressionKind>>) -> Option<Box<ExpressionKind>> {
        let args = self.parse_expression_list(token::RPAREN);
        let cur_token = self.cur_token.clone();
        match func {
            Some(f) => {
                match args {
                    Some(args_unwrapped) => {
                        Some(Box::new(ExpressionKind::CallExpression { token: cur_token, function: f, arguments: args_unwrapped }))
                    }
                    None => {
                        Some(Box::new(ExpressionKind::CallExpression { token: cur_token, function: f, arguments: vec![]}))
                    }
                }
            },
            None => {
                panic!("not a call expression.")
            }
        }
    }

    fn parse_call_arguments(&mut self) -> Option<Vec<Option<Box<ExpressionKind>>>> {
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

    fn parse_string_literal(&mut self) -> Option<Box<ExpressionKind>> {
        let cur_token = self.cur_token.clone();
        Some(Box::new(ExpressionKind::StringLiteral { token: cur_token, value: self.cur_token.literal.clone() }))
    }

    /*fn parse_while_literal(&mut self) -> Option<Box<astenum::ExpressionKind>> {
        let cur_token = self.cur_token;
        if !self.expect_peek(token::RPAREN) {
            return None;
        }
        if !self.expect_peek(token::LBRACE) {
            return None;
        }
        Some(Box::new(ast::WhileLiteral { token: self.cur_token, consequence: self.parse_block_statement().unwrap() }))
    }

    fn parse_import_literal(&mut self) -> Option<Box<astenum::ExpressionKind>> {
        let cur_token = self.cur_token;
        if !self.expect_peek(token::STRING) {
            return None;
        }
        Some(Box::new(ast::ImportLiteral { token: cur_token, path: self.parse_string_literal() }))
    }
        */

    fn parse_array_literal(&mut self) -> Option<Box<ExpressionKind>> {
        let cur_token = self.cur_token.clone();
        Some(Box::new(ExpressionKind::ArrayLiteral { token: cur_token, elements: self.parse_expression_list(token::RBRACKET).unwrap() }))
    }

    fn prase_index_expression(&mut self, left: Box<ExpressionKind>) -> Option<Box<ExpressionKind>> {
        let cur_token = self.cur_token.clone();
        let left = left;
        self.next_token();
        let index = self.parse_expression(LOWEST);
        if !self.expect_peek(token::RBRACKET) {
            return None;
        }
        Some(Box::new(ExpressionKind::IndexExpression { token: cur_token, left: left, index: index } ))
    }

    fn parse_expression_list(&mut self, end: token::TokenType) -> Option<Vec<Box<ExpressionKind>>> {
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

    fn parse_index_expression(&mut self, left: Option<Box<ExpressionKind>>) -> Option<Box<ExpressionKind>> {
        let cur_token = self.cur_token.clone();
        self.next_token();
        let index = self.parse_expression(LOWEST);

        if !self.expect_peek(token::RBRACKET) {
            return None;
        }
        match left {
            Some(l) => {
                return Some(Box::new(ExpressionKind::IndexExpression { token: cur_token, left: l, index: index }));
            },
            None => {
                return None;
            }
        }
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
            ("let foobar := {};", ("foobar", String::from("spam"), "OBJECT")),
        ];
        for test in tests.into_iter() {
            let lexer = lexer::Lexer::new( String::from(test.0) );
            let mut p = Parser::new(lexer);
            let mut program = p.parse_program();
            match program {
                NodeKind::ProgramNode{statements, ..} => {
                    let let_stmt_op = statements.get(0);
                    match let_stmt_op {
                        Some(let_stmt) => {
                            match let_stmt {
                                StatementKind::LetStatement{name, value, ..} => {
                                    let output = test.1;
                                    match name {
                                        ExpressionKind::Identifier{value: i_val, ..} => {
                                            assert_eq!(i_val, output.0);
                                        },
                                        _ => panic!("not an identifier statement")
                                    }
                                    match value {
                                        Some(v) => {
                                            let let_value = v.clone();
                                            expression_test(output.2, *let_value, output.1);
                                        },
                                        None => panic!("no value found.")
                                    }
                                },
                                _ => panic!("not a let statement")
                            }
                        },
                        _ => {
                            panic!("first statement not found");
                        }
                    }
                },
                _ => {
                    panic!("not a program");
                }
            }
        }
    }

    #[test]
    fn test_return_statements() {
        let input = "
            return 5;
            return 10;
            return 993322;
        ";
        let lexer = lexer::Lexer::new( String::from(input) );
        let mut p = Parser::new(lexer);
        let program = p.parse_program();
        match program {
            NodeKind::ProgramNode{statements} => {
                assert_eq!(statements.len(), 3);
            },
            _ => {
                panic!("not a program");
            }
        }
    }

    #[test]
    fn test_identifier_expressions() {
        let input = "foobar;";
        let lexer = lexer::Lexer::new( String::from(input) );
        let mut p = Parser::new(lexer);
        let program = p.parse_program();
        match program {
            NodeKind::ProgramNode{statements} => {
                assert_eq!(statements.len(), 1);
                let first_statement = &statements[0];
                match first_statement {
                    StatementKind::ExpressionStatement{expression, ..} => {
                        match expression {
                            Some(exp) => {
                                let e = exp.clone();
                                match *e {
                                    ExpressionKind::Identifier{value, ..} => {
                                        assert_eq!(value, String::from("foobar"));
                                    },
                                    _ => panic!("not an expression")
                                }
                            },
                            _ => {
                                panic!("not an expression");
                            }
                        }
                    },
                    _ => {
                        panic!("no first statement");
                    }
                } 
            },
            _ => {
                panic!("not a program");
            }
        }
    }



    fn expression_test(type_t: &str, exp: ExpressionKind, expected: String) {
        match type_t {
            "INT" => {
                match exp {
                    ExpressionKind::IntegerLiteral{value, ..} => {
                        assert_eq!(value.to_string(), expected);
                    },
                    _ => panic!("Not a IntegerLiteral.")
                }
            },
            "BOOL" => {
                match exp {
                    ExpressionKind::BooleanExpression{value, ..} => {
                        assert_eq!(value.to_string(), expected);
                    },
                    _ => panic!("Not a Boolean: ")
                }
            },
            "IDENT" => {
                match exp {
                    ExpressionKind::Identifier{value, ..} => {
                        assert_eq!(value.to_string(), expected);
                    },
                    _ => panic!("Not a Identifier.")
                }
            },
            "STRING" => {
                match exp {
                    ExpressionKind::StringLiteral{value, ..} => {
                        assert_eq!(value.to_string(), expected);
                    },
                    _ => panic!("Not a StringLiteral.")
                }
            },
            "OBJECT" => {
                match exp {
                    ExpressionKind::ObjectLiteral{..} => {},
                    _ => panic!("Not an ObjectLiteral.")
                }
            },
            "SLOT_EXPRESSION" => {
                match exp {
                    ExpressionKind::SlotIdentiferExpression{parent, children, ..} => {
                        if expected != format!("{}.{}", parent, children.join(".")) {
                            panic!("Does not match.")
                        }
                    },
                    _ => panic!("Not an slot expression.")
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
        let program = p.parse_program();
        expression_test("STRING", unwrap_first_expression_from_program(program), String::from("hello world"));
    }

    #[test]
    fn test_slot_expression() {
        let input = "vehicle.wheels;";
        let lexer = lexer::Lexer::new(String::from(input));
        let mut p = Parser::new(lexer);
        let program = p.parse_program();
        expression_test("SLOT_EXPRESSION", unwrap_first_expression_from_program(program), String::from("vehicle.wheels"));
    }

    #[test]
    fn test_mulitiple_slot_expression() {
        let input = "vehicle.wheels.tires;";
        let lexer = lexer::Lexer::new(String::from(input));
        let mut p = Parser::new(lexer);
        let program = p.parse_program();
        expression_test("SLOT_EXPRESSION", unwrap_first_expression_from_program(program), String::from("vehicle.wheels.tires"));
    }

    #[test]
    fn test_slot_assign_statement() {
        let input = "vehicle.wheels.tires := fast;";
        let lexer = lexer::Lexer::new(String::from(input));
        let mut p = Parser::new(lexer);
        let program = p.parse_program();
        let stmt = unwrap_first_statement_from_program(program);
        match stmt {
            StatementKind::SlotAssignmentStatement{slot, value, ..} => {
                match slot {
                    Some(s) => {
                        match *s {
                            ExpressionKind::SlotIdentiferExpression{parent, ..} => {
                                assert_eq!(parent, String::from("vehicle"));
                            },
                            _ => {
                                panic!("not a slot identifier expression");
                            }
                        }
                    },
                    _ => {
                        panic!("slot not found");
                    }
                }
                match value {
                    Some(v) => {
                        match *v {
                            ExpressionKind::Identifier{value, ..} => {
                                assert_eq!(value, String::from("fast"));
                            },
                            _ => {
                                panic!("no value for slot");
                            }
                        }
                    },
                    _ => {
                        panic!("value not found");
                    }
                }
            },
            _ => {
                panic!("not a slot assignment statement");
            }
        }
    }    

    fn unwrap_first_expression_from_program(program: NodeKind) -> ExpressionKind {
        let st = unwrap_first_statement_from_program(program);
        match st {
            StatementKind::ExpressionStatement{expression, ..} => {
                match expression {
                    Some(exp) => {
                        return *exp.clone()
                    },
                    None => panic!("can't unwrap expression since expression statement is None.")
                }
            },
            _ => {
                panic!("not an expression statement")
            }
        }
    }

    fn unwrap_first_statement_from_program(program: NodeKind) -> StatementKind {
        match program {
            NodeKind::ProgramNode{statements, ..} => {
                let ref mut stmt = statements.get(0);
                match stmt {
                    Some(st) => {
                        return st.clone()
                    },
                    None => panic!("no statements")
                }
            },
            _ => panic!("not a program")
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let tests = vec![(
			"-a * b",
			"((-a) * b)",
		), (
			"!-a",
			"(!(-a))",
		),
		(
			"a + b + c",
			"((a + b) + c)",
		),
		(
			"a + b - c",
			"((a + b) - c)",
		),
		(
			"a * b * c",
			"((a * b) * c)",
		),
		(
			"a * b / c",
			"((a * b) / c)",
		),
		(
			"a + b / c",
			"(a + (b / c))",
		),
		(
			"a + b * c + d / e - f",
			"(((a + (b * c)) + (d / e)) - f)",
		),
		(
			"3 + 4; -5 * 5",
			"(3 + 4)((-5) * 5)",
		),
		(
			"5 > 4 == 3 < 4",
			"((5 > 4) == (3 < 4))",
		), (
			"5 < 4 != 3 > 4",
			"((5 < 4) != (3 > 4))",
		),
		(
			"3 + 4 * 5 == 3 * 1 + 4 * 5",
			"((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
		),
		(
			"true",
			"true",
		),
		(
			"false",
			"false",
		),
		(
			"3 > 5 == false",
			"((3 > 5) == false)",
		),
		(
			"3 < 5 == true",
			"((3 < 5) == true)",
		),
		(
			"1 + (2 + 3) + 4",
			"((1 + (2 + 3)) + 4)",
		),
		(
			"(5 + 5) * 2",
			"((5 + 5) * 2)",
		),
		(
			"2 / (5 + 5)",
			"(2 / (5 + 5))",
		),
		(
			"-(5 + 5)",
			"(-(5 + 5))",
		),
		(
			"!(true == true)",
			"(!(true == true))",
		),
		(
			"a + add(b * c) + d",
			"((a + add((b * c))) + d)",
		),
		(
			"add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
			"add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
		),
		(
			"add(a + b + c * d / f + g)",
			"add((((a + b) + ((c * d) / f)) + g))",
		),
		(
			"add(a + b + c * d / f + g % 5)",
			"add((((a + b) + ((c * d) / f)) + (g % 5)))",
		),
		(
			"a * [1, 2, 3, 4][b * c] * d",
			"((a * ([1, 2, 3, 4][(b * c)])) * d)",
		),
		(
			"add(a * b[2], b[1], 2 * [1, 2][1])",
			"add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))",
		)];
        for test in tests {
            let lexer = lexer::Lexer::new(String::from(test.0));
            let mut p = Parser::new(lexer);
            let program = p.parse_program();
            let program_str = program.clone().string();
            match program {
                NodeKind::ProgramNode{..} => {
                    assert_eq!(program_str, test.1);
                },
                _ => {
                    panic!("not a program");
                }
            }
        }
    }

    #[test]
    fn test_parse_prefix_expressions() {
        let tests = vec![
            ("!5;", ("!", ("INT", 5.to_string()))),
    		("-15;", ("-", ("INT", 15.to_string()))),
    		("!true;", ("!", ("BOOL", true.to_string()))),
    		("!false;", ("!", ("BOOL", false.to_string()))),
        ];
        for test in tests {
            let lexer = lexer::Lexer::new(String::from(test.0));
            let mut p = Parser::new(lexer);
            let program = p.parse_program();
            match program {
                NodeKind::ProgramNode{statements, ..} => {
                    assert_eq!(statements.len(), 1);
                    let ref exp_stmt = statements[0];
                    match exp_stmt {
                        StatementKind::ExpressionStatement{expression, ..} => {
                            let outputs = test.1;
                            match expression {
                                Some(e) => {
                                    match **e { // TODO: why does this need to be dereffed twice?
                                        ExpressionKind::PrefixExpression{ref operator, ref right, ..} => {
                                            assert_eq!(operator, outputs.0);
                                            let output = outputs.1;
                                            match right {
                                                Some(r) => {
                                                    expression_test(output.0, *r.clone(), String::from(output.1));
                                                },
                                                None => panic!("right not found.")
                                            }
                                        },
                                        _ => panic!("not a PrefixExpression")
                                    }
                                },
                                None => panic!("no value found.")
                            }
                        },
                        _ => panic!("not a ExpressionStatement")
                    }
                },
                _ => {}
            }
            
        }
    }

}
