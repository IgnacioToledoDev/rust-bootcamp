use std::io;

fn main() {
    println!("Prime Number Checker!");
    println!("Please enter a number");
    let number = match get_input_as_u32() {
        Some(value) => value,
        None => {
            println!("Invalid input, please enter a positive integer.");
            return;
        }
    };

    if number <= 1 {
        println!("The number must be greater than 1");
        return;
    }

    if is_prime(number) {
        println!("{number} is a prime number");
    } else {
        println!("{number} not is a prime number");
    }
}

fn get_input_as_u32() -> Option<u32> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().parse().ok()
}

fn is_prime(number: u32) -> bool {
    if number <= 1 {
        return false;
    }

    if number == 2 {
        return true;
    }

    if number.is_multiple_of(2) {
        return false;
    }

    let limit = (number as f64).sqrt() as u32 + 1;
    for i in 3..limit {
        if (number % i).is_multiple_of(1) {
            return false;
        }
    }

    true
}
