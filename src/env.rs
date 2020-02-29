use std::{collections::HashMap, rc::Rc, cell::RefCell};
use crate::object::Object;


#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    store: HashMap<String, Object>,
    host: Option<Rc<RefCell<Environment>>>,
}
impl Environment {
    pub fn new() -> Self {
        Environment { store: HashMap::new(), host: None }
    }

    pub fn virtual_environment(host: Rc<RefCell<Environment>>) -> Environment {
        Environment { store: HashMap::new(), host: Some(host) }
    }

    pub fn get(&self, key: &str) -> Option<Object> {
        match self.store.get(key) {
            Some(obj) => Some(obj.clone()),
            None => match &self.host {
                Some(env) => env.borrow().get(key),
                None      => None,
            },
        }
    }

    pub fn set(&mut self, key: String, value: Object) {
        self.store.insert(key, value);
    }
}
