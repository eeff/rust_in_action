//! Example while-true-incr-count from the book, chapter 2 code listing 2.7

use std::time::{Duration, Instant};

fn main() {
    let mut count = 0;
    let time_limit = Duration::new(1, 0);
    let start = Instant::now();

    while (Instant::now() - start) < time_limit {
        count += 1;
    }
    println!("{}", count);
}
