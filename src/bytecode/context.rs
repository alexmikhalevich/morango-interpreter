use crate::config::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    data: HashMap<String, Value>,
    args: Vec<String>,
    pub line_number: usize,
}

impl Context {
    pub fn new() -> Context {
        Context {
            data: HashMap::new(),
            args: Vec::new(),
            line_number: 0,
        }
    }

    pub fn has_var(&self, name: &str) -> bool {
        self.data.contains_key(name)
    }

    pub fn add_var(&mut self, name: &str) -> Value {
        let address = self.data.len() as Value;
        self.data.insert(name.to_string(), address);
        address
    }

    pub fn get_var(&self, name: &str) -> Option<Value> {
        self.data.get(name).map(|v| *v)
    }

    pub fn set_args(&mut self, args: Vec<String>) {
        self.args = args;
    }

    pub fn get_arg(&self, index: usize) -> Option<&String> {
        self.args.get(index)
    }

    pub fn args_len(&self) -> usize {
        self.args.len()
    }

    pub fn data_size(&self) -> usize {
        self.data.len()
    }
}
