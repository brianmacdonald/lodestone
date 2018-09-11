use std::collections::HashMap;

use super::ast::NodeKind;
use super::ast::StatementKind;
use super::ast::ExpressionKind;

use super::object::ObjectKind;
use super::environment::Environment;

pub fn eval(node: NodeKind, env: &mut Environment) -> ObjectKind {
	match node {
        NodeKind::ProgramNode{statements} => {
            return eval_program(statements, env);
        },
        NodeKind::StatementNode{statementKind} => {
            match statementKind {
                StatementKind::LetStatement{token, name, value} => {
                    match value {
                        Some(v) => {
                            let val = eval(NodeKind::ExpressionNode{expressionKind: *v}, env);
                            match val {
                                ObjectKind::Error{..} => {
                                    return val;
                                },
                                _ => {
                                    match name {
                                        ExpressionKind::Identifier{token, value: name_value} => {
                                            env.insert(name_value, val);
                                        },
                                        _ => {}
                                    }
                                }
                            }
                        },
                        _ => {}
                    }
                },
                StatementKind::ReturnStatement{return_value, ..} => {
                    match return_value {
                        Some(return_val) => {
                            let val = eval(NodeKind::StatementNode{statementKind: *return_val}, env);
                            match is_error(val.clone()) {
                                true => {
                                    return val.clone();
                                },
                                _ => {}
                            }
                            return ObjectKind::ReturnValue{value: Box::new(val)};
                        },
                        None => {}
                    }
                    panic!("not implmented");
                },
                StatementKind::AssignStatement{name, slot_name, value, ..} => {
                    match value {
                        Some(w_val) => {
                            match name {
                                ExpressionKind::Identifier{value: val, ..} => {
                                    println!("For some reason this pulled from env: {}", val);
                                    let mut e_val = env.get(val);
                                    let eval_val = eval(NodeKind::ExpressionNode{expressionKind: *w_val}, env);
                                    e_val.add_to_slots(slot_name, eval_val);
                                },
                                _ => {}
                            }
                        },
                        _ => {}
                    }
                },
                StatementKind::ExpressionStatement{expression, ..} => {
                    match expression {
                        Some(exp) => {
                            return eval(NodeKind::ExpressionNode{expressionKind: *exp}, env);
                        },
                        _ => {
                            panic!("expression not implmented: ");
                        }
                    }
                },
                StatementKind::BlockStatement{..} => {
                    return eval_block_statement(statementKind, env);
                }
                _ => {
                    panic!("not implmented");
                }                
            }
        },
        NodeKind::ExpressionNode{expressionKind: expression_kind} => {
            match expression_kind {
                ExpressionKind::SlotIdentifer{ parent, child, ..} => {
                    println!("slot ident");
                    return eval_slot_identifier(parent, child, env);
                },
                ExpressionKind::Identifier{..} => {
                    println!("ident");
                    return eval_identifier(expression_kind, env);
                },
                ExpressionKind::PrefixExpression{token, operator, right} => {
                    match right {
                        Some(r) => {
                            let eval_right = eval(NodeKind::ExpressionNode{expressionKind: *r}, env);
                            match eval_right {
                                ObjectKind::Error{..} => {
                                    return eval_right;
                                },
                                _ => {
                                    return eval_prefix_expression(operator, eval_right);
                                }
                            }
                        },
                        _ => {}
                    }
                    panic!("right part of prefix not found.");
                },
                ExpressionKind::InfixExpression{operator, left, right, ..} => {
                    match left {
                        Some(l) => {
                            let eval_left = eval(NodeKind::ExpressionNode{expressionKind: *l}, env);
                            match is_error(eval_left.clone()) {
                                true => {
                                    return eval_left.clone();
                                },
                                _ => {}
                            }
                            match right {
                                Some(r) => {
                                    let eval_right = eval(NodeKind::ExpressionNode{expressionKind: *r}, env);
                                    match is_error(eval_right.clone()) {
                                        true => {
                                            return eval_right.clone();
                                        },
                                        _ => {}
                                    }
                                    return eval_infix_expression(operator, eval_left, eval_right);
                                },
                                _ => {}
                            }
                        }, 
                        _ => {}
                    }
                },
                ExpressionKind::BooleanExpression{token, value} => {
                    return native_bool_to_boolean_object(value);
                },
                ExpressionKind::IfExpression{..} => {
                    return eval_if_expression(expression_kind, env);
                },
                ExpressionKind::FunctionLiteral{token, parameters, body} => {
                    return ObjectKind::Function{slots: HashMap::new(), parameters: parameters, body: *body, env: env.clone()};
                },
                ExpressionKind::ObjectLiteral{..} => {
                    return ObjectKind::LObject{slots: HashMap::new()};
                },
                ExpressionKind::CallExpression{function, arguments, ..} => {
                    let func = eval(NodeKind::ExpressionNode{expressionKind: *function}, env);

                    let args = eval_expressions(arguments, env);
                    match args.len() == 1 {
                        true => {
                            match args.get(0) {
                                Some(arg) => {
                                    let first_arg = arg.clone();
                                    match is_error(first_arg) {
                                        true => {
                                            return arg.clone();
                                        },
                                        _ => {}
                                    }
                                },
                                _ => {}
                            }
                        },
                        _ => {}
                    }
                    return apply_function(func, args);
                },
                ExpressionKind::StringLiteral{value, ..} => {
                    return ObjectKind::StringObj{slots: HashMap::new(), value};
                },
                ExpressionKind::IntegerLiteral{token, value} => {
                    return ObjectKind::Integer{slots: HashMap::new(), value};
                },
                _ => {
                    panic!("not implmented");
                }
            }
        }    
    }
    return ObjectKind::Null;
}

fn eval_program(statements: Vec<StatementKind>, env: &mut Environment) -> ObjectKind {

    for s in statements {
        let s_node = NodeKind::StatementNode{statementKind: s};
        let result = eval(s_node, env);
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
    return ObjectKind::Null;
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

fn eval_infix_expression(operator: String, left: ObjectKind, right: ObjectKind) -> ObjectKind {
    match left {
        ObjectKind::Integer{..} => {
            match right {
                ObjectKind::Integer{..} => {
                    return eval_integer_infix_expression(operator, left, right);
                },
                _ => {}
            }
        },
        ObjectKind::Boolean{value: l_value, ..} => {
            match right {
                ObjectKind::Boolean{value: r_value} => {
                    match operator.as_ref() {
                        "==" => {
                            return native_bool_to_boolean_object(l_value == r_value);
                        },
                        "!=" => {
                            return native_bool_to_boolean_object(l_value != r_value);
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }
        ObjectKind::StringObj{..} => {
            let left_clone = left.clone();
            match left_clone.variant_eq(&right) {
                true => {
                    return eval_string_infix_expression(operator, left.clone(), right)
                },
                _ => {}
            }
        },
        _ => {}
    }
    match left.variant_eq(&right) {
        false => {
            panic!("infix operator not valid for types");
        },
        _ => {}
    }
    panic!("not implmented");
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

fn eval_block_statement(block: StatementKind, env: &mut Environment) -> ObjectKind {
    let mut result = ObjectKind::Error{message: String::from("block statement error")};

    match block {
        StatementKind::BlockStatement{statements, ..} => {
            for statement in statements {
                result = eval(NodeKind::StatementNode{statementKind:*statement}, env);
                match result {
                    ObjectKind::ReturnValue{..} | ObjectKind::Error{..} => {
                        return result.clone();
                    },
                    _ => {}
                }
            }
        },
        _ => {}
    }
    return result.clone();
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
            panic!("not implmented. need to change to sized int");
            //ObjectKind::Integer{slots: slots, value: -value}
        },
        _ => {
            ObjectKind::Error{message: String::from("operator error")}
        }
    }
}


fn eval_integer_infix_expression (operator: String, left: ObjectKind, right: ObjectKind) -> ObjectKind {
    match left {
        ObjectKind::Integer{slots: lslots, value: lvalue} => {
            match right {
                ObjectKind::Integer{slots: rslots, value: rvalue} => {
                    match operator.as_ref() {
                        "+" => {   
                            return ObjectKind::Integer{slots: HashMap::new(), value: lvalue + rvalue};
                        },
                        "-" => {
                            return ObjectKind::Integer{slots: HashMap::new(), value: lvalue - rvalue};
                        },
                        "*" => {
                            return ObjectKind::Integer{slots: HashMap::new(), value: lvalue * rvalue};
                        },
                        "/" => {
                            return ObjectKind::Integer{slots: HashMap::new(), value: lvalue / rvalue};
                        },
                        "%" => {
                            return ObjectKind::Integer{slots: HashMap::new(), value: lvalue % rvalue};
                        },
                        "<" => {
                            return ObjectKind::Boolean{value: lvalue > rvalue};
                        },
                        ">" => {
                            return ObjectKind::Boolean{value: lvalue > rvalue};
                        },
                        "==" => {
                            return ObjectKind::Boolean{value: lvalue == rvalue};
                        },
                        "!=" => {
                            return ObjectKind::Boolean{value: lvalue != rvalue};
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        },
        _ => {}
    }
    ObjectKind::Error{message: String::from("operator error")}
}

fn eval_if_expression(ie: ExpressionKind, env: &mut Environment) -> ObjectKind {
    match ie {
        ExpressionKind::IfExpression{token, condition, consequence, alternative} => {
            match condition {
                Some(c) => {
                    let evaluated_condition = eval(NodeKind::ExpressionNode{expressionKind:*c}, env);
                    match evaluated_condition {
                        ObjectKind::Error{..} => {
                            return evaluated_condition;
                        },
                        _ => {
                            if is_truthy(evaluated_condition) {
                                match consequence {
                                    Some(con) => {
                                        return eval(NodeKind::StatementNode{statementKind:*con}, env);
                                    },
                                    _ => {}
                                }
                            } else {
                                match alternative {
                                    Some(alt) => {
                                        return eval(NodeKind::StatementNode{statementKind:*alt}, env);
                                    },
                                    _ => {}
                                }
                            }
                        }
                    }
                },
                _ => {}
            }
        },
        _ => {}
    }
    return ObjectKind::Null;
}



fn eval_identifier(node: ExpressionKind, env: &mut Environment) -> ObjectKind {
    match node {
        ExpressionKind::Identifier{token, value} => {
            // Design decision: we're pulling a value out of the environment here.
            //                  This basically makes its value immutable since changing
            //                  the value wont change it in the environment.
            println!("eval'ing indent {}", value);
            env.clone().get(value)
        },
        _ => {
            // TODO: Add builtins check here.
            ObjectKind::Error{message: String::from("Ident not found.")}
        }
    }
}

fn eval_slot_identifier(parent: Option<Box<ExpressionKind>>, child: Option<Box<ExpressionKind>>, env: &mut Environment) -> ObjectKind {

    match parent {
        Some(p) => {
            match *p {
                ExpressionKind::Identifier{value: parent_val, ..} => {
                    println!("getting parent from env: {}", parent_val);
                    let mut parent_in_env = env.get(parent_val);
                    match child {
                        Some(c) => {
                            let child_expression = *c.clone();
                            match child_expression {
                                ExpressionKind::SlotIdentifer{parent, child, value, ..} => {
                                    let c_obj = eval_slot_identifier(parent.clone(), child.clone(), env);
                                    println!("adding {} to slot", value);
                                    parent_in_env.add_to_slots(value, c_obj);
                                },
                                _ => {
                                    let eval_child = eval(NodeKind::ExpressionNode{expressionKind: *c}, env);
                                    parent_in_env.add_to_slots(String::from("bar"), eval_child);
                                }
                            }
                        },
                        _ => {}
                    }
                    return parent_in_env;
                },
                _ => {}
            }
        },
        _ => {}
    }
    panic!("slot identifiter not valid.")
}

fn eval_expressions(exps: Vec<Box<ExpressionKind>>, env: &mut Environment) -> Vec<ObjectKind> {
	let mut result = Vec::new();

	for e in exps {
        let expression_node = NodeKind::ExpressionNode{expressionKind: *e};
		let evaluated = eval(expression_node, env);
		match evaluated {
            ObjectKind::Error{..} => {
                panic!("not implmented");
            },
            _ => {}
        }
		result.push(evaluated);
	}
	return result;
}

fn apply_function(func: ObjectKind, args: Vec<ObjectKind>) -> ObjectKind {
    match func {
        ObjectKind::Function{parameters, body, env, ..} => {
            let fn_body = body.clone();
            let extended_env = extend_function_env(parameters, env, args);
            let evaluated = eval(NodeKind::StatementNode{statementKind: fn_body}, &mut extended_env.clone());
            return unwrap_return_value(evaluated);
        },
        _ => {
            panic!("not implmented");
        }
    }
}

fn extend_function_env(parameters: Vec<ExpressionKind>, env: Environment, args: Vec<ObjectKind>) -> Environment {
    let mut closure = env.clone();
    let mut param_index = 0;
    for param in parameters {
        match param {
            ExpressionKind::Identifier{token, value} => {
                match args.get(param_index) {
                    Some(arg) => {
                        closure.insert(value, arg.clone());
                    },
                    _ => {}
                }
            }, 
            _ => {}
        }
        param_index = param_index + 1;
    }
    return closure;
}

fn unwrap_return_value(obj: ObjectKind) -> ObjectKind {
	match obj {
        ObjectKind::ReturnValue{value} => {
            return *value;
        },
        _ => {
            return obj;
        }
    }
}

fn eval_string_infix_expression (operator: String, left: ObjectKind, right: ObjectKind) -> ObjectKind {
    match operator != "+" {
        true => {
            return ObjectKind::Error{message: String::from("not a valid operator.")};
        },
        _ => {}
    }

    match left {
        ObjectKind::StringObj{value: l_value, ..} => {
            match right {
                ObjectKind::StringObj{value: r_value, ..} => {
                    let concat = format!("{}{}", l_value, r_value);
                    return ObjectKind::StringObj{slots: HashMap::new(), value: concat};
                },
                _ => {
                    panic!("right is not a string.");
                }
            }
        },
        _ => {
            panic!("left is not a string.");
        }
    }
}

fn is_error(obj: ObjectKind) -> bool {
    match obj {
        ObjectKind::Error{..} => {
            true
        },
        _ => {
            false
        }
    }
}