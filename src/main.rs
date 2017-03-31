
use std::fs::File;
use std::io::Read;
use std::path::Path;

extern crate juggle;

use juggle::*;

fn main() {
    // TODO: handle errors
    let mut file = File::open(Path::new("test.txt")).unwrap();
    let mut s = String::new();

    file.read_to_string(&mut s).unwrap();

    let l: Result<Vec<Token>, Error> = lex(s);

    if l.is_ok() {
        // parse
    } else {
        println!("{}", l.err().unwrap());
    }
}

