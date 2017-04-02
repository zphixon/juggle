
use value::*;

#[derive(Debug)]
pub struct Air {
    stack: Vec<Value>,
}

impl Air {
    pub fn new() -> Air {
        Air {
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self, val: Value) {
        self.stack.insert(0, val);
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.stack.pop()
    }

    pub fn pop_last(&mut self) -> Option<Value> {
        // this is stupid
        if self.stack.is_empty() {
            None
        } else {
            Some(self.stack.remove(0))
        }
    }

    pub fn clone(&mut self) -> Vec<Value> {
        self.stack.clone()
    }
}

