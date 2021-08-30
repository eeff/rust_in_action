//! Example fruit from the book, chapter 1 code listing 1.5
//!
//! To compile the code:
//! ``` shell
//! rustc fruit.rs
//! ```

fn main() {
    let fruit = vec!['🥝', '🍌', '🍇'];

    let buffer_overflow = fruit[4]; // panic!
    assert_eq!(buffer_overflow, '🍉');
}
