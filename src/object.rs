use std::collections::HashMap;

use super::astenum::NodeKind;
use super::astenum::StatementKind;
use super::astenum::ExpressionKind;

#[derive(Clone)]
pub enum ObjectKind {
    Integer{slots: HashMap<String, ObjectKind>, value: u32},
    Boolean{value: bool},
    Null,
    ReturnValue{value: Box<ObjectKind>},
    Error{message: String},
    Function{slots: HashMap<String, ObjectKind>, parameters: Vec<ExpressionKind>, body: StatementKind},
    StringObj{slots: HashMap<String, ObjectKind>, value: String},
    BuiltIn,
    Array{slots: HashMap<String, ObjectKind>, elements: Vec<ObjectKind>}
}

impl ObjectKind {

    fn get_from_slots(self, key: String) -> ObjectKind {
        match self {
            ObjectKind::Integer{slots, ..} => {
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

    fn add_to_slots(&mut self, key: String, value: ObjectKind) {
        match self {
            ObjectKind::Integer{slots, ..} => {
                slots.insert(key, value);
            },
            _ => {
                panic!("not implmented");
            }
        }
    }
}