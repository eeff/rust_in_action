//! Example from the book, chapter 2 code listing 2.5

use std::convert::TryInto;

fn main() {
    let a: i32 = 10;
    let b: u16 = 100;

    let b = b.try_into().unwrap();

    if a < b {
        println!("Ten is less than one hundred.");
    }
}
