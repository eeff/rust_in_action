//! Example bufreader-lines from the book, chapter 2 code listing 2.28

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("bufreader-lines.rs").unwrap();
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
}
