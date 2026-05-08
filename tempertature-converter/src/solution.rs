pub fn solution() {
    println!("Temperature converter");
    println!("1: Celsius to Fahrenheit");
    println!("2: Fahrenheit to Celsius");

    let mut choice = String::new();
    std::io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid choice. Please enter 1 or 2");
            return;
        }
    };

    if choice == 1 {
        celsius_to_fahrenheit();
    } else if choice == 2 {
        fahrenheit_to_celsius();
    } else {
        println!("Invalid choice. Please enter 1 or 2");
    }
}

fn celsius_to_fahrenheit() {
    let input = get_user_input();
    let fahrenheit = (input * 9.0 / 5.0) + 32.0;

    println!("{input}C is {fahrenheit}F");
}

fn fahrenheit_to_celsius() {
    let input = get_user_input();
    let celsius = (input - 32.0) * 5.0 / 9.0;

    println!("{input}F is {celsius}C");
}

fn get_user_input() -> f64 {
    println!("Enter the value");
    let mut temp = String::new();
    std::io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");
    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input . Please enter a valid number");
            get_user_input()
        }
    };

    temp
}
