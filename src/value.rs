
#[derive(PartialOrd, PartialEq, Debug, Clone)]
pub enum Value {
    Bool(bool),
    Number(i64),
    Array(Vec<Value>),
}

impl Value {
    pub fn same_type(a: &Value, b: &Value) -> bool {
        (a.is_number() && b.is_number()) || (a.is_array() && b.is_array()) || (a.is_bool() && b.is_bool())
    }

    pub fn print_as_char(&self) {
        if self.is_number() {
            print!("{}", self.get_number() as u8 as char);
        } else if self.is_array() {
            for c in self.get_array() {
                c.print_as_char();
            }
        } else {
            print!("{}", self.get_bool());
        }
    }

    pub fn print_as_number(&self) {
        if self.is_number() {
            print!("{}", self.get_number());
        } else if self.is_array() {
            print!("{:?}", self.get_array());
        } else {
            print!("{}", self.get_bool());
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            &Value::Number(_) => true,
            _ => false
        }
    }

    pub fn is_array(&self) -> bool {
        match self {
            &Value::Array(_) => true,
            _ => false
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            &Value::Bool(_) => true,
            _ => false
        }
    }

    pub fn get_number(&self) -> i64 {
        let res = match self {
            &Value::Number(n) => n,
            _ => panic!("called get_number on non-number")
        };
        res
    }

    pub fn get_array(&self) -> Vec<Value> {
        let res = match self {
            &Value::Array(ref a) => a,
            _ => panic!("called get_array on non-array")
        };
        res.to_vec()
    }

    pub fn get_bool(&self) -> bool {
        let res = match self {
            &Value::Bool(b) => b,
            _ => panic!("called get_bool on non-bool")
        };
        res
    }
}

