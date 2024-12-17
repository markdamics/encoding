use std::io;
use colored::Colorize;

fn main() {
    loop {
        println!("{}", "Encoding and decoding strings to Unicode:".bold().green());
        println!("{}", "--------------------------------------".green());
        println!("{}", "Choose an option:".green());
        println!("{}", "1. Encode to Unicode code points".green());
        println!("{}", "2. Encode to UTF-8 (bytes)".green());
        println!("{}", "3. Encode to UTF-8 binary".green());
        println!("{}", "4. Exit".green());
        // Prompt user for input
        println!("Enter an option: ");
        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option = option.trim();

        match option {
            "1" => encode_string_to_unicode(),
            "2" => encode_string_to_utf8(),
            "3" => encode_string_to_utf8_binary(),
            "4" => break,
            _ => println!("{}", "Invalid option, try again".red())
        }
    }
}

fn encode_string_to_unicode() ->() {
    // Prompt user for input
    println!("Enter a string: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    // Show the string in Unicode code points
    println!("Unicode Code Points:");
    for c in input.chars() {
        println!("Character: '{}', Code Point: U+{:X}", c, c as u32);
    }
    println!("\n\n\n");
}

fn encode_string_to_utf8() ->() {
    // Prompt user for input
    println!("Enter a string: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    // Show the string in UTF-8 encoding (bytes)
    let utf8_bytes = input.as_bytes();
    println!("UTF-8 Encoding (Bytes):");
    println!("{:?}", utf8_bytes);
    println!("\n\n\n");
}

fn encode_string_to_utf8_binary() ->() {
    // Prompt user for input
    println!("Enter a string: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    // Show the string in UTF-8 binary representation
    println!("\nUTF-8 Encoding (Binary):");
    for byte in input.as_bytes() {
        println!("{:08b}", byte);
    }
    println!("\n\n\n");
}
