use std::fmt;
use std::mem::discriminant;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use super::ast::StatementKind;
use super::ast::ExpressionKind;

use super::environment::Environment;

#[derive(Clone, Debug)]
pub enum ObjectKind {
    Integer{slots: Rc<RefCell<HashMap<String, ObjectKind>>>, value: u32},
    Boolean{value: bool},
    Null,
    ReturnValue{value: Box<ObjectKind>},
    Error{message: String},
    Function{slots: Rc<RefCell<HashMap<String, ObjectKind>>>, parameters: Vec<ExpressionKind>, body: StatementKind, env: Rc<RefCell<HashMap<String, ObjectKind>>>},
    LObject{slots: Rc<RefCell<HashMap<String, ObjectKind>>>},
    StringObj{slots: Rc<RefCell<HashMap<String, ObjectKind>>>, value: String},
    BuiltIn,
    Array{slots: Rc<RefCell<HashMap<String, ObjectKind>>>, elements: Vec<ObjectKind>}
}

impl ObjectKind {

    pub fn variant_eq(self, b: &ObjectKind) -> bool {
        discriminant(&self) == discriminant(b)
    }

    pub fn deep_clone(self) -> ObjectKind {

        return self.clone();
    }

    pub fn get(self, key: String) -> ObjectKind {
        match self {
            ObjectKind::LObject{slots, ..} => {
                return Environment::get(slots, key);
            },
            _ => {}
        }
        return ObjectKind::Null;
    }

    pub fn set(&mut self, key: String, value: &mut ObjectKind) {
        match self {
            ObjectKind::LObject{slots, ..} => {
                Environment::insert(slots.clone(), key, value.clone());
            },
            _ => {}
        }
    }

    pub fn set_child(&mut self, value: &mut ObjectKind, paths: &mut Vec<String>) {
        let first_path_vec = paths.split_off(0);
        let first = first_path_vec.get(0);
        println!("first: option {:?}", first_path_vec);
        match first {
            Some(f) => {
                println!("first {}", f);
                let first_path = f.clone();
                let child = &self.clone().get(f.clone());
                match child {
                    ObjectKind::LObject{..} => {
                        let mut child = child.clone();
                        child.set_child(value, &mut paths.clone());
                        self.set(first_path, &mut child);       
                    },
                    _ => {
                        self.set(f.to_string(), value);
                    }
                }
            },
            _ => {}
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
                write!(f, "{:#?}", slots)
            },
            _ => {
                write!(f, "{}", "display not implmented")
            }
        }
    }
}