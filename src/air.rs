
use value;

pub struct Air {
    stack: Vec<value::Value>,
}

impl Air {
    pub fn push(&mut self, val: value::Value) {
        self.stack.insert(0, val);
    }

    pub fn pop(&mut self) -> Option<value::Value> {
        self.stack.pop()
    }
}

