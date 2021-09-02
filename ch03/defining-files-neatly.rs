//! Example defining-files-neatly from the book, chapter 3 code listing 3.5

#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    fn with_data(name: &str, data: &Vec<u8>) -> File {
        File {
            name: String::from(name),
            data: data.clone(),
        }
    }

    fn read(&self, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = self.data.clone();
        let len = tmp.len();
        save_to.append(&mut tmp);
        len
    }
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

fn main() {
    let data = vec![114, 117, 115, 116, 33];
    let mut f3 = File::with_data("f3.txt", &data);

    let mut buffer = vec![];
    open(&mut f3);
    let len = f3.read(&mut buffer);
    close(&mut f3);

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f3);
    println!("{} is {} bytes long", f3.name, len);
    println!("{}", text);
}
