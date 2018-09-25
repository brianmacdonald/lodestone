use std::collections::HashMap;

use std::rc::Rc;
use std::cell::RefCell;

use super::object::ObjectKind;

pub type LodeEnvironment = Rc<RefCell<HashMap<String, ObjectKind>>>;

#[derive(Clone, Debug)]
pub struct Environment {}

impl Environment {
    pub fn new() -> LodeEnvironment {
        return Rc::new(RefCell::new(HashMap::new()))
    }

    pub fn get(store: LodeEnvironment, key: String) -> ObjectKind {
        let store = store.borrow();
        let found = store.get(&key);
        match found {
            Some(v) => {
                return v.to_owned();
            },
            _ => {
                return ObjectKind::Null;
            }
        }
    }

    pub fn insert(store: LodeEnvironment, key: String, value: ObjectKind) {
        store.borrow_mut().insert(key, value);
    }

}