use std::io;
use crate::board::Direction::*;
use crate::board::MoveOutcome::*;

mod board;

fn main() {

    println!("Hello!\nThis is an implementation of the popular app 2048,\
    written in its entirety by Justin Thein.\nHave fun!");
    let mut b = board::Board::startpos();
    loop {
        println!("{}", b);
        println!("Use WASD to specify merge direction. Press `q` to quit.");
        let mut input = String::new();
        let input: &str = {
            io::stdin().read_line(&mut input).expect("Failed to read input");
            &input.trim()
        };
        let result = match input {
            "w" => { println!("You entered {}", Up); Ok(b.mv(Up)) },
            "a" => { println!("You entered {}", Left); Ok(b.mv(Left)) },
            "s" => { println!("You entered {}", Down); Ok(b.mv(Down)) },
            "d" => { println!("You entered {}", Right); Ok(b.mv(Right)) },
            _ => Err(()),
        };
        match result {
            Ok(outcome) => {
                match outcome {
                    Ongoing => continue,
                    Win => {
                        println!("Hooray! You won!");
                        return;
                    },
                    Loss => {
                        println!("Game over. Better luck next time.");
                        return;
                    },
                    NoMove => println!("Oops! That move's not allowed right now!"),
                }
            }
            Err(_) => {
                if input.eq("q") {
                    println!("Quitting...");
                    return;
                } else {
                    println!("Unable to parse the provided input: `{}`", input);
                }
            }
        }
    }
}
