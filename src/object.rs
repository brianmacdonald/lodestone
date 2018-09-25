use std::fmt;
use std::mem::discriminant;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use super::ast::StatementKind;
use super::ast::ExpressionKind;

use super::environment::Environment;

pub type BuiltinFunction = fn(Vec<ObjectKind>) -> ObjectKind;

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
    BuiltIn{func: BuiltinFunction},
    Array{slots: Rc<RefCell<HashMap<String, ObjectKind>>>, elements: Vec<ObjectKind>}
}

impl ObjectKind {

    pub fn new_lobject() -> ObjectKind {
        return ObjectKind::LObject{slots: Environment::new()};
    }

    pub fn variant_eq(self, b: &ObjectKind) -> bool {
        discriminant(&self) == discriminant(b)
    }

    pub fn deep_clone(self) -> ObjectKind {
        let cloned = self.clone();
        match self {
            ObjectKind::LObject{slots, ..} => {
                let new_env = Environment::new();
                let value = slots.borrow().clone();
                for (key, val) in &value {
                    let copied_val = val.to_owned().deep_clone();
                    Environment::insert(new_env.clone(), key.to_string(), copied_val);
                }
                return ObjectKind::LObject{slots: Rc::new(RefCell::new(value))};
            },
            _ => {}
        }
        return cloned;
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
        let rest_paths = paths.split_off(1);
        let first = paths.get(0);
        match first {
            Some(f) => {
                let first_path = f.clone();
                // Check for existing child with that identifier
                let child = &self.clone().get(f.clone());
                match child {
                    ObjectKind::LObject{..} => {
                        let mut child = child.clone();
                        if rest_paths.len() > 0 {
                            child.set_child(value, &mut rest_paths.clone());
                        } else {
                            child.set(f.to_string(), value);
                        }
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
            ObjectKind::Boolean{value, ..} => {
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
            ObjectKind::LObject{..} => {
                //let mut slots = slots.clone();
                // write!(f, "{:#?}", slots)
                write!(f, "{}", "LObject")
            },
            _ => {
                write!(f, "{}", "display not implmented")
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn unwrap_lobject_slots(lobject: ObjectKind) -> Rc<RefCell<HashMap<String, ObjectKind>>>  {
        match lobject {
            ObjectKind::LObject{slots, ..} => {
                return slots;
            },
            _ => panic!("not an LObject.")
        }
    }

    #[test]
    fn test_get_slot() {
        let mut test = ObjectKind::new_lobject();
        test.set("key".to_string(), &mut ObjectKind::StringObj{value: "val".to_string(), slots: Environment::new()});
        assert_eq!(test.get("key".to_string()).to_string(), "val".to_string());
    }

    #[test]
    fn test_set_slot() {
        let mut test = ObjectKind::new_lobject();
        test.set("key".to_string(), &mut ObjectKind::StringObj{value: "val".to_string(), slots: Environment::new()});
        let slots = unwrap_lobject_slots(test);
        assert_eq!(slots.borrow().get("key").unwrap().to_string(), "val".to_string());
    }

    #[test]
    fn test_deep_clone_clone() {
        let mut first = ObjectKind::new_lobject();
        first.set("key_one".to_string(), &mut ObjectKind::StringObj{value: "val".to_string(), slots: Environment::new()});
        let second = first.deep_clone();
        assert_eq!(second.get("key_one".to_string()).to_string(), "val".to_string());
    }

    #[test]
    fn test_deep_clone_set() {
        let mut first = ObjectKind::new_lobject();
        first.set("key_one".to_string(), &mut ObjectKind::StringObj{value: "val".to_string(), slots: Environment::new()});
        let mut second = first.deep_clone();
        second.set("key_two".to_string(), &mut ObjectKind::StringObj{value: "val2".to_string(), slots: Environment::new()});
        assert_eq!(second.get("key_two".to_string()).to_string(), "val2".to_string());
    }


    #[test]
    fn test_deep_clone_child_object() {
        let mut first = ObjectKind::new_lobject();
        first.set("key_one".to_string(), &mut ObjectKind::StringObj{value: "val1".to_string(), slots: Environment::new()});
        let mut second = first.deep_clone();
        second.set("key_two".to_string(), &mut ObjectKind::StringObj{value: "val2".to_string(), slots: Environment::new()});
        let mut third = ObjectKind::new_lobject();
        third.set("key_three".to_string(), &mut ObjectKind::StringObj{value: "val3".to_string(), slots: Environment::new()});
        second.set("key_four".to_string(), &mut third);
        let forth = second.deep_clone();
        let third_slots = unwrap_lobject_slots(forth.get("key_four".to_string()));
        assert_eq!(third_slots.borrow().get("key_three").unwrap().to_string(), "val3".to_string());
    }

    #[test]
    fn test_variant_eq() {
        let tests = vec![
            (ObjectKind::Boolean{value: true}, ObjectKind::Boolean{value: true}, true, "Boolean and Boolean should be true"),
            (ObjectKind::Boolean{value: true}, ObjectKind::Null, false, "Boolean and Null should be false"),
        ];
        for test in tests {
            let result = test.0.variant_eq(&test.1);
            assert_eq!(result, test.2, "test_variant_eq: {}", test.3.to_string());
        }
    }

}