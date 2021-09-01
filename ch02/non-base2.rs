//! Example non-base2 from the book, chapter 2 code listing 2.4

fn main() {
    let three = 0b11; // binary
    let thirty = 0o36; // octal
    let three_hundred = 0x12c; // hexadecimal

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
}
