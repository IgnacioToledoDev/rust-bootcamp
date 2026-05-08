fn main() {
    println!("Temperature converter");
    // get the value to converter
    println!("Insert the value to convert");
    let mut value_to_convert = String::new();
    std::io::stdin()
        .read_line(&mut value_to_convert)
        .expect("Failed to read line");

    // select 1 if is celsius
    // select 2 if is fahrenheit
    println!(
        "Write 1 if is celsius to converter to fahrenheit or write 2 if is fahrenheit to celsius"
    );
    let mut user_option = String::new();
    std::io::stdin()
        .read_line(&mut user_option)
        .expect("Failed to read line");

    let allowed_option = get_option(user_option.trim().parse().unwrap());

    // show the value converter with a good description
    show_converter(value_to_convert.trim().parse().unwrap(), allowed_option);
}

fn get_option(user_option: i8) -> String {
    let mut allowed_action = String::new();
    if user_option == 1 {
        allowed_action = "celsius".to_owned();
    }

    if user_option == 2 {
        allowed_action = "fahrenheit".to_owned();
    }

    println!("Option selected: {allowed_action}");
    allowed_action
}

#[warn(unused_assignments)]
fn show_converter(value: f32, option: String) -> Option<()> {
    let mut converter: f32 = 0.0;
    let mut oposite: String = String::new();
    if option == "celsius" {
        converter = (value * 9.0 / 5.0) + 32.0;
        oposite = String::from("Fahrenheit");
    } else {
        converter = (value - 32.0) * 5.0 / 9.0;
        oposite = String::from("Celsius");
    }

    println!("Value to converter {value} {option}, Result {converter} {oposite}");
    None
}
