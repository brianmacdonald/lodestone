use std::collections::HashMap;

use super::astenum::NodeKind;
use super::astenum::StatementKind;
use super::astenum::ExpressionKind;

use super::object::ObjectKind;
use super::environment::Environment;

fn eval(node: NodeKind, env: &Environment) -> ObjectKind {
	match node {
        NodeKind::StatementNode{statementKind} => {
            match statementKind {
                StatementKind::ExpressionStatement{token, expression} => {
                    match expression {
                        Some(exp) => {
                            eval(NodeKind::ExpressionNode{expressionKind: *exp}, env)
                        },
                        _ => {
                            panic!("not implmented");
                        }
                    }
                },
                _ => {
                    panic!("not implmented");
                }                
            }
        },
        NodeKind::ExpressionNode{expressionKind} => {
            match expressionKind {
                ExpressionKind::IntegerLiteral{token, value} => {
                    ObjectKind::Integer{slots: HashMap::new(), value: value}
                },
                _ => {
                    panic!("not implmented");
                }
            }
        },
        _ => {
            panic!("not implmented");
        }        
    }
}



fn eval_program(program: NodeKind, env: Environment) -> ObjectKind {
    match program {
        NodeKind::ProgramNode{statements} => {
            for s in statements {
                let sNode = NodeKind::StatementNode{statementKind: s};
                let result = eval(sNode, &env);
                match result {
                    ObjectKind::ReturnValue{value} => {
                        return *value;
                    },
                    ObjectKind::Error{..} => {
                        return result
                    },
                    _ => {}
                }
            }
        },
        _ => {}
    }
    ObjectKind::Null
}

fn eval_prefix_expression(operator: String, right: ObjectKind) -> ObjectKind {
	match operator.as_ref() {
        "!" => {
            eval_bang_operator_expression(right)
        },
        "-" => {
            eval_minus_prefix_operator_expression(right)
        },
        _ => {
            ObjectKind::Error{message: String::from("operator error")}
        }
    }
}

fn native_bool_to_boolean_object(input: bool) -> ObjectKind {
	ObjectKind::Boolean{value: input}
}

fn is_truthy(obj: ObjectKind) -> bool {
    match obj {
        ObjectKind::Boolean{value} => {
            value
        },
        ObjectKind::Null => {
            false
        },
        _ => {
            false
        }
    }
}

fn eval_bang_operator_expression(right: ObjectKind) -> ObjectKind {
    match right {
        ObjectKind::Boolean{value} => {
            ObjectKind::Boolean{value: !value}
        },
        ObjectKind::Null => {
            ObjectKind::Boolean{value: true}
        },
        _ => {
            ObjectKind::Boolean{value: false}
        }
    }
}

fn eval_minus_prefix_operator_expression(right: ObjectKind) -> ObjectKind {	
    match right {
        ObjectKind::Integer{slots, value} => {
            panic!("not implmented");
            //ObjectKind::Integer{slots: slots, value: -value}
        },
        _ => {
            ObjectKind::Error{message: String::from("operator error")}
        }
    }
}