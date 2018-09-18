use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use super::object::ObjectKind;

#[derive(Clone, Debug)]
pub struct Environment {
    pub store: Arc<Mutex<HashMap<String, Arc<Mutex<ObjectKind>>>>>
}

impl Environment {
    pub fn new() -> Environment {
        return Environment{store: Arc::new(Mutex::new(HashMap::new()))};
    }

    pub fn get(self, key: String) -> ObjectKind {
        let store = self.store.lock().unwrap().clone();
        let obj = store.get(&key);
        match obj {
            Some(s) => {
                return s.lock().unwrap().clone();
            },
            None => {
                println!("Key `{}` not in env store! Here is what it contains:", &key);
                println!("{:#?}", store.clone());
                return ObjectKind::Null{};
            }
        }
    }

    pub fn insert(self, key: String, value: ObjectKind) {
        let store = self.store.lock().unwrap();
        println!("Inserting {:#?} as {}", value, key);
        store.clone().insert(key, Arc::new(Mutex::new(value)));
    }

    pub fn clone_insert(env: &Environment, key: String, value: ObjectKind) {
        let store = env.store.lock().unwrap();
        println!("Inserting {:#?} as {}", value, key);
        store.clone().insert(key, Arc::new(Mutex::new(value)));
    }

}