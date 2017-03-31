
use std::fs::File;
use std::io::Read;
use std::path::Path;

extern crate juggle;

use juggle::*;

fn main() {
    let mut file = File::open(Path::new("test.txt")).unwrap();
    let mut s = String::new();

    file.read_to_string(&mut s).unwrap();

    let l: Result<Vec<Token>, Error> = lex(s);

    if l.is_ok() {
        let e: Result<(), Error> = eval(l.unwrap());
        if e.is_err() {
            println!("{}", e.err().unwrap());
        }
    } else {
        println!("{}", l.err().unwrap());
    }
}

fn eval(prog: Vec<Token>) -> Result<(), Error> {
    let mut air = Air::new();
    let mut hands = Hands::new();

    let mut frames = vec![true];
    let mut current_frame = 0;

    let mut whiles = vec![0];
    let mut current_while = whiles.len() - 1;
    let mut jump = false;

    let mut k = 0;

    while k < prog.len()  {
        // mkay...
        let ref tok = prog[k];

        match tok.which {
            TokenType::Toss => {
                if frames[current_frame] {
                    if k + 1 != prog.len() && prog[k + 1].is_value() {
                        air.push(prog[k + 1].to_value());
                    } else {
                        let tmp = hands.pop();
                        if tmp.is_some() {
                            air.push(tmp.unwrap());
                        } else {
                            return Err(Error::new(ErrorType::HandsUnderflowError, "Hands underflowed when tossing to air".into(), prog[k].line));
                        }
                    }
                }
            },
            TokenType::Catch => {
                if frames[current_frame] {
                    let tmp = air.pop();
                    if tmp.is_some() {
                        hands.push(tmp.unwrap());
                    } else {
                        return Err(Error::new(ErrorType::AirUnderflowError, "Air underflowed when catching to hands".into(), prog[k].line));
                    }
                }
            },
            TokenType::Joke => {
                if frames[current_frame] {
                    let opt = hands.pop();
                    if opt.is_some() {
                        opt.unwrap().print_as_number();
                    } else {
                        return Err(Error::new(ErrorType::HandsUnderflowError, "Hands underflowed when printing value".into(), prog[k].line));
                    }
                }
            },
            TokenType::Curse => {
                if frames[current_frame] {
                    let opt = hands.pop();
                    if opt.is_some() {
                        opt.unwrap().print_as_char();
                    } else {
                        return Err(Error::new(ErrorType::HandsUnderflowError, "Hands underflowed when printing value".into(), prog[k].line));
                    }
                }
            }
            TokenType::Equal => {
                if frames[current_frame] {
                    let lho = air.pop();
                    let rho = air.pop();
                    if lho.is_some() && rho.is_some() {
                        let lhs = lho.unwrap();
                        let rhs = rho.unwrap();
                        if Value::same_type(&lhs, &rhs) {
                            air.push(Value::Bool(lhs == rhs));
                        } else {
                            return Err(Error::new(ErrorType::TypeError, "Attempted to compare different types (equal)".into(), prog[k].line));
                        }
                    } else {
                        return Err(Error::new(ErrorType::AirUnderflowError, "Air underflowed when comparing values (equal)".into(), prog[k].line));
                    }
                }
            },
            TokenType::Greater => {
                if frames[current_frame] {
                    let lho = air.pop();
                    let rho = air.pop();
                    if lho.is_some() && rho.is_some() {
                        let lhs = lho.unwrap();
                        let rhs = rho.unwrap();
                        if lhs.is_number() && rhs.is_number() {
                            air.push(Value::Bool(lhs.get_number() > rhs.get_number()));
                        } else {
                            return Err(Error::new(ErrorType::TypeError, "Attempted to compare non-number types (greater)".into(), prog[k].line));
                        }
                    } else {
                        return Err(Error::new(ErrorType::AirUnderflowError, "Air underflowed when comparing values (greater)".into(), prog[k].line));
                    }
                }
            },
            TokenType::Lesser => {
                if frames[current_frame] {
                    let lho = air.pop();
                    let rho = air.pop();
                    if lho.is_some() && rho.is_some() {
                        let lhs = lho.unwrap();
                        let rhs = rho.unwrap();
                        if lhs.is_number() && rhs.is_number() {
                            air.push(Value::Bool(lhs.get_number() < rhs.get_number()));
                        } else {
                            return Err(Error::new(ErrorType::TypeError, "Attempted to compare non-number types (lesser)".into(), prog[k].line));
                        }
                    } else {
                        return Err(Error::new(ErrorType::AirUnderflowError, "Air underflowed when comparing values (lesser)".into(), prog[k].line));
                    }
                }
            },
            TokenType::And => {
                if frames[current_frame] {
                    let lho = air.pop();
                    let rho = air.pop();
                    if lho.is_some() && rho.is_some() {
                        let lhs = lho.unwrap();
                        let rhs = rho.unwrap();
                        if lhs.is_bool() && rhs.is_bool() {
                            air.push(Value::Bool(lhs.get_bool() && rhs.get_bool()));
                        } else {
                            return Err(Error::new(ErrorType::TypeError, "Attempted to compare non-bools (and)".into(), prog[k].line));
                        }
                    } else {
                        return Err(Error::new(ErrorType::AirUnderflowError, "Air underflowed when comparing values (and)".into(), prog[k].line));
                    }
                }
            },
            TokenType::Or => {
                if frames[current_frame] {
                    let lho = air.pop();
                    let rho = air.pop();
                    if lho.is_some() && rho.is_some() {
                        let lhs = lho.unwrap();
                        let rhs = rho.unwrap();
                        if lhs.is_bool() && rhs.is_bool() {
                            air.push(Value::Bool(lhs.get_bool() || rhs.get_bool()));
                        } else {
                            return Err(Error::new(ErrorType::TypeError, "Attempted to compare non-bools (and)".into(), prog[k].line));
                        }
                    } else {
                        return Err(Error::new(ErrorType::AirUnderflowError, "Air underflowed when comparing values (and)".into(), prog[k].line));
                    }
                }
            },
            TokenType::Not => {
                if frames[current_frame] {
                }
            },
            TokenType::If => {
                if frames[current_frame] {
                }
            },
            TokenType::While => {
                if frames[current_frame] {
                }
            },
            TokenType::Else => {
            },
            TokenType::End => {
            },
            TokenType::Append => {
                if frames[current_frame] {
                }
            },
            TokenType::Drop => {
                if frames[current_frame] {
                }
            },
            _ => {
            }
        }

        if jump {
            k = whiles[current_while];
            jump = false;
        } else {
            k += 1;
        }
    }

    println!("air: {:?}\nhands: {:?}", air, hands);

    Ok(())
}

