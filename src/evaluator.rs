use std::sync::Arc;
use std::sync::Mutex;

use super::ast::NodeKind;
use super::ast::StatementKind;
use super::ast::ExpressionKind;

use super::object::ObjectKind;
use super::environment::Environment;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use super::token;

pub fn eval(node: NodeKind, env: Rc<RefCell<HashMap<String, ObjectKind>>>) -> ObjectKind {
	match node {
        NodeKind::ProgramNode{statements} => {
            return eval_program(statements, env);
        },
        NodeKind::StatementNode{statementKind} => {
            match statementKind {
                StatementKind::LetStatement{name, value, ..} => {
                    match value {
                        Some(v) => {
                            let store = env.clone();
                            let val = eval(NodeKind::ExpressionNode{expressionKind: *v}, env);
                            match val {
                                ObjectKind::Error{..} => {
                                    return val;
                                },
                                _ => {
                                    match name {
                                        ExpressionKind::Identifier{value: name_value, ..} => {
                                            Environment::insert(store, name_value, val);
                                        },
                                        _ => {}
                                    }
                                }
                            }
                        },
                        _ => {}
                    }
                },
                StatementKind::LetCloneStatement{name, value, ..} => {
                    match value {
                        Some(v) => {
                            let store = env.clone();
                            let val = eval(NodeKind::ExpressionNode{expressionKind: *v}, env);
                            match val {
                                ObjectKind::Error{..} => {
                                    return val;
                                },
                                _ => {
                                    match name {
                                        ExpressionKind::Identifier{value: name_value, ..} => {
                                            Environment::insert(store, name_value, val.clone());
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
                StatementKind::ExpressionStatement{expression, ..} => {
                    match expression {
                        Some(exp) => {
                            return eval(NodeKind::ExpressionNode{expressionKind: *exp}, env);
                        },
                        _ => {
                            panic!("Statement is expression but is None.");
                        }
                    }
                },
                StatementKind::BlockStatement{..} => {
                    return eval_block_statement(statementKind, env);
                },
                StatementKind::SlotAssignmentStatement{slot, value, ..} => {
                    println!("evaling slot...");
                    return eval_slot_assignment(slot, value, env);
                },
                _ => {
                    panic!("not implmented");
                }                
            }
        },
        NodeKind::ExpressionNode{expressionKind: expression_kind} => {
            match expression_kind {
                ExpressionKind::Identifier{..} => {
                    println!("ident");
                    return eval_identifier(expression_kind, env);
                },
                ExpressionKind::SlotIdentiferExpression{parent, children, ..} => {
                    return eval_slot_identifier(parent, children, env).clone();
                },
                ExpressionKind::PrefixExpression{operator, right, ..} => {
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
                    let mut env_e = env.clone();
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
                                    let eval_right = eval(NodeKind::ExpressionNode{expressionKind: *r}, env_e);
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
                    return ObjectKind::Function{slots: Environment::new(), parameters: parameters, body: *body, env: env.clone()};
                },
                ExpressionKind::ObjectLiteral{..} => {
                    return ObjectKind::LObject{slots: Environment::new()};
                },
                ExpressionKind::CallExpression{function, arguments, ..} => {
                    let mut env_e = env.clone();
                    let func = eval(NodeKind::ExpressionNode{expressionKind: *function}, env);
                    let args = eval_expressions(arguments, env_e);
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
                    return ObjectKind::StringObj{slots: Environment::new(), value};
                },
                ExpressionKind::IntegerLiteral{token, value} => {
                    return ObjectKind::Integer{slots: Environment::new(), value};
                },
                _ => {
                    panic!("not implmented");
                }
            }
        }    
    }
    return ObjectKind::Null;
}

fn eval_program(statements: Vec<StatementKind>, env: Rc<RefCell<HashMap<String, ObjectKind>>>) -> ObjectKind {

    for s in statements {
        let s_node = NodeKind::StatementNode{statementKind: s};
        let result = eval(s_node, env.clone());
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

fn eval_block_statement(block: StatementKind, env: Rc<RefCell<HashMap<String, ObjectKind>>>) -> ObjectKind {
    let mut result = ObjectKind::Error{message: String::from("block statement error")};
    match block {
        StatementKind::BlockStatement{statements, ..} => {
            let statement_size = statements.len();
            let mut count = 0;
            let mut env_e = env.clone();
            for statement in statements {
                result = eval(NodeKind::StatementNode{statementKind:*statement}, env_e.clone());
                count += 1;
                match result {
                    ObjectKind::ReturnValue{..} | ObjectKind::Error{..} => {
                        return result.clone();
                    },
                    _ => {
                        if count == statement_size {
                            return result.clone();
                        }
                    }
                }
            }
        },
        _ => {}
    }
    return result.clone();
}

fn eval_slot_assignment(slot: Option<Box<ExpressionKind>>, value: Option<Box<ExpressionKind>>, env: Rc<RefCell<HashMap<String, ObjectKind>>>) -> ObjectKind {
    match slot {
        Some(s) => {
            let slot_expression = *s.clone();
            match slot_expression {
                ExpressionKind::SlotIdentiferExpression{parent, mut children, ..} => {
                    match value {
                        Some(v) => {
                            //let e_env = env.clone();
                            let mut first_parent = Environment::get(env.clone(), parent.clone());
                            //let mut first_parent = env_val.borrow_mut();;
                            let mut val = eval(NodeKind::ExpressionNode{expressionKind: *v}, env);
                            println!("setting child slot of {} with {} ", parent, val);
                            first_parent.set_child(&mut val, &mut children);
                            return ObjectKind::Null{};
                        },
                        _ => {}
                    }
                },
                _ =>  {}
            }
        },
        _ =>  {}
    }
    panic!("slot assignment failed!")
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
                            return ObjectKind::Integer{slots: Environment::new(), value: lvalue + rvalue};
                        },
                        "-" => {
                            return ObjectKind::Integer{slots: Environment::new(), value: lvalue - rvalue};
                        },
                        "*" => {
                            return ObjectKind::Integer{slots: Environment::new(), value: lvalue * rvalue};
                        },
                        "/" => {
                            return ObjectKind::Integer{slots: Environment::new(), value: lvalue / rvalue};
                        },
                        "%" => {
                            return ObjectKind::Integer{slots: Environment::new(), value: lvalue % rvalue};
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

fn eval_if_expression(ie: ExpressionKind, env: Rc<RefCell<HashMap<String, ObjectKind>>>) -> ObjectKind {
    let mut env_e = env.clone();
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
                            let mut env = env_e.clone();
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



fn eval_identifier(node: ExpressionKind, env: Rc<RefCell<HashMap<String, ObjectKind>>>) -> ObjectKind {
    match node {
        ExpressionKind::Identifier{value, ..} => {
            // Design decision: we're pulling a value out of the environment here.
            //                  This basically makes its value immutable since changing
            //                  the value wont change it in the environment.
            println!("eval'ing indent {}", value);
            let mut val = Environment::get(env, value);
            //let mut val = env_val.borrow_mut().clone();
            return val;
        },
        _ => {
            // TODO: Add builtins check here.
            ObjectKind::Error{message: String::from("Ident not found.")}
        }
    }
}

fn eval_slot_identifier(parent: String, mut children: Vec<String>, env: Rc<RefCell<HashMap<String, ObjectKind>>>) -> ObjectKind {
    let parent_val = Environment::get(env, parent);
    //let parent_val = env_val.borrow();
    match parent_val.clone() {
        ObjectKind::LObject{slots, ..} => {
            let mut slots = slots.clone();
            if children.len() > 1 {
                let first_vec = children.split_off(1);
                let first = first_vec.get(0);
                match first {
                    Some(f) => {
                        return eval_slot_identifier(f.to_string(), children, slots);
                    },
                    _ => {}
                }
            } else {
                let first = children.get(0);
                match first {
                    Some(f) => {
                        let slot_val = Environment::get(slots, f.to_string());
                        //let slot_val = env_val.borrow();;
                        return slot_val.clone();
                    },
                    _ => {}
                }
            }
        },
        _ => {}
    }
    return ObjectKind::Null{};
}

fn eval_expressions(exps: Vec<Box<ExpressionKind>>, env: Rc<RefCell<HashMap<String, ObjectKind>>>) -> Vec<ObjectKind> {
	let mut result = Vec::new();

	for e in exps {
        let expression_node = NodeKind::ExpressionNode{expressionKind: *e};
        let mut env = env.clone();
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
            let evaluated = eval(NodeKind::StatementNode{statementKind: fn_body}, extended_env.clone());
            return unwrap_return_value(evaluated);
        },
        _ => {
            panic!("not implmented");
        }
    }
}

fn extend_function_env(parameters: Vec<ExpressionKind>, env: Rc<RefCell<HashMap<String, ObjectKind>>>, args: Vec<ObjectKind>) -> Rc<RefCell<HashMap<String, ObjectKind>>> {
    let closure = env.clone();
    let mut param_index = 0;
    for param in parameters {
        match param {
            ExpressionKind::Identifier{value, ..} => {
                match args.get(param_index) {
                    Some(arg) => {
                        Environment::insert(closure.clone(), value, arg.clone());
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
                    return ObjectKind::StringObj{slots: Environment::new(), value: concat};
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

#[cfg(test)]
mod tests {
    use super::*;

/*
    #[test]
    fn test_eval_program_kind_input() {
        let token = token::Token{t_type: token::IDENT, literal: String::from("")};
        let return_exp = Some(Box::new(ExpressionKind::Identifier{token: token.clone(), value: String::from("foobar")}));
        let return_value = Some(Box::new(StatementKind::ExpressionStatement{token: token.clone(), expression: return_exp}));
        let statement_vec = vec![StatementKind::ReturnStatement{token: token.clone(), return_value: return_value}];
        let program = NodeKind::ProgramNode{statements: statement_vec};
        let env = Environment::new();
        Environment::lock_insert(&env, String::from("foobar"), ObjectKind::StringObj{slots: Environment::new(), value: String::from("foobar")});
        let output = eval(program, env);
        match output {
            ObjectKind::StringObj{value, ..} => {
                assert_eq!(value, String::from("foobar"));
            },
            _ => {
                panic!("not an identifier.");
            }
        }
    }
    */


}