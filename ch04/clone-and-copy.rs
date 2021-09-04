//! Example check-sat-clone-and-copy from the book, chapter 4 code listing 4.18

#[derive(Clone, Copy, Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

impl Copy for StatusMessage {}

impl Clone for StatusMessage {
    fn clone(&self) -> Self {
        *self
    }
}

fn check_status(_sat: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    let sat_a = CubeSat { id: 0 };

    let a_status = check_status(sat_a);
    println!("a: {:?}", a_status);

    let a_status = check_status(sat_a.clone());
    println!("a: {:?}", a_status.clone());
}
