use std::fmt;
use std::mem::discriminant;
use std::sync::Arc;
use std::sync::Mutex;

use super::ast::StatementKind;
use super::ast::ExpressionKind;

use super::environment::Environment;

#[derive(Clone)]
pub enum ObjectKind {
    Integer{slots: Arc<Mutex<Environment>>, value: u32},
    Boolean{value: bool},
    Null,
    ReturnValue{value: Box<ObjectKind>},
    Error{message: String},
    Function{slots: Arc<Mutex<Environment>>, parameters: Vec<ExpressionKind>, body: StatementKind, env: Arc<Mutex<Environment>>},
    LObject{slots: Arc<Mutex<Environment>>},
    StringObj{slots: Arc<Mutex<Environment>>, value: String},
    BuiltIn,
    Array{slots: Arc<Mutex<Environment>>, elements: Vec<ObjectKind>}
}

impl ObjectKind {

    pub fn variant_eq(self, b: &ObjectKind) -> bool {
        discriminant(&self) == discriminant(b)
    }

    pub fn get_from_slots(self, key: String) -> ObjectKind {
        match self {
            ObjectKind::Integer{slots, ..} | ObjectKind::LObject{slots, ..} => {
                let val = slots.lock().unwrap().get(key).clone();
                return val.lock().unwrap().clone();
            },
            _ => {
                panic!("not implmented");
            }
        }
    }

    fn remove_from_slots(&mut self, key: String) {
        match self {
            ObjectKind::Integer{slots, ..} => {
                slots.lock().unwrap().remove(key);
            },
            _ => {
                panic!("not implmented");
            }
        }
    }

    pub fn add_to_slots(&mut self, key: String, value: Arc<Mutex<ObjectKind>>) {
        match self {
            ObjectKind::Integer{slots, ..} => {
                slots.lock().unwrap().insert(key, value);
            },
            ObjectKind::LObject{slots, ..} => {
                slots.lock().unwrap().insert(key, value);
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
                let mut slots = slots.clone();
                for (key, value) in slots.lock().unwrap().store.clone() {
                    let value = value.lock().unwrap().clone();
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