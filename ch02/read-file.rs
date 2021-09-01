//! Example read-file from the book, chapter 2 code listing 2.27

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("read-file.rs").unwrap();
    let mut reader = BufReader::new(f);
    let mut line = String::new();

    loop {
        let len = reader.read_line(&mut line).unwrap();

        if len == 0 {
            break;
        }

        println!("{} ({} bytes long)", line, len);

        line.truncate(0);
    }
}
