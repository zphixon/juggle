
use error::*;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Toss,
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
    Newline,
    StringLiteral(String),
    Number(u64),
    Bool(bool),
    EndOfFile,
    None, // filter out later
}

#[derive(Debug)]
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
}

