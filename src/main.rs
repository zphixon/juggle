
extern crate argparse;
extern crate rustyline;

use argparse::{ArgumentParser, StoreTrue, Store, Print};
use rustyline::Editor;

use std::fs::File;
use std::io::Read;
use std::path::Path;

extern crate juggle;

use juggle::*;

static mut DEBUG: bool = false;
static mut SHOW_STACK: bool = false;

static VERSION: &'static str = "0.1.0";

fn main() {
    let mut debug = false;
    let mut show_stack = false;
    let mut filename: String = "".into();

    {
        let mut args = ArgumentParser::new();
        args.set_description("Run a juggle language file");
        args.refer(&mut debug)
            .add_option(&["-d", "--debug"], StoreTrue, "Show debugging");
        args.refer(&mut show_stack)
            .add_option(&["-s", "--stack"], StoreTrue, "Show stacks");
        args.refer(&mut filename)
            .add_argument("file", Store, "File to run")
            .required();
        args.add_option(&["-V", "--version"],
                        Print("juggle version ".to_string() + VERSION), "Show version");
        args.parse_args_or_exit();
    }

    unsafe {
        DEBUG = debug;
        SHOW_STACK = show_stack;
    }

    let mut file  = match File::open(Path::new(&filename)) {
        Ok(f) => f,
        Err(e) => {
            println!("File failed to open: {}", e);
            std::process::exit(1);
        }
    };

    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let l: Result<Vec<Token>, Error> = parse(s);

    if l.is_ok() {
        let mut air = Air::new();
        let mut hands = Hands::new();

        let e: Result<(), Error> = eval(l.unwrap(), &mut air, &mut hands);

        if e.is_err() {
            println!("{}", e.err().unwrap());
        }
    } else {
        println!("{}", l.err().unwrap());
    }
}

fn eval(prog: Vec<Token>, air: &mut Air, hands: &mut Hands) -> Result<(), Error> {
    let mut rl = Editor::<()>::new();

    let mut frames = vec![true];
    let mut current_frame = 0;

    let mut whiles = vec![0];
    let mut current_while = whiles.len() - 1;
    let mut jump = false;

    let mut k = 0;

    while k < prog.len()  {
        // mkay...
        let ref tok = prog[k];

        unsafe {
            if DEBUG || SHOW_STACK {
                println!("");
            }

            if DEBUG {
                println!("{}: {:?}, line {}, {} {} {} {}", k, tok.which, tok.line,
                         frames[current_frame], frames.len(), whiles[current_while], whiles.len());
            }

            if SHOW_STACK {
                println!("{}: air", k);
                for v in air.clone() {
                    println!("   {:?}", v);
                }
                println!("{}: hands", k);
                for v in hands.clone() {
                    println!("   {:?}", v);
                }
            }
        }

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
                            return Err(Error::new(ErrorType::HandsUnderflowError, "Hands underflowed when tossing to air".into(), tok.line));
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
                        return Err(Error::new(ErrorType::AirUnderflowError, "Air underflowed when catching to hands".into(), tok.line));
                    }
                }
            },

            TokenType::Curse => {
                if frames[current_frame] {
                    let opt = hands.pop();
                    if opt.is_some() {
                        let tmp = opt.unwrap();
                        if !tmp.is_null() {
                            tmp.print_as_number();
                        }
                    } else {
                        return Err(Error::new(ErrorType::HandsUnderflowError, "Hands underflowed when printing value".into(), tok.line));
                    }
                }
            },

            TokenType::Joke => {
                if frames[current_frame] {
                    let opt = hands.pop();
                    if opt.is_some() {
                        let tmp = opt.unwrap();
                        if !tmp.is_null() {
                            tmp.print_as_char();
                        }
                    } else {
                        return Err(Error::new(ErrorType::HandsUnderflowError, "Hands underflowed when printing value (curse)".into(), tok.line));
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
                            return Err(Error::new(ErrorType::TypeError, "Attempted to add non-number values (add)".into(), tok.line));
                        }
                    } else {
                        return Err(Error::new(ErrorType::HandsUnderflowError, "Hands underflowed when adding values (add)".into(), tok.line));
                    }
                }
            },

            TokenType::Minus => {
                if frames[current_frame] {
                    let lho = hands.pop();
                    let rho = hands.pop();
                    if lho.is_some() {
                        let lhs = lho.unwrap();
                        let rhs = rho.unwrap();
                        if lhs.is_number() && rhs.is_number() {
                            hands.push(Value::Number(lhs.get_number() - rhs.get_number()));
                        } else {
                            return Err(Error::new(ErrorType::TypeError, "Attempted to subtract non-number values (minus)".into(), tok.line));
                        }
                    } else {
                        return Err(Error::new(ErrorType::HandsUnderflowError, "Hands underflowed when subtracting values (minus)".into(), tok.line));
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
                            return Err(Error::new(ErrorType::TypeError, "Attempted to multiply non-number values (times)".into(), tok.line));
                        }
                    } else {
                        return Err(Error::new(ErrorType::HandsUnderflowError, "Hands underflowed when multiplying values (times)".into(), tok.line));
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
                            return Err(Error::new(ErrorType::TypeError, "Attempted to divide non-number values (divided)".into(), tok.line));
                        }
                    } else {
                        return Err(Error::new(ErrorType::HandsUnderflowError, "Hands underflowed when dividing values (divided)".into(), tok.line));
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
                            return Err(Error::new(ErrorType::TypeError, "Attempted to modulus non-number values (modulo)".into(), tok.line));
                        }
                    } else {
                        return Err(Error::new(ErrorType::HandsUnderflowError, "Hands underflowed when remainding values (modulo)".into(), tok.line));
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
                            return Err(Error::new(ErrorType::TypeError, "Attempted to compare different types (equal)".into(), tok.line));
                        }
                    } else {
                        return Err(Error::new(ErrorType::AirUnderflowError, "Air underflowed when comparing values (equal)".into(), tok.line));
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
                            return Err(Error::new(ErrorType::TypeError, "Attempted to compare non-number types (greater)".into(), tok.line));
                        }
                    } else {
                        return Err(Error::new(ErrorType::AirUnderflowError, "Air underflowed when comparing values (greater)".into(), tok.line));
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
                            return Err(Error::new(ErrorType::TypeError, "Attempted to compare non-number types (lesser)".into(), tok.line));
                        }
                    } else {
                        return Err(Error::new(ErrorType::AirUnderflowError, "Air underflowed when comparing values (lesser)".into(), tok.line));
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
                            return Err(Error::new(ErrorType::TypeError, "Attempted to compare non-bools (and)".into(), tok.line));
                        }
                    } else {
                        return Err(Error::new(ErrorType::AirUnderflowError, "Air underflowed when comparing values (and)".into(), tok.line));
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
                            return Err(Error::new(ErrorType::TypeError, "Attempted to compare non-bools (and)".into(), tok.line));
                        }
                    } else {
                        return Err(Error::new(ErrorType::AirUnderflowError, "Air underflowed when comparing values (and)".into(), tok.line));
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
                            return Err(Error::new(ErrorType::TypeError, "Attempted to negate non-bool (not)".into(), tok.line));
                        }
                    } else {
                        return Err(Error::new(ErrorType::HandsUnderflowError, "Hands underflowed when negating a value (not)".into(), tok.line));
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
                            return Err(Error::new(ErrorType::TypeError, "Condition requires bool (if)".into(), tok.line));
                        }
                    } else {
                        return Err(Error::new(ErrorType::AirUnderflowError, "Air underflowed when checking condition (if)".into(), tok.line));
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
                            return Err(Error::new(ErrorType::TypeError, "Condition requires bool (while)".into(), tok.line));
                        }
                    } else {
                        return Err(Error::new(ErrorType::AirUnderflowError, "Air underflowed when checking condition (while)".into(), tok.line));
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
                    return Err(Error::new(ErrorType::SyntaxError, "Mismatching if/while/end (end)".into(), tok.line));
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
                        return Err(Error::new(ErrorType::HandsUnderflowError, "Hands underflowed when appending to array (append)".into(), tok.line));
                    }
                }
            },

            TokenType::Nth => {
                if frames[current_frame] {
                    let from = air.pop();
                    if from.is_some() {
                        let arr = from.unwrap();
                        if arr.is_array() {
                            let nth = hands.pop();
                            if nth.is_some() {
                                let n = nth.unwrap();
                                if n.is_number() {
                                    air.push(arr.get_array()[n.get_number() as usize].clone());
                                } else {
                                    return Err(Error::new(ErrorType::TypeError, "Attempted to index array with non-number (nth)".into(), tok.line));
                                }
                            } else {
                                return Err(Error::new(ErrorType::HandsUnderflowError, "Hands underflowed when getting nth element (nth)".into(), tok.line));
                            }
                        } else {
                            return Err(Error::new(ErrorType::TypeError, "Attempted to index non-array (nth)".into(), tok.line));
                        }
                    } else {
                        return Err(Error::new(ErrorType::AirUnderflowError, "Air underflowed when getting nth element (nth)".into(), tok.line));
                    }
                }
            }

            TokenType::Feedback => {
                if frames[current_frame] {
                    let read = rl.readline("> ");
                    let input: String = match read {
                        Ok(line) => line,
                        Err(rustyline::error::ReadlineError::Eof) => "".into(),
                        Err(_) => {
                            return Err(Error::new(ErrorType::IOError, "Error on input (feedback)".into(), tok.line));
                        }
                    };
                    let t = Value::Array(input.bytes()
                                         .map(|x| Value::Number(x as i64))
                                         .collect::<Vec<Value>>());
                    air.push(t);
                }
            }

            TokenType::Rethrow => {
                if frames[current_frame] {
                    let tmp = air.pop_last();
                    if tmp.is_some() {
                        let t = tmp.unwrap();
                        air.push(t.clone());
                        air.push(t);
                    } else {
                        return Err(Error::new(ErrorType::AirUnderflowError, "Air underflowed when duplicating (rethrow)".into(), tok.line));
                    }
                }
            },

            TokenType::Recatch => {
                if frames[current_frame] {
                    let tmp = hands.pop();
                    if tmp.is_some() {
                        let t = tmp.unwrap();
                        hands.push(t.clone());
                        hands.push(t);
                    } else {
                        return Err(Error::new(ErrorType::HandsUnderflowError, "Hands underflowed when duplicating (recatch)".into(), tok.line));
                    }
                }
            },

            TokenType::Drop => {
                if frames[current_frame] {
                    if hands.pop().is_none() {
                        return Err(Error::new(ErrorType::HandsUnderflowError, "Hands underflowed when dropping value (drop)".into(), tok.line));
                    }
                }
            },

            TokenType::Turn => {
                if frames[current_frame] {
                    air.reverse();
                }
            },

            TokenType::Routine => {
                if frames[current_frame] {
                    let nameo = air.pop();
                    if nameo.is_some() {
                        let name = nameo.unwrap();
                        frames.push(false);
                        current_frame += 1;
                    } else {
                        return Err(Error::new(ErrorType::AirUnderflowError, "Air underflowed when creating routine (routine)".into(), tok.line));
                    }
                }
            },

            TokenType::EndOfFile => {
                if current_frame > 0 {
                    return Err(Error::new(ErrorType::SyntaxError, "Mismatching if/while/end (eof)".into(), tok.line));
                }
            },

            TokenType::Value(_) => {
                if frames[current_frame] {
                    unreachable!();
                }
            },

            _ => {
                unreachable!();
            }
        }

        if jump {
            k = whiles[current_while];
            jump = false;
        } else {
            k += 1;
        }
    }

    Ok(())
}

