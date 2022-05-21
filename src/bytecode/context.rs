use crate::config::Value;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Context {
    data: HashMap<String, Value>,
    labels: HashMap<String, Value>,
    args: Vec<String>,
    pub line_number: usize,
    pub instruction_number: Value,
}

impl Context {
    pub fn new() -> Context {
        Context {
            data: HashMap::new(),
            labels: HashMap::new(),
            args: Vec::new(),
            line_number: 0,
            instruction_number: 0,
        }
    }

    pub fn is_label(label: &str) -> bool {
        lazy_static! {
            static ref LABEL_RE: Regex =
                Regex::new(r"^&[a-zA-Z_0-9][a-zA-Z0-9_]*").expect("Invalid regex");
        }
        LABEL_RE.is_match(label)
    }

    pub fn has_var(&self, name: &str) -> bool {
        self.data.contains_key(name)
    }

    pub fn add_var(&mut self, name: &str) -> Value {
        let address = self.data.len() as Value;
        self.data.insert(name.to_string(), address);
        address
    }

    pub fn has_label(&self, name: &str) -> bool {
        self.labels.contains_key(name)
    }

    pub fn add_label(&mut self, name: &str, address: Value) {
        self.labels.insert(name.to_string(), address);
    }

    pub fn get_label(&self, name: &str) -> Value {
        self.labels.get(name).unwrap().clone()
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
