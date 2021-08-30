//! Example penguin from the book, chapter 1 code listing 1.2
//!
//! To compile the code:
//! ``` shell
//! rustc penguin.rs
//! rustc -C debug_assertions=no penguin.rs
//! ```

fn main() {
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yello-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    for (i, record) in penguin_data.lines().enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();

        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length);
        }
    }
}
