use std::io;
use rand::{RngExt};
use std::cmp::Ordering;

fn main() {
    let secret_numer = rand::rng().random_range(1..100);
    println!("{secret_numer}");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // transform the user input in a f64
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Try again, input cannot transform in a integer value");
                continue;
            }
        };

        match input.cmp(&secret_numer) {
            Ordering::Greater => println!("Greater"),
            Ordering::Less => println!("Less"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
    }
}