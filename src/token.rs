
use error::*;

#[derive(Debug, PartialEq)]
pub enum Token {
    Toss,
    Throw,
    Catch,
    Joke,
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
    Drop,
    StringLiteral(String),
    Number(u64),
    Bool(bool),
    Error(Error),
    EndOfFile,
    None, // filter out later
}

