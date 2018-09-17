use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use super::object::ObjectKind;

#[derive(Clone)]
pub struct Environment {
    pub store: HashMap<String, Arc<Mutex<ObjectKind>>>
}

impl Environment {
    pub fn new() -> Arc<Mutex<Environment>> {
        return Arc::new(Mutex::new(Environment{store: HashMap::new()}));
    }

    pub fn get(&mut self, key: String) -> Arc<Mutex<ObjectKind>> {
        match self.store.get(&key) {
            Some(v) => {
                v.clone()
            },
            _ => {
                println!("Key `{}` not in env store! Here is what it contains:", &key);
                for (key, value) in &self.store {
                    let value = value.lock().unwrap().clone();
                    println!("  - {}: \"{}\"", key, value);
                }
                Arc::new(Mutex::new(ObjectKind::Error{message: String::from("Error finding key in environment")}))
            }
        }
    }
    pub fn remove(&mut self, key: String) {
        self.store.remove(&key);
    }
    pub fn insert(&mut self, key: String, value: Arc<Mutex<ObjectKind>>) {
        self.store.insert(key, value);
    }

    pub fn lock_insert(env: &Arc<Mutex<Environment>>, key: String, value: ObjectKind) {
        env.lock().unwrap().insert(key, Arc::new(Mutex::new(value)));
    }
}