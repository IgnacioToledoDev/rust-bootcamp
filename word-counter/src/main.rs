use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    println!("Word counter");
    println!("Attach a file to counter the words has it");

    // TODO: check this collect method is private warning
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run file_path");
        return;
    }

    dbg!(args[1]);
    let file_path = &args[1];
    println!("Reading file: {}", file_path);

    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => {
            println!("Cannot get the file with the path {}", file_path);
            return;
        }
    };

    let mut contents = String::new();
    if let Err(err) = file.read_to_string(contents) {
        println!("Error reading file: {}", err);
        return;
    }

     // Words counter
    let word_count = count_words(contents);
    println!("Word count: {}", word_count);
}

fn count_words(file_content: &str) -> usize {
    file_content.split_whitespace().count()
}