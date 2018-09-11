use std::fmt;
use std::collections::HashMap;
use std::mem::discriminant;

use super::ast::StatementKind;
use super::ast::ExpressionKind;

use super::environment::Environment;

#[derive(Clone)]
pub enum ObjectKind {
    Integer{slots: HashMap<String, ObjectKind>, value: u32},
    Boolean{value: bool},
    Null,
    ReturnValue{value: Box<ObjectKind>},
    Error{message: String},
    Function{slots: HashMap<String, ObjectKind>, parameters: Vec<ExpressionKind>, body: StatementKind, env: Environment},
    LObject{slots: HashMap<String, ObjectKind>},
    StringObj{slots: HashMap<String, ObjectKind>, value: String},
    BuiltIn,
    Array{slots: HashMap<String, ObjectKind>, elements: Vec<ObjectKind>}
}

impl ObjectKind {

    pub fn variant_eq(self, b: &ObjectKind) -> bool {
        discriminant(&self) == discriminant(b)
    }

    pub fn get_from_slots(self, key: String) -> ObjectKind {
        match self {
            ObjectKind::Integer{slots, ..} | ObjectKind::LObject{slots, ..} => {
                let found = slots.get(&key);
                match found {
                    Some(v) => {
                        v.clone()
                    },
                    _ => {
                        ObjectKind::Error{message: String::from("Error finding key")}
                    }
                }
            },
            _ => {
                panic!("not implmented");
            }
        }
    }

    fn remove_from_slots(&mut self, key: String) {
        match self {
            ObjectKind::Integer{slots, ..} => {
                slots.remove(&key);
            },
            _ => {
                panic!("not implmented");
            }
        }
    }

    pub fn add_to_slots(&mut self, key: String, value: ObjectKind) {
        match self {
            ObjectKind::Integer{slots, ..} => {
                slots.insert(key, value);
            },
            ObjectKind::LObject{slots, ..} => {
                slots.insert(key, value);
            },
            _ => {
                panic!("not implmented");
            }
        }
    }
}

impl fmt::Display for ObjectKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ObjectKind::Integer{value, ..} => {
                write!(f, "{}", value)
            },
            ObjectKind::Error{message, ..} => {
                write!(f, "{}", message)
            },
            ObjectKind::Null => {
                write!(f, "{}", "Null")
            },
            ObjectKind::StringObj{value, ..} => {
                write!(f, "{}", value)
            },
            ObjectKind::LObject{slots, ..} => {
                for (key, value) in slots {
                    println!("  - {}: \"{}\"", key, &value);
                }
                write!(f, "{}", "Object")
            },
            _ => {
                write!(f, "{}", "display not implmented")
            }
        }
    }
}