use crate::value::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    pub bindings: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        let mut env = Self {
            bindings: HashMap::new(),
        };
        
        // Initialize with builtins - we'll move this to builtins module later
        crate::builtins::register_all(&mut env);
        env
    }
    
    pub fn define(&mut self, name: &str, value: Value) {
        self.bindings.insert(name.to_string(), value);
    }
    
    pub fn lookup(&self, name: &str) -> Option<&Value> {
        self.bindings.get(name)
    }
}