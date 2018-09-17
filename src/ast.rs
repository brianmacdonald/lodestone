use super::token;

#[derive(Hash, Clone)]
pub enum NodeKind {
    ExpressionNode{expressionKind: ExpressionKind},
    StatementNode{statementKind: StatementKind},
    ProgramNode{statements: Vec<StatementKind>}
}

impl NodeKind {
    fn token_literal(self) -> String {
        match self {
            NodeKind::ProgramNode{statements} => {
                let mut out = String::from("");
                if statements.len() > 0 {
                    out.push_str(&statements[1].clone().token_literal());
                }    
                out 
            },
            _ => {
                String::from("")
            }
        }
    }
    pub fn string(self) -> String {
        match self {
            NodeKind::ExpressionNode{expressionKind} => {
                expressionKind.string()
            },
            NodeKind::StatementNode{statementKind} => {
                statementKind.string()
            },
            NodeKind::ProgramNode{statements} => {
                let mut out = String::from("");
                for s in statements {
                    out.push_str(&s.string());
                }
                out
            }
        }
    }
}

#[derive(Hash, Clone)]
pub enum ExpressionKind {
    Identifier{token: token::Token, value: String},
    SlotIdentifer{token: token::Token, parent: Option<Box<ExpressionKind>>, child: Option<Box<ExpressionKind>>, value: String},
    SlotIdentiferExpression{token: token::Token, parent: String, children: Vec<String>},
    PrefixExpression{token: token::Token, operator: String, right: Option<Box<ExpressionKind>>},
    InfixExpression{token: token::Token, left: Option<Box<ExpressionKind>>, operator: String, right: Option<Box<ExpressionKind>>},
    BooleanExpression{token: token::Token, value: bool}, 
    IfExpression{token: token::Token, condition: Option<Box<ExpressionKind>>, consequence: Option<Box<StatementKind>>, alternative: Option<Box<StatementKind>>}, 
    ObjectLiteral{token: token::Token},
    FunctionLiteral{token: token::Token, parameters: Vec<ExpressionKind>, body: Box<StatementKind>},
    CallExpression{token: token::Token, function: Box<ExpressionKind>, arguments: Vec<Box<ExpressionKind>>},
    StringLiteral{token: token::Token, value: String},
    IntegerLiteral{token: token::Token, value: u32},
    WhileLiteral{token: token::Token, condition: Box<ExpressionKind>, consequence: Box<StatementKind>},
    ArrayLiteral{token: token::Token, elements: Vec<Box<ExpressionKind>>},
    IndexExpression{token: token::Token, left: Box<ExpressionKind>, index: Option<Box<ExpressionKind>>}
}

impl ExpressionKind {
    fn token_literal(self) -> String {
        match self {
            ExpressionKind::Identifier{token, ..} => {
                token.literal.clone()
            },
            ExpressionKind::SlotIdentifer{token, ..} => {
                token.literal.clone()
            },
            ExpressionKind::SlotIdentiferExpression{token, ..} => {
                token.literal.clone()
            },
            ExpressionKind::PrefixExpression{token, ..} => {
                token.literal.clone()
            },
            ExpressionKind::InfixExpression{token, ..} => {
                token.literal.clone()
            },
            ExpressionKind::IfExpression{token, ..} => {
                token.literal.clone()
            },
            ExpressionKind::BooleanExpression{token, ..} => {
                token.literal.clone()
            }
            ExpressionKind::FunctionLiteral{token, ..} => {
                token.literal.clone()
            },
            ExpressionKind::CallExpression{token, ..} => {
                token.literal.clone()
            },
            ExpressionKind::StringLiteral{token, ..} => {
                token.literal.clone()
            },
            ExpressionKind::IntegerLiteral{token, ..} => {
                token.literal.clone()
            },
            ExpressionKind::WhileLiteral{token, ..} => {
                token.literal.clone()
            },
            ExpressionKind::ArrayLiteral{token, ..} => {
                token.literal.clone()
            },
            ExpressionKind::IndexExpression{token, ..} => {
                token.literal.clone()
            },
            ExpressionKind::ObjectLiteral{token, ..} => {
                token.literal.clone()
            },
        }
    }
    fn string(self) -> String {
        match self {
            ExpressionKind::Identifier{token, value} => {
                value
            },
            ExpressionKind::SlotIdentifer{child, parent, ..} => {
                String::from("slot")
            },
            ExpressionKind::SlotIdentiferExpression{parent, children, ..} => {
                let mut out = String::from("SlotIdentifer(");
                out.push_str(&parent);
                out.push_str(".");
                for child in children {
                    out.push_str(&child);
                }
                out.push_str(")");
                out
            },
            ExpressionKind::PrefixExpression{token, operator, right} => {
                let mut out = String::from("(");
                out.push_str(&operator);
                match right {
                    Some(x) => {
                        out.push_str(&x.string());
                    }
                    None => {}
                }
                out.push_str(")");
                out
            },
            ExpressionKind::InfixExpression{token, left, operator, right} => {
                let mut out = String::from("(");
                match left {
                    Some(s) => {
                        out.push_str(&s.string());
                    },
                    _ => {}
                }
                out.push_str(" ");
                out.push_str(&operator);
                out.push_str(" ");
                match right {
                    Some(s) => {
                        out.push_str(&s.string());
                    },
                    _ => {}
                }
                out.push_str(")");
                out
            },
            ExpressionKind::BooleanExpression{token, value} => {
                token.literal.clone()
            },
            ExpressionKind::IfExpression{token, condition, consequence, alternative} => {
                let mut out = String::from("if");
                match condition {
                    Some(c) => {
                        out.push_str(&c.string());
                    },
                    _ => {}
                }
                out.push_str(" ");
                match consequence {
                    Some(c) => {
                        out.push_str(&c.string());
                    },
                    _ => {}
                }
                out.push_str(" ");
                match alternative {
                    Some(a) => {
                        out.push_str("else ");
                        out.push_str(&a.string());
                    },
                    _ => {}
                }
                out
            },
            ExpressionKind::FunctionLiteral{token, parameters, body} => {
                let mut out = String::from("");
                out.push_str(&token.literal.clone());
                out.push_str("(");
                let parameters = parameters;
                for p in parameters {
                    out.push_str(&p.string());
                }
                out.push_str(")");
                out.push_str(&body.string());
                out
            },
            ExpressionKind::CallExpression{token, function, arguments} => {
                let mut out = String::from("");
                out.push_str(&function.string());
                out.push_str("(");
                let mut args_vec = vec![];
                for a in arguments {
                    let arg = &a.string();
                    args_vec.push(arg.clone());
                }
                out.push_str(&args_vec.join(", "));
                out.push_str(")");
                out
            },
            ExpressionKind::StringLiteral{token, value} => {
                value
            },
            ExpressionKind::IntegerLiteral{token, value} => {
                token.literal.clone()
            },
            ExpressionKind::WhileLiteral{token, condition, consequence} => {
                let mut out = String::from("");
                out.push_str(&token.literal.clone());
                out.push_str("(");
                out.push_str(&condition.string());
                out.push_str(") ");
                out.push_str(&consequence.string());
                out
            },
            ExpressionKind::ArrayLiteral{token, elements} => {
                let mut out = String::from("[");
                let elements = elements;
                let mut eles_vec = vec![];
                for e in elements {
                    let ele = &e.string();
                    eles_vec.push(ele.clone());
                }
                out.push_str(&eles_vec.join(", "));
                out.push_str("]");
                out
            },
            ExpressionKind::IndexExpression{token, left, index} => {
                let mut out = String::from("");
                out.push_str("(");
                out.push_str(&left.string());
                out.push_str("[");
                match index {
                    Some(i) => {
                        out.push_str(&i.string());
                    }
                    _ => {}
                }
                out.push_str("])");
                out
            },
            ExpressionKind::ObjectLiteral{token, ..} => {
                String::from("ObjectLiteral")
            }
        }
    }
}

#[derive(Hash, Clone)]
pub enum StatementKind {
    LetStatement{token: token::Token, name: ExpressionKind, value: Option<Box<ExpressionKind>>}, 
    LetCloneStatement{token: token::Token, name: ExpressionKind, value: Option<Box<ExpressionKind>>},
    SlotAssignmentStatement{token: token::Token, slot: Option<Box<ExpressionKind>>, value: Option<Box<ExpressionKind>>}, 
    ReturnStatement{token: token::Token, return_value: Option<Box<StatementKind>>},
    ExpressionStatement{token: token::Token, expression: Option<Box<ExpressionKind>>},
    BlockStatement{token: token::Token, statements: Vec<Box<StatementKind>>},
    NullStatement
}

impl StatementKind {

    fn token_literal(self) -> String {
        String::from("")    
    }
    
    fn string(self) -> String {
        match self {
            StatementKind::LetStatement{token, name, value} => {
                let mut out = String::from("");
                out.push_str(&token.literal.clone());
                out.push_str(" ");
                out.push_str(&name.string());
                out.push_str(" = ");
                match value {
                    Some(x) => {
                        out.push_str(&x.string());
                    }
                    _ => {}
                }
                out
            },
            StatementKind::ReturnStatement{token, return_value} => {
                let mut out = String::from("");
                out.push_str(&token.literal.clone());
                out.push_str(" ");
                match return_value {
                    Some(rv) => {
                        out.push_str(&rv.string());
                    }
                    _ => {}
                }
                out.push_str(";");
                out
            },
            StatementKind::ExpressionStatement{token, expression} => {
                match expression {
                    Some(x) => {
                        return x.string().clone();
                    },
                    _ => {
                        return String::from("");
                    }
                }
            },
            StatementKind::BlockStatement{token, statements} => {
                let mut out = String::from("");
                for s in statements {
                    out.push_str(&s.string());
                }
                out
            },
            _ => {
                String::from("needs string")
            }
        }
    }

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indentifer_string() {
        let indentifer = ExpressionKind::Identifier{token: token::Token { literal: String::from("let"), t_type: token::LET}, value: String::from("5")};
        assert_eq!(indentifer.string(), "5");
    }

    #[test]
    fn test_prefix_expression_string() {
        let exp = ExpressionKind::Identifier{token: token::Token { literal: String::from("let"), t_type: token::LET}, value: String::from("5")};
        let prefix = ExpressionKind::PrefixExpression{token: token::Token { literal: String::from("let"), t_type: token::LET},
            operator: String::from("+"),
            right: Some(Box::new(exp))};
        assert_eq!(prefix.string(), "(+5)");
    }    

}
