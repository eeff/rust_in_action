//! Example race from the book, chapter 1 code listing 1.4
//! This code does not compile.

use std::thread;

fn main() {
    let mut data = 100;

    thread::spawn(|| {
        data = 500;
    });
    thread::spawn(|| {
        data = 1000;
    });
    println!("{}", data);
}
