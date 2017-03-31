
extern crate regex;

use self::regex::Regex;

use token::*;
use error::*;

pub fn lex(data: String) -> Result<Vec<Token>, Error> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut line: u64 = 1;

    let number = Regex::new("[0-9]+").unwrap();

    let mut str_tokens: Vec<&str> = Vec::new();

    for line in data.lines() {
        for token in line.split_whitespace() {
            str_tokens.push(token);
        }
        str_tokens.push("\n");
    }

    for str_tok in str_tokens {
        //println!("{} \"{}\"", i, str_tok.clone());
        tokens.push(Token::new(match str_tok {
            "toss" => TokenType::Toss,
            "catch" => TokenType::Catch,
            "joke" => TokenType::Joke,
            "curse" => TokenType::Curse,
            "true" => TokenType::Bool(true),
            "false" => TokenType::Bool(false),
            "equal" => TokenType::Equal,
            "greater" => TokenType::Greater,
            "lesser" => TokenType::Lesser,
            "and" => TokenType::And,
            "or" => TokenType::Or,
            "not" => TokenType::Not,
            "if" => TokenType::If,
            "while" => TokenType::While,
            "else" => TokenType::Else,
            "end" => TokenType::End,
            "append" => TokenType::Append,
            "drop" => TokenType::Drop,
            "\n" => {
                line += 1;
                TokenType::Newline
            },
            _ => {
                if number.is_match(str_tok) {
                    TokenType::Number(str_tok.parse::<i64>().unwrap())
                } else {
                    return Err(Error::new(ErrorType::SyntaxError, "Unkown command".into(), line));
                }
            }
        }, line));
    }

    tokens.push(Token::new(TokenType::EndOfFile, line));

    Ok(tokens.into_iter().filter(|ref x| (**x).which != TokenType::None).collect())
}

