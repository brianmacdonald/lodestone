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

    pub fn deep_clone(self) -> ObjectKind {

        return self.clone();
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