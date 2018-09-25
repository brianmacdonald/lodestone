use std::sync::Arc;
use std::sync::Mutex;

use super::ast::NodeKind;
use super::ast::StatementKind;
use super::ast::ExpressionKind;

use super::builtins::eval_builtin;

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
                                            Environment::insert(store, name_value, val.deep_clone());
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
        ObjectKind::Integer{..} => {
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
        ObjectKind::Integer{value: lvalue, ..} => {
            match right {
                ObjectKind::Integer{value: rvalue, ..} => {
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
                            return ObjectKind::Boolean{value: lvalue < rvalue};
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
    let env_e = env.clone();
    match ie {
        ExpressionKind::IfExpression{condition, consequence, alternative, ..} => {
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
            let mut val = Environment::get(env, value.clone());
            match val {
                ObjectKind::Null => {
                    match eval_builtin(value) {
                        Ok(func) => {
                            return func;
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
            //let mut val = env_val.borrow_mut().clone();
            return val;
        },
        _ => {
            // TODO: Add builtins check here.
            return ObjectKind::Error{message: String::from("Ident not found.")};
        }
    }
}

fn eval_slot_identifier(parent: String, mut children: Vec<String>, env: Rc<RefCell<HashMap<String, ObjectKind>>>) -> ObjectKind {
    let parent_val = Environment::get(env, parent);
    match parent_val.clone() {
        ObjectKind::LObject{slots, ..} => {
            let mut slots = slots.clone();
            if children.len() > 1 {
                let rest_vec = children.split_off(1);
                let first = children.get(0);
                match first {
                    Some(f) => {
                        return eval_slot_identifier(f.to_string(), rest_vec, slots);
                    },
                    _ => {}
                }
            } else {
                let first = children.get(0);
                match first {
                    Some(f) => {
                        let slot_val = Environment::get(slots, f.to_string());
                        return slot_val.clone();
                    },
                    _ => { 
                        panic!("Slot is required to have one child.")
                    }
                }
            }
        },
        _ => panic!("null pointer error!")
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
        ObjectKind::BuiltIn{func} => {
            return unwrap_return_value(func(args));
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
        ObjectKind::ReturnValue{value} => *value,
        _ => obj
    }
}

fn eval_string_infix_expression (operator: String, left: ObjectKind, right: ObjectKind) -> ObjectKind {
    if operator != "+" {
        return ObjectKind::Error{message: String::from("not a valid operator.")};
    }
    match left {
        ObjectKind::StringObj{value: l_value, ..} => {
            match right {
                ObjectKind::StringObj{value: r_value, ..} => {
                    let concat = format!("{}{}", l_value, r_value);
                    return ObjectKind::StringObj{slots: Environment::new(), value: concat};
                },
                _ => panic!("right is not a string.")
            }
        },
        _ => panic!("left is not a string.")
    }
}

fn is_error(obj: ObjectKind) -> bool {
    match obj {
        ObjectKind::Error{..} => true,
        _ => false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bool_value_unwrap(obj: ObjectKind) -> bool {
        match obj {
            ObjectKind::Boolean{value} => value,
            _ => panic!("cannot unwrap bool value. not a Boolean object")
        }
    }

    fn error_message_unwrap(obj: ObjectKind) -> String {
        match obj {
            ObjectKind::Error{message} => message,
            _ => panic!("cannot unwrap error message. not an Error object")
        }
    }

    fn create_integer(value: u32) -> ObjectKind {
        return ObjectKind::Integer{value: value, slots: Environment::new()};
    }

    fn create_boolean(value: bool) -> ObjectKind {
        return ObjectKind::Boolean{value: value};
    }

    #[test]
    fn test_bang_prefix_expression() {
        let result = eval_prefix_expression("!".to_string(), create_boolean(false));
        assert!(bool_value_unwrap(result));
    }

    #[test]
    #[should_panic]
    fn test_minus_prefix_expression() {
        eval_prefix_expression("-".to_string(), create_integer(42));
    }

    #[test]
    fn test_unsupported_prefix_expression() {
        let result = eval_prefix_expression("%".to_string(), create_integer(42));
        assert!(error_message_unwrap(result) != "".to_string());
    }

    #[test]
    fn test_native_bool_to_boolean_object() {
        let tests = vec![
            (true, true, "true should return true object"),
            (false, false, "false should return false object"),
        ];
        for test in tests {
            let result = native_bool_to_boolean_object(test.0);
            assert_eq!(bool_value_unwrap(result), test.1, "test_is_truthy: {}", test.2.to_string());
        }
    }

    #[test]
    fn test_is_truthy() {
        let tests = vec![
            (ObjectKind::Boolean{value: true}, true, "true should return false"),
            (ObjectKind::Boolean{value: false}, false, "false should return true"),
            (ObjectKind::Null{}, false, "null should return false"),
            (ObjectKind::new_lobject(), false, "non-null should return false"),
        ];
        for test in tests {
            let result = is_truthy(test.0);
            assert_eq!(result, test.1, "test_is_truthy: {}", test.2.to_string());
        }
    }

    #[test]
    fn test_eval_operator_expression() {
        let tests = vec![
            (ObjectKind::Boolean{value: true}, false, "true should return false"),
            (ObjectKind::Boolean{value: false}, true, "false should return true"),
            (ObjectKind::Null{}, true, "null should return true"),
            (ObjectKind::new_lobject(), false, "non-null should return false"),
        ];
        for test in tests {
            let result = eval_bang_operator_expression(test.0);
            assert_eq!(bool_value_unwrap(result), test.1, "test_eval_operator_expression: {}", test.2.to_string());
        }
    }

    #[test]
    #[should_panic]
    fn test_eval_minus_prefix_operator_expression() {
        eval_minus_prefix_operator_expression(ObjectKind::Integer{value: 42, slots: Environment::new()});
    }

    #[test]
    fn test_eval_integer_infix_expression() {
        let tests = vec![
            ("+", create_integer(9), create_integer(9), "18", "9 + 9 = 18"),
            ("-", create_integer(9), create_integer(9), "0", "9 - 9 = 0"),
            ("*", create_integer(9), create_integer(9), "81", "9 * 9 = 81"),
            ("/", create_integer(18), create_integer(9), "2", "18/9 = 2"),
            ("%", create_integer(15), create_integer(2), "1", "15 % 2 = 1"),
            ("<", create_integer(10), create_integer(2), "false", "10 < 2 == false"),
            ("<", create_integer(1), create_integer(2), "true", "1 < 2 == true"),
            (">", create_integer(10), create_integer(2), "true", "10 > 2 == true"),
            (">", create_integer(1), create_integer(2), "false", "1 > 2 == false"),
            ("==", create_integer(10), create_integer(9), "false", "10 == 9 = false"),
            ("==", create_integer(10), create_integer(10), "true", "10 == 10 = true"),
            ("!=", create_integer(10), create_integer(10), "false", "10 != 10 = false"),
            ("!=", create_integer(10), create_integer(9), "true", "10 != 10 = true")
        ];
        for test in tests {
            let result = eval_integer_infix_expression(test.0.to_string(), test.1, test.2);
            assert_eq!(result.to_string(), test.3, "test_eval_integer_infix_expression: {}", test.4.to_string());
        }
    }

    #[test]
    fn test_eval_string_infix_expression_unsupported_operator() {
        let result = eval_string_infix_expression("!".to_string(), ObjectKind::Null, ObjectKind::Null);
        match result {
            ObjectKind::Error{..} => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    #[should_panic]
    fn test_eval_string_infix_expression_null_null_objects() {
        eval_string_infix_expression("+".to_string(), ObjectKind::Null, ObjectKind::Null);
    }

    #[test]
    #[should_panic]
    fn test_eval_string_infix_expression_string_null_objects() {
        eval_string_infix_expression("+".to_string(), ObjectKind::StringObj{value: "".to_string(), slots: Environment::new()}, ObjectKind::Null);
    }

    #[test]
    fn test_eval_string_infix_expression_string_string_objects() {
        let result = eval_string_infix_expression("+".to_string(), 
            ObjectKind::StringObj{value: "hello ".to_string(), slots: Environment::new()},
            ObjectKind::StringObj{value: "world".to_string(), slots: Environment::new()});
        assert_eq!(result.to_string(), "hello world".to_string());   
    }

    #[test]
    fn test_unwrap_return_value() {
        let tests = vec![
            (ObjectKind::ReturnValue{value: Box::new(create_integer(43))}, "43", "Should unwrap return value from ReturnValue object."),
            (create_integer(55), "55", "Should return object for non-return value objects."),
        ];
        for test in tests {
            let result = unwrap_return_value(test.0);
            assert_eq!(result.to_string(), test.1, "test_unwrap_return_value: {}", test.2);
        }
    }

    #[test]
    fn test_is_error() {
        let tests = vec![
            (ObjectKind::Error{message: "".to_string()}, true, "Error is error"),
            (ObjectKind::Null{}, false, "Error is not error"),
        ];
        for test in tests {
            let result = is_error(test.0);
            assert_eq!(result, test.1, "test_is_error: {}", test.2);
        }
    }

}