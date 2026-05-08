use std::io;

fn main() {
    println!("Simple calculator in Rust");
    println!("Insert a expresion e.g. 5 + 6");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let tokens: Vec<&str> = input.split_whitespace().collect();

    if tokens.len() > 3 {
        println!("Operation can be <number_1> <operator> <number_2> e.g 2 + 5");
        return;
    }

    let num1: u64 = match tokens[0].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error getting num1");
            return;
        }
    };

    let operation = tokens[1];

    let num2: u64 = match tokens[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error getting num2");
            return;
        }
    };

    let result = match operation {
        "+" => sum(num1, num2),
        "-" => rest(num1, num2),
        "/" => divide(num1, num2),
        "*" => multiply(num1, num2),
        _ => {
            print!("Error match with operation {operation}");
            return;
        }
    };

    println!("Result: {}", result)

    // Interation of vector with for for_each
    // tokens.iter().for_each(|f| println!("{f}"))
}

fn sum(num1: u64, num2: u64) -> u64 {
    num1 + num2
}

fn rest(num1: u64, num2: u64) -> u64 {
    num1 - num2
}

fn multiply(num1: u64, num2: u64) -> u64 {
    num1 * num2
}

fn divide(num1: u64, num2: u64) -> u64 {
    if num2 == 0 {
        println!("Cannot divide by Zero");
        std::process::exit(1);
    }

    num1 / num2
}
