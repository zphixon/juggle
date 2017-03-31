
extern crate regex;

use self::regex::Regex;

use token::*;
use error::*;

pub fn lex(data: String) -> Result<Vec<Token>, Error> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut stringing = false;
    let mut to_string: String = "".into();

    let mut line: u64 = 1;

    let number = Regex::new("[0-9]+").unwrap();

    let mut str_tokens: Vec<&str> = Vec::new();

    for line in data.lines() {
        for token in line.split_whitespace() {
            str_tokens.push(token);
        }
        str_tokens.push("\n");
    }

    for (i, str_tok) in str_tokens.iter().enumerate() {
        //println!("{} \"{}\"", i, str_tok.clone());
        tokens.push(Token::new(if !stringing {
            match *str_tok {
                "toss" => TokenType::Toss,
                "catch" => TokenType::Catch,
                "joke" => TokenType::Joke,
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
                    if str_tok.starts_with("\"") && !stringing {
                        if str_tok.ends_with("\"") {
                            if str_tok.len() != 1 {
                                to_string.push_str(&str_tok[1..str_tok.len()-1]);
                                stringing = false;
                                let s = to_string.clone();
                                to_string.clear();
                                TokenType::StringLiteral(s)
                            } else {
                                // aaghgghhh
                                to_string.push_str(" ");
                                stringing = true;
                                TokenType::None
                            }
                        } else {
                            to_string.push_str(&str_tok[1..]);
                            stringing = true;
                            TokenType::None
                        }
                    } else if number.is_match(str_tok) {
                        TokenType::Number(str_tok.parse::<u64>().unwrap())
                    } else {
                        return Err(Error::new(ErrorType::SyntaxError, "Unkown command".into(), line));
                    }
                }
            }
        } else {
            if !str_tok.ends_with("\"") {
                to_string.push_str(str_tok);
                TokenType::None
            } else {
                // whoops, I'm going to need to experiment with rust
                // parsers some  more...
                to_string.push_str(" ");
                to_string.push_str(&str_tok[..str_tok.len() - 1]);
                stringing = false;
                let s = to_string.clone();
                to_string.clear();
                TokenType::StringLiteral(s)
            }
        }, line));
    }

    if !stringing {
        tokens.push(Token::new(TokenType::EndOfFile, line));
    } else {
        return Err(Error::new(ErrorType::SyntaxError, "Unterminated string".into(), line));
    }

    Ok(tokens.into_iter().filter(|ref x| (**x).which != TokenType::None).collect())
}

pub fn parse() {}

