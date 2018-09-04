use std::collections::HashMap;

use super::object::ObjectKind;

#[derive(Clone)]
pub struct Environment {
    store: HashMap<String, ObjectKind>
}

impl Environment {
    fn get(&mut self, key: String) -> ObjectKind {
        match self.store.get(&key) {
            Some(v) => {
                v.clone()
            },
            _ => {
                ObjectKind::Error{message: String::from("Error finding key")}
            }
        }
    }
    fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
    fn insert(&mut self, key: String, value: ObjectKind) {
        self.store.insert(key, value);
    }
}