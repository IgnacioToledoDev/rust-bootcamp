use std::io;

fn main() {
    println!("Hello, world!");
    let input = get_input();
    let cleaned_input = clean_string(&input);

    if cleaned_input.is_empty() {
        println!("Please enter a valid non-empty string.");
        return;
    }

    if is_palindrome(&cleaned_input) {
        println!("{cleaned_input}, is a palindrome");
    } else {
        println!("{cleaned_input}, not is a palindrome")
    }
}

fn get_input() -> String {
    println!("Please enter a word");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input
}

fn clean_string(input: &str) -> String {
    input
        .chars()
        .filter(|char| char.is_alphanumeric())
        .map(|char| char.to_lowercase().to_string())
        .collect()
}

fn is_palindrome(word: &str) -> bool {
    word == word.chars().rev().collect::<String>()
}
