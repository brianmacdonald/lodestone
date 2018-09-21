use std::collections::HashMap;

use std::rc::Rc;
use std::cell::RefCell;

use super::object::ObjectKind;

#[derive(Clone, Debug)]
pub struct Environment {}

impl Environment {
    pub fn new() -> Rc<RefCell<HashMap<String, ObjectKind>>> {
        return Rc::new(RefCell::new(HashMap::new()))
    }

    pub fn get(store: Rc<RefCell<HashMap<String, ObjectKind>>>, key: String) -> ObjectKind {
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

    pub fn insert(store: Rc<RefCell<HashMap<String, ObjectKind>>>, key: String, value: ObjectKind) {
        println!("Inserting {} === in ==> {} for #{:p}", value, key, store);
        store.borrow_mut().insert(key, value);
    }

}