//! Example intro-to-numbers from the book, chapter 2 code-listing 2.3

fn main() {
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;

    let addition = twenty + twenty_one + twenty_two;
    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2)); // numbers have methods

    let forty_twos = [
        42.0, // inferred to f32
        42f32, 42.0_f32,
    ];
    println!("{:02}", forty_twos[0]);
}
