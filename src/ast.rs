use std::vec::Vec;

use super::token;

trait Node {
    fn token_literal(&mut self) -> String;
    fn string(&mut self) -> String;
}

trait ParentNode {
    fn get_node() -> Node;
}

trait Statement: Node {
    fn statementNode(&mut self);
}

struct Expression {

}

impl Expression {
    fn expression_node(&mut self) {}
    fn string(&mut self) -> String {
        return String::from("");
    }
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
    value: Option<Expression>
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
    fn statementNode(&mut self) {}
}

struct Identifier {
    token: token::Token,
    value: String
}

impl Identifier {
    fn expressionNode() {}
    fn token_literal(&mut self) -> String {
        self.token.literal.clone()
    }
    fn string(&mut self) -> String {
        self.value.clone()
    }
}

struct ReturnStatement {
    token: token::Token,
    return_value: Option<Expression>
}

impl ReturnStatement {
    fn statement_node() {}
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

impl ExpressionStatement {
    fn statement_node() {}
    fn token_literal(&mut self) -> String {
        self.token.literal.clone()
    }
    fn string(&mut self) -> String {
        self.expression.string().clone()
    }
}

struct IntegerLiteral {
    token: token::Token,
    return_value: Expression
}

impl IntegerLiteral {
    fn statement_node() {}
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

impl PrefixExpression {
    fn expressionNode() {}
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
    left: Expression,
    operator: String,
    right: Expression
}

impl InfixExpression {
    fn expression_node() {}
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

impl Boolean {
    fn statement_node() {}
    fn token_literal(&mut self) -> String {
        self.token.literal.clone()
    }
    fn string(&mut self) -> String {
        self.token.literal.clone()
    }
}

struct IfExpression {
    token: token::Token,
    condition: Expression,
    consequence: BlockStatement,
    alternative: Option<BlockStatement>
}

impl IfExpression {
    fn statement_node() {}
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

impl BlockStatement {
    fn statement_node() {}
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

impl FunctionLiteral {
    fn expression_node() {}
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
    function: Expression,
    arguments: Vec<Expression>
}

impl CallExpression {
    fn expression_node() {}
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

impl StringLiteral {
    fn expression_node() {}
    fn token_literal(&mut self) -> String {
        self.token.literal.clone()
    }
    fn string(&mut self) -> String {
        self.token.literal.clone()
    }
}

struct WhileLiteral {
    token: token::Token,
    condition: Expression,
    consequence: BlockStatement
}

impl WhileLiteral {
    fn expression_node() {}
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
    elements: Vec<Expression>
}

impl ArrayLiteral {
    fn expression_node() {}
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
    left: Expression,
    index: Expression
}

impl IndexExpression {
    fn expression_node() {}
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
        let ident = Identifier {value: String::from("myVar"), token: token::Token { literal: String::from("let"),  t_type: token::LET } };
        let exp = Some(Expression {});
        statements.push(Box::new(LetStatement {value: exp, name: ident, token: token::Token { literal: String::from("let"),  t_type: token::LET }}) as Box<Statement>);
        let mut program = Program {statements: statements};
        assert_eq!(program.string(), "let myVar = ");
    }

}
