use std::io;

mod board;

fn main() {
    let test = board::Board::startpos();
    println!("{}", test);
}
