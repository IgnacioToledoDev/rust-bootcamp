use std::io;

fn main() {
    println!("Fibonacci sequence generator!");
    println!("Enter the number of terms you want generate:");

    let num_terms = match get_input_as_u32() {
        Some(value) => value,
        None => {
            println!("Invalid input. Please enter a positive integer.");
            return;
        }
    };

    if num_terms == 0 {
        println!("Number of terms must be greater than zero");
        return;
    }

    let sequence = generate_sequence(num_terms);
    println!("Fibonacci sequence generated: {:?}", sequence)
}

fn get_input_as_u32() -> Option<u32> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input ");

    input.trim().parse::<u32>().ok()
}

fn generate_sequence(num: u32) -> Vec<u32> {
    let mut sequence = Vec::new();

    if num >= 1 {
        sequence.push(0); // First term
    }

    if num >= 2 {
        sequence.push(1); // Second term
    }

    for i in 2..num {
        let next = sequence[i as usize - 1] + sequence[i as usize - 2];
        sequence.push(next);
    }

    sequence
}
