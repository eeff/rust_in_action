//! Example complex from the book, chapter 2 code listing 2.6

use num::complex::Complex;

fn main() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let c = a + b;

    println!("{} + {}i", c.re, c.im);
}
