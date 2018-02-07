use std::vec::Vec;

use super::token;

pub trait Node {
    fn token_literal(&mut self) -> String;
    fn string(&mut self) -> String;
}

pub trait Statement: Node {
    fn statement_node(&mut self);
}

pub trait Expression: Node {
    fn expression_node(&mut self) {}
}

pub struct Program {
    pub statements: Vec<Box<Statement>>
}

impl Node for Program {
    fn token_literal(&mut self) -> String {
        if self.statements.len() > 0 {
            return self.statements[0].token_literal();
        }
        String::from("")
    }
    fn string(&mut self) -> String {
        let mut out = String::from("");
        for s in &mut self.statements {
            out.push_str(&s.string());
        }
        out
    }
}

pub struct LetStatement {
    pub token: token::Token,
    pub name: Identifier,
    pub value: Option<Box<Expression>>
}

impl Node for LetStatement {
    fn token_literal(&mut self) -> String {
        self.token.literal.clone()
    }
    fn string(&mut self) -> String {
        let mut out = String::from("");
        out.push_str(&self.token_literal());
        out.push_str(" ");
        out.push_str(&self.name.string());
        out.push_str(" = ");
        match self.value {
            Some(ref mut x) => {
                out.push_str(&x.string());
            }
            _ => {}
        }
        return out;
    }
}

impl Statement for LetStatement {
    fn statement_node(&mut self) {}
}

pub struct Identifier {
    pub token: token::Token,
    pub value: String
}

impl Expression for Identifier {
    fn expression_node(&mut self) {}
}

impl Node for Identifier {
    fn token_literal(&mut self) -> String {
        self.token.literal.clone()
    }
    fn string(&mut self) -> String {
        self.value.clone()
    }
}

pub struct ReturnStatement {
    pub token: token::Token,
    pub return_value: Option<Box<Expression>>
}

impl Statement for ReturnStatement {
    fn statement_node(&mut self) {}
}

impl Node for ReturnStatement {
    fn token_literal(&mut self) -> String {
        self.token.literal.clone()
    }
    fn string(&mut self) -> String {
        let mut out = String::from("");
        out.push_str(&self.token_literal());
        out.push_str(" ");
        match self.return_value {
            Some(ref mut rv) => {
                out.push_str(&rv.string());
            }
            _ => {}
        }
        out.push_str(";");
        out
    }
}

pub struct ExpressionStatement {
    pub token: token::Token,
    pub expression: Option<Box<Expression>>
}

impl Statement for ExpressionStatement {
    fn statement_node(&mut self) {}
}

impl Node for ExpressionStatement {
    fn token_literal(&mut self) -> String {
        self.token.literal.clone()
    }
    fn string(&mut self) -> String {
        let ref mut expression = self.expression;
        match *expression {
            Some(ref mut x) => {
                return x.string().clone()
            },
            None => {
                return String::from("");
            }
        }
    }
}

pub struct IntegerLiteral {
    pub token: token::Token,
    pub value: u32
}

impl Expression for IntegerLiteral {
    fn expression_node(&mut self) {}
}

impl Node for IntegerLiteral {
    fn token_literal(&mut self) -> String {
        self.token.literal.clone()
    }
    fn string(&mut self) -> String {
        self.token.literal.clone()
    }
}

pub struct PrefixExpression {
    pub token: token::Token,
    pub operator: String,
    pub right: Option<Box<Expression>>
}

impl Expression for PrefixExpression {
    fn expression_node(&mut self) {}
}

impl Node for PrefixExpression {
    fn token_literal(&mut self) -> String {
        self.token.literal.clone()
    }
    fn string(&mut self) -> String {
        let mut out = String::from("(");
        out.push_str(&self.operator);
        let ref mut right = self.right;
        match *right {
            Some(ref mut x) => {
                out.push_str(&x.string());
            }
            None => {}
        }
        out.push_str(")");
        out
    }
}

pub struct InfixExpression {
    pub token: token::Token,
    pub left: Option<Box<Expression>>,
    pub operator: String,
    pub right: Option<Box<Expression>>
}

impl Expression for InfixExpression {
    fn expression_node(&mut self) {}
}

impl Node for InfixExpression {
    fn token_literal(&mut self) -> String {
        self.token.literal.clone()
    }
    fn string(&mut self) -> String {
        let mut out = String::from("(");
        let ref mut left_option = self.left;
        match *left_option {
            Some(ref mut s) => {
                out.push_str(&s.string());
            },
            _ => {}
        }
        out.push_str(" ");
        out.push_str(&self.operator);
        out.push_str(" ");
        let ref mut right_option = self.right;
        match *right_option {
            Some(ref mut s) => {
                out.push_str(&s.string());
            },
            _ => {}
        }
        out.push_str(")");
        out
    }
}

pub struct Boolean {
    pub token: token::Token,
    pub value: bool
}

impl Expression for Boolean {
    fn expression_node(&mut self) {}
}

impl Node for Boolean {
    fn token_literal(&mut self) -> String {
        self.token.literal.clone()
    }
    fn string(&mut self) -> String {
        self.token.literal.clone()
    }
}

pub struct IfExpression {
    pub token: token::Token,
    pub condition: Option<Box<Expression>>,
    pub consequence: Option<BlockStatement>,
    pub alternative: Option<BlockStatement>
}

impl Expression for IfExpression {
    fn expression_node(&mut self) {}
}

impl Node for IfExpression {
    fn token_literal(&mut self) -> String {
        self.token.literal.clone()
    }
    fn string(&mut self) -> String {
        let mut out = String::from("if");
        let ref mut condition = self.condition;
        match *condition {
            Some(ref mut c) => {
                out.push_str(&c.string());
            },
            _ => {}
        }
        out.push_str(" ");
        let ref mut consequence = self.consequence;
        match *consequence {
            Some(ref mut c) => {
                out.push_str(&c.string());
            },
            _ => {}
        }
        out.push_str(" ");
        match self.alternative {
            Some(ref mut a) => {
                out.push_str("else ");
                out.push_str(&a.string());
            },
            _ => {}
        }
        out
    }
}

pub struct BlockStatement {
    pub token: token::Token,
    pub statements: Vec<Box<Statement>>
}

impl Statement for BlockStatement {
    fn statement_node(&mut self) {}
}

impl Node for BlockStatement {
    fn token_literal(&mut self) -> String {
        self.token.literal.clone()
    }
    fn string(&mut self) -> String {
        let mut out = String::from("");
        let ref mut statements = self.statements;
        for s in statements {
            out.push_str(&s.string());
        }
        out
    }
}

pub struct FunctionLiteral {
    pub token: token::Token,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement
}

impl Expression for FunctionLiteral {
    fn expression_node(&mut self) {}
}

impl Node for FunctionLiteral {
    fn token_literal(&mut self) -> String {
        self.token.literal.clone()
    }
    fn string(&mut self) -> String {
        let mut out = String::from("");
        out.push_str(&self.token_literal());
        out.push_str("(");
        let ref mut parameters = self.parameters;
        for p in parameters {
            out.push_str(&p.string());
        }
        out.push_str(")");
        out.push_str(&self.body.string());
        out
    }
}

pub struct CallExpression {
    pub token: token::Token,
    pub function: Box<Expression>,
    pub arguments: Vec<Box<Expression>>
}

impl Expression for CallExpression {
    fn expression_node(&mut self) {}
}

impl Node for CallExpression {
    fn token_literal(&mut self) -> String {
        self.token.literal.clone()
    }
    fn string(&mut self) -> String {
        let mut out = String::from("");
        out.push_str(&self.function.string());
        out.push_str("(");
        let ref mut args = self.arguments;
        for a in args {
            out.push_str(", ");
            out.push_str(&a.string());
        }
        out.push_str(")");
        out
    }
}

pub struct StringLiteral {
    pub token: token::Token,
    pub value: String
}

impl Expression for StringLiteral {
    fn expression_node(&mut self) {}
}

impl Node for StringLiteral {
    fn token_literal(&mut self) -> String {
        self.token.literal.clone()
    }
    fn string(&mut self) -> String {
        self.token.literal.clone()
    }
}

pub struct WhileLiteral {
    pub token: token::Token,
    pub condition: Box<Expression>,
    pub consequence: BlockStatement
}

impl Expression for WhileLiteral {
    fn expression_node(&mut self) {}
}

impl Node for WhileLiteral {
    fn token_literal(&mut self) -> String {
        self.token.literal.clone()
    }
    fn string(&mut self) -> String {
        let mut out = String::from("");
        out.push_str(&self.token_literal());
        out.push_str("(");
        out.push_str(&self.condition.string());
        out.push_str(") ");
        out.push_str(&self.consequence.string());
        out
    }
}

pub struct ArrayLiteral {
    pub token: token::Token,
    pub elements: Vec<Box<Expression>>
}

impl Expression for ArrayLiteral {
    fn expression_node(&mut self) {}
}

impl Node for ArrayLiteral {
    fn token_literal(&mut self) -> String {
        self.token.literal.clone()
    }
    fn string(&mut self) -> String {
        let mut out = String::from("");
        out.push_str("[");
        let ref mut elements = self.elements;
        for e in elements {
            let ele = e;
            out.push_str(", ");
            out.push_str(&ele.string());
        }
        out.push_str("]");
        out
    }
}

pub struct IndexExpression {
    pub token: token::Token,
    pub left: Box<Expression>,
    pub index: Option<Box<Expression>>
}

impl Expression for IndexExpression {
    fn expression_node(&mut self) {}
}

impl Node for IndexExpression {
    fn token_literal(&mut self) -> String {
        self.token.literal.clone()
    }
    fn string(&mut self) -> String {
        let mut out = String::from("");
        out.push_str("(");
        out.push_str(&self.left.string());
        out.push_str("[");
        let ref mut index = self.index;
        match *index {
            Some(ref mut i) => {
                out.push_str(&i.string());
            }
            _ => {}
        }
        out.push_str("])");
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let mut statements = Vec::new();
        let ident = Identifier {value: String::from("myVarNew"), token: token::Token { literal: String::from("let"),  t_type: token::LET } };
        let exp = Some(Box::new(Identifier {value: String::from("myVarOld"), token: token::Token { literal: String::from("let"),  t_type: token::LET } }) as Box<Expression>);
        statements.push(Box::new(LetStatement {value: exp, name: ident, token: token::Token { literal: String::from("let"),  t_type: token::LET }}) as Box<Statement>);
        let mut program = Program {statements: statements};
        assert_eq!(program.string(), "let myVarNew = myVarOld");
    }

}
