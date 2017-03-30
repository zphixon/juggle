
#[derive(PartialOrd, PartialEq)]
pub enum Value {
    Bool(bool),
    Number(i64),
    Array(Vec<Value>),
}

