
use std::fs::File;
use std::io::Read;
use std::path::Path;

extern crate juggle;

use juggle::*;

fn main() {
    let mut file = File::open(Path::new("test.txt")).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    let tokens: Vec<&str> = s.split_whitespace().collect();
    println!("{:?}", parse(tokens));
}

