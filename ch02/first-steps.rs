//! Example first-steps from the book, chapter 2 code-listing 2.2
//!
//! To compile the code:
//! ``` shell
//! rustc first-steps.rs
//! ```

fn main() {
    let a = 10; // type inferred, default to i32
    let b: i32 = 20; // type annotated
    let c = 30i32;
    let d = 30_i32; // underscore for readability
    let e = add(add(a, b), add(c, d));

    println!("(a+b) + (c+d) = {}", e);
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}
