use std::vec::Vec;

use super::token;

trait Node {
    fn token_literal(&mut self) -> String;
    fn string(&mut self) -> String;
}

trait Statement: Node {
    fn statement_node(&mut self);
}

trait Expression: Node {
    fn expression_node(&mut self) {}
}

struct Program {
    statements: Vec<Box<Statement>>
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

struct LetStatement {
    token: token::Token,
    name: Identifier,
    value: Option<Box<Expression>>
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

struct Identifier {
    token: token::Token,
    value: String
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

struct ReturnStatement {
    token: token::Token,
    return_value: Option<Box<Expression>>
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

struct ExpressionStatement {
    token: token::Token,
    expression: Expression
}

impl Statement for ExpressionStatement {
    fn statement_node(&mut self) {}
}

impl Node for ExpressionStatement {
    fn token_literal(&mut self) -> String {
        self.token.literal.clone()
    }
    fn string(&mut self) -> String {
        self.expression.string().clone()
    }
}

struct IntegerLiteral {
    token: token::Token,
    value: u16
}

impl Statement for IntegerLiteral {
    fn statement_node(&mut self) {}
}

impl Node for IntegerLiteral {
    fn token_literal(&mut self) -> String {
        self.token.literal.clone()
    }
    fn string(&mut self) -> String {
        self.token.literal.clone()
    }
}

struct PrefixExpression {
    token: token::Token,
    operator: String,
    right: Expression
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
        out.push_str(&self.right.string());
        out.push_str(")");
        out
    }
}

struct InfixExpression {
    token: token::Token,
    left: Box<Expression>,
    operator: String,
    right: Expression
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
        out.push_str(&self.left.string());
        out.push_str(" ");
        out.push_str(&self.operator);
        out.push_str(" ");
        out.push_str(&self.right.string());
        out.push_str(")");
        out
    }
}

struct Boolean {
    token: token::Token,
    value: bool
}

impl Statement for Boolean {
    fn statement_node(&mut self) {}
}

impl Node for Boolean {
    fn token_literal(&mut self) -> String {
        self.token.literal.clone()
    }
    fn string(&mut self) -> String {
        self.token.literal.clone()
    }
}

struct IfExpression {
    token: token::Token,
    condition: Box<Expression>,
    consequence: BlockStatement,
    alternative: Option<BlockStatement>
}

impl Statement for IfExpression {
    fn statement_node(&mut self) {}
}

impl Node for IfExpression {
    fn token_literal(&mut self) -> String {
        self.token.literal.clone()
    }
    fn string(&mut self) -> String {
        let mut out = String::from("if");
        out.push_str(&self.condition.string());
        out.push_str(" ");
        out.push_str(&self.consequence.string());
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

struct BlockStatement {
    token: token::Token,
    statements: Vec<Box<Statement>>
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
        for s in &mut self.statements {
            out.push_str(&s.string());
        }
        out
    }
}

struct FunctionLiteral {
    token: token::Token,
    parameters: Vec<Identifier>,
    body: BlockStatement
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
        for p in &mut self.parameters {
            out.push_str(&p.string());
        }
        out.push_str(")");
        out.push_str(&self.body.string());
        out
    }
}

struct CallExpression {
    token: token::Token,
    function: Box<Expression>,
    arguments: Vec<Box<Expression>>
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
        for a in &mut self.arguments {
            out.push_str(", ");
            out.push_str(&a.string());
        }
        out.push_str(")");
        out
    }
}

struct StringLiteral {
    token: token::Token,
    value: String
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

struct WhileLiteral {
    token: token::Token,
    condition: Box<Expression>,
    consequence: BlockStatement
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

struct ArrayLiteral {
    token: token::Token,
    elements: Vec<Box<Expression>>
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
        for e in &mut self.elements {
            out.push_str(", ");
            out.push_str(&e.string());
        }
        out.push_str("]");
        out
    }
}

struct IndexExpression {
    token: token::Token,
    left: Box<Expression>,
    index: Expression
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
        out.push_str(&self.index.string());
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
