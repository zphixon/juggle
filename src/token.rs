
use value::*;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Toss,
    Joke,
    Catch,
    Curse,
    Plus,
    Minus,
    Times,
    Divided,
    Modulo,
    Equal,
    Greater,
    Lesser,
    And,
    Or,
    Not,
    If,
    While,
    Else,
    End,
    Append,
    Nth,
    Feedback,
    Rethrow,
    Recatch,
    Drop,
    Turn,
    Routine,
    Value(Value),
    EndOfFile,
    None,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub which: TokenType,
    pub line: u64,
}

impl Token {
    pub fn new(which: TokenType, line: u64) -> Token {
        Token {
            which: which,
            line: line,
        }
    }

    pub fn is_value(&self) -> bool {
        match self.which {
            TokenType::Value(_) => true,
            _ => false
        }
    }

    pub fn to_value(&self) -> Value {
        match self.which {
            TokenType::Value(ref v) => v.clone(),
            _ => panic!("Called to_value on non-value")
        }
    }
}

