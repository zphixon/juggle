
extern crate regex;

use self::regex::Regex;

use token::*;
use error::*;

// data split on whitespace
pub fn parse(data: Vec<&str>) -> Result<Vec<Token>, Error> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut stringing = false;
    let mut to_string: String = "".into();

    let number = Regex::new("[0-9]+").unwrap();

    for str_tok in data {
        tokens.push(if !stringing {
            match str_tok {
                "toss" => Token::Toss,
                "throw" => Token::Throw,
                "catch" => Token::Catch,
                "joke" => Token::Joke,
                "true" => Token::Bool(true),
                "false" => Token::Bool(false),
                "equal" => Token::Equal,
                "greater" => Token::Greater,
                "lesser" => Token::Lesser,
                "and" => Token::And,
                "or" => Token::Or,
                "not" => Token::Not,
                "if" => Token::If,
                "while" => Token::While,
                "else" => Token::Else,
                "end" => Token::End,
                "append" => Token::Append,
                "drop" => Token::Drop,
                _ => {
                    if str_tok.starts_with("\"") && !stringing {
                        to_string.push_str(&str_tok[1..]);
                        stringing = true;
                        Token::None
                    } else if number.is_match(str_tok) {
                        Token::Number(str_tok.parse::<u64>().unwrap())
                    } else {
                        return Err(Error::Syntax);
                    }
                }
            }
        } else {
            if !str_tok.ends_with("\"") {
                to_string.push_str(str_tok);
                Token::None
            } else {
                // whoops, I'm going to need to experiment with rust
                // parsers some  more...
                to_string.push_str(" ");
                to_string.push_str(&str_tok[..str_tok.len() - 1]);
                Token::StringLiteral(to_string.clone())
            }
        });
    }

    tokens.push(Token::EndOfFile);

    Ok(tokens.into_iter().filter(|ref x| **x != Token::None).collect())
}

