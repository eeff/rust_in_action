//! Example letters from the book, chapter 1 code listing 1.6
//! This code does not compile

fn main() {
    let mut letters = vec!["a", "b", "c"];

    for letter in letters {
        println!("{}", letter);
        letters.push(letter.clone());
    }
}
