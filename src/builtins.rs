use super::object::ObjectKind;

fn print(args: Vec<ObjectKind>) -> ObjectKind {
    print!("{:?}", args);
    return ObjectKind::Null;
}

fn println(args: Vec<ObjectKind>) -> ObjectKind {
    let first_arg = args.get(0);
    match first_arg {
        Some(arg) => {
            match arg {
                ObjectKind::StringObj{value, ..} => println!("{}", value),
                _ => println!("{:?}", arg)
            }
        },
        _ => {}
    }
    return ObjectKind::Null;
}

fn assert(args: Vec<ObjectKind>) -> ObjectKind {
    if args.len() == 1 {
        let first_arg = args.get(0);
        match first_arg {
            Some(arg) => {
                match arg {
                    ObjectKind::Boolean{value, ..} => {
                        if *value {
                            return ObjectKind::Null{};
                        } else {
                            panic!("Assertion Error {}", value);
                        }
                    },
                    _ => {
                        return ObjectKind::Error{message: "Argument must be Boolean".to_string()};
                    }
                }
            },
            _ => {}
        }
    }
    return ObjectKind::Error{message: "One argument is required".to_string()};
}


pub fn eval_builtin(func_name: String) -> Result<ObjectKind, String> {
    match func_name.as_ref() {
        "print" => {
            return Ok(ObjectKind::BuiltIn{func: print});
        },
        "println" => {
            return Ok(ObjectKind::BuiltIn{func: println});
        },        
        "assert" => {
            return Ok(ObjectKind::BuiltIn{func: assert});
        },
        _ => {
            return Err("builtin not found".to_string());
        }
    }
}
