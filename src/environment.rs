use std::collections::HashMap;

use super::object::ObjectKind;

#[derive(Clone)]
pub struct Environment {
    pub store: HashMap<String, ObjectKind>
}

impl Environment {
    pub fn get(&mut self, key: String) -> ObjectKind {
        match self.store.get(&key) {
            Some(v) => {
                v.clone()
            },
            _ => {
                println!("Key `{}` not in env store! Here is what it contains:", &key);
                for (key, value) in &self.store {
                    println!("  - {}: \"{}\"", key, value);
                }
                ObjectKind::Error{message: String::from("Error finding key in environment")}
            }
        }
    }
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
    pub fn insert(&mut self, key: String, value: ObjectKind) {
        self.store.insert(key, value);
    }
}