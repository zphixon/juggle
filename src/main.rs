
extern crate argparse;
extern crate rustyline;

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
                        k += 1;
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
                        let tmp = opt.unwrap();
                        if !tmp.is_null() {
                            tmp.print_as_number();
                        }
                    } else {
                        return Err(Error::new(ErrorType::HandsUnderflowError, "Hands underflowed when printing value".into(), prog[k].line));
                    }
                }
            },

            TokenType::Curse => {
                if frames[current_frame] {
                    let opt = hands.pop();
                    if opt.is_some() {
                        let tmp = opt.unwrap();
                        if !tmp.is_null() {
                            tmp.print_as_char();
                        }
                    } else {
                        return Err(Error::new(ErrorType::HandsUnderflowError, "Hands underflowed when printing value (curse)".into(), prog[k].line));
                    }
                }
            },

            TokenType::Plus => {
                if frames[current_frame] {
                    let lho = hands.pop();
                    let rho = hands.pop();
                    if lho.is_some() && rho.is_some() {
                        let lhs = lho.unwrap();
                        let rhs = rho.unwrap();
                        if lhs.is_number() && rhs.is_number() {
                            hands.push(Value::Number(lhs.get_number() + rhs.get_number()));
                        } else {
                            return Err(Error::new(ErrorType::TypeError, "Attempted to add non-number values (add)".into(), prog[k].line));
                        }
                    } {
                        return Err(Error::new(ErrorType::HandsUnderflowError, "Hands underflowed when adding values (add)".into(), prog[k].line));
                    }
                }
            },

            TokenType::Minus => {
                if frames[current_frame] {
                    let lho = hands.pop().unwrap();
                    let rho = hands.pop().unwrap();
                    //if lho.is_some() {
                    if true {
                        ////let lhs = lho.unwrap();
                        ////let rhs = rho.unwrap();
                        //if lho.is_number() && rho.is_number() {
                        //    hands.push(Value::Number(lho.get_number() - rho.get_number()));
                        //} else {
                        //    return Err(Error::new(ErrorType::TypeError, "Attempted to subtract non-number values (minus)".into(), prog[k].line));
                        //}
                    } {
                        unreachable!();
                        return Err(Error::new(ErrorType::HandsUnderflowError, "Hands underflowed when subtracting values (minus)".into(), prog[k].line));
                    }
                }
            },

            TokenType::Times => {
                if frames[current_frame] {
                    let lho = hands.pop();
                    let rho = hands.pop();
                    if lho.is_some() && rho.is_some() {
                        let lhs = lho.unwrap();
                        let rhs = rho.unwrap();
                        if lhs.is_number() && rhs.is_number() {
                            hands.push(Value::Number(lhs.get_number() * rhs.get_number()));
                        } else {
                            return Err(Error::new(ErrorType::TypeError, "Attempted to multiply non-number values (times)".into(), prog[k].line));
                        }
                    } {
                        return Err(Error::new(ErrorType::HandsUnderflowError, "Hands underflowed when multiplying values (times)".into(), prog[k].line));
                    }
                }
            },

            TokenType::Divided => {
                if frames[current_frame] {
                    let lho = hands.pop();
                    let rho = hands.pop();
                    if lho.is_some() && rho.is_some() {
                        let lhs = lho.unwrap();
                        let rhs = rho.unwrap();
                        if lhs.is_number() && rhs.is_number() {
                            hands.push(Value::Number(lhs.get_number() / rhs.get_number()));
                        } else {
                            return Err(Error::new(ErrorType::TypeError, "Attempted to divide non-number values (divided)".into(), prog[k].line));
                        }
                    } {
                        return Err(Error::new(ErrorType::HandsUnderflowError, "Hands underflowed when dividing values (divided)".into(), prog[k].line));
                    }
                }
            },

            TokenType::Modulo => {
                if frames[current_frame] {
                    let lho = hands.pop();
                    let rho = hands.pop();
                    if lho.is_some() && rho.is_some() {
                        let lhs = lho.unwrap();
                        let rhs = rho.unwrap();
                        if lhs.is_number() && rhs.is_number() {
                            hands.push(Value::Number(lhs.get_number() % rhs.get_number()));
                        } else {
                            return Err(Error::new(ErrorType::TypeError, "Attempted to modulus non-number values (modulo)".into(), prog[k].line));
                        }
                    } {
                        return Err(Error::new(ErrorType::HandsUnderflowError, "Hands underflowed when remainding values (modulo)".into(), prog[k].line));
                    }
                }
            },

            TokenType::Equal => {
                if frames[current_frame] {
                    let lho = air.pop();
                    let rho = air.pop();
                    if lho.is_some() && rho.is_some() {
                        let lhs = lho.unwrap();
                        let rhs = rho.unwrap();
                        if Value::same_type(&lhs, &rhs) {
                            hands.push(Value::Bool(lhs == rhs));
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
                            hands.push(Value::Bool(lhs.get_number() > rhs.get_number()));
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
                            hands.push(Value::Bool(lhs.get_number() < rhs.get_number()));
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
                            hands.push(Value::Bool(lhs.get_bool() && rhs.get_bool()));
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
                            hands.push(Value::Bool(lhs.get_bool() || rhs.get_bool()));
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
                    let tmpo = hands.pop();
                    if tmpo.is_some() {
                        let tmp = tmpo.unwrap();
                        if tmp.is_bool() {
                            hands.push(Value::Bool(!tmp.get_bool()));
                        } else {
                            return Err(Error::new(ErrorType::TypeError, "Attempted to negate non-bool (not)".into(), prog[k].line));
                        }
                    } else {
                        return Err(Error::new(ErrorType::HandsUnderflowError, "Hands underflowed when negating a value (not)".into(), prog[k].line));
                    }
                }
            },

            TokenType::If => {
                if frames[current_frame] {
                    let tmpo = air.pop();
                    if tmpo.is_some() {
                        let tmp = tmpo.unwrap();
                        if tmp.is_bool() {
                            frames.push(tmp.get_bool());
                            current_frame += 1;
                        } else {
                            return Err(Error::new(ErrorType::TypeError, "Condition requires bool (if)".into(), prog[k].line));
                        }
                    } else {
                        return Err(Error::new(ErrorType::AirUnderflowError, "Air underflowed when checking condition (if)".into(), prog[k].line));
                    }
                }
            },

            TokenType::While => {
                if frames[current_frame] {
                    let tmpo = air.pop();
                    if tmpo.is_some() {
                        let tmp = tmpo.unwrap();
                        if tmp.is_bool() {
                            if tmp.get_bool() {
                                frames.push(true);
                                whiles.push(k);
                            } else {
                                frames.push(false);
                                whiles.push(0);
                            }
                            current_frame += 1;
                            current_while += 1;
                        } else {
                            return Err(Error::new(ErrorType::TypeError, "Condition requires bool (while)".into(), prog[k].line));
                        }
                    } else {
                        return Err(Error::new(ErrorType::AirUnderflowError, "Air underflowed when checking condition (while)".into(), prog[k].line));
                    }
                }
            },

            TokenType::Else => {
                frames[current_frame] = !frames[current_frame];
            },

            TokenType::End => {
                if current_frame != 0 {
                    if whiles[current_while] == 0 {
                        frames.pop();
                        current_frame -= 1;
                    }
                } else {
                    return Err(Error::new(ErrorType::MismatchingEndError, "Mismatching if/while/end (end)".into(), prog[k].line));
                }
                if whiles[current_while] != 0 {
                    jump = true;
                } else if current_while != 0 {
                    whiles.pop();
                    current_while -= 1;
                }
            },

            TokenType::Append => {
                if frames[current_frame] {
                    let dsto = hands.pop();
                    let srco = hands.pop();
                    if srco.is_some() && dsto.is_some() {
                        let src = srco.unwrap();
                        let dst = dsto.unwrap();
                        if dst.is_array() {
                            let mut tmp = dst.get_array();
                            tmp.push(src);
                            hands.push(Value::Array(tmp));
                        } else {
                            hands.push(Value::Array(vec![dst, src]));
                        }
                    } else {
                        return Err(Error::new(ErrorType::HandsUnderflowError, "Hands underflowed when appending to array (append)".into(), prog[k].line));
                    }
                }
            },

            TokenType::Drop => {
                if frames[current_frame] {
                    if hands.pop().is_none() {
                        return Err(Error::new(ErrorType::HandsUnderflowError, "Hands underflowed when dropping value (drop)".into(), prog[k].line));
                    }
                }
            },

            TokenType::EndOfFile => {
                if current_frame > 0 {
                    return Err(Error::new(ErrorType::MismatchingEndError, "Mismatching if/while/end (eof)".into(), prog[k].line));
                }
            },

            _ => {
                unreachable!();
            },
        }

        if jump {
            k = whiles[current_while];
            jump = false;
        } else {
            k += 1;
        }

        //println!("hands: {:?}\nair: {:?}", hands, air);
    }

    Ok(())
}

