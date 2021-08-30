//! Example hello2 from the book, chapter 1 code listing 1.1
//!
//! To compile the code:
//! ``` shell
//! rustc hello.rs
//! ```

fn main() {
    greet_world();
}

fn greet_world() {
    println!("Hello, world!");
    let southern_germany = "Grüß Gott!";
    let japan = "ハロー・ワールド";
    let regions = [southern_germany, japan];
    for region in regions.iter() {
        println!("{}", region);
    }
}
