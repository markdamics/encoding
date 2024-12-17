use std::{io};
use std::fs::File;
use std::io::{BufRead, BufReader};
use colored::Colorize;
use prettytable::{row, Table};

fn main() {
    loop {
        println!("{}", "Encoding and decoding strings to ASCII:".bold().green());
        println!("{}", "--------------------------------------".green());
        println!("{}", "Choose an option:".green());
        println!("{}", "1. Encode a string to ASCII".green());
        println!("{}", "2. Decode ASCII to a string".green());
        println!("{}", "3. Print the Basic ASCII table".green());
        println!("{}", "4. Exit".green());

        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option = option.trim();

        match option {
            "1" => encode_string_to_ascii(),
            "2" => decode_ascii_to_string(),
            "3" => load_ascii_table(),
            "4" => break,
            _ => println!("{}", "Invalid option, try again".red())
        }
    }
}

fn load_descriptions(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)
        .expect("Could not open file");

    let reader = BufReader::new(file);

    let descriptions:Vec<String> = reader.lines()
        .collect::<Result<_, _>>()?;

    Ok(descriptions)
}

fn load_ascii_table() -> () {
    println!("The ASCII table(0-127):");

    let mut table = Table::new();
    table.add_row(row![Fb=> "Dec", "Hex", "Char", "Description"]);

    let descriptions = load_descriptions("src/descriptions.txt");

    for i in 0..=127 {
        //Convert the loop index `i` to an ASCII value `u8`
        let ascii_value= i as u8;

        let printable_char: String = match i {
            0..=31 => separate_description_and_code(descriptions.as_ref().unwrap().get(i).unwrap()).unwrap().1,
            32..=126 => (ascii_value as char).to_string(),
            127 => separate_description_and_code(descriptions.as_ref().unwrap().get(32).unwrap()).unwrap().1,
            _ => "Unknown".to_string()
        };

        let description: String = match i {
            0..=31 => separate_description_and_code(descriptions.as_ref().unwrap().get(i).unwrap()).unwrap().0,
            32..=126 => "".to_string(),
            127 => separate_description_and_code(descriptions.as_ref().unwrap().get(32).unwrap()).unwrap().0,
            _ => "Unknown".to_string()
        };

        table.add_row(row![i, format!("{:X}", i), printable_char, description]);
    }

    table.printstd();
}

fn separate_description_and_code(data: &str) -> Option<(String, String)> {
    // Find the index of the opening parenthesis
    if let Some(pos) = data.find('(') {
        let description = data[..pos].trim().to_string();  // The part before '('
        let code = data[pos..].trim().to_string();         // The part from '(' onwards

        Some((description, code))
    } else {
        None  // Return None if no parenthesis is found
    }
}

fn encode_string_to_ascii() -> () {
    println!("Enter a string to encode to ASCII:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let ascii_chars: Vec<u8> = input.chars().map(|c| c as u8).collect();
    println!("The ASCII representation of the string is: {:?}", ascii_chars);
}

fn decode_ascii_to_string() -> () {
    // Prompt the user
    println!("{} {} {}", "Enter ASCII values (space-separated, e.g.,", "72 101 108".blue(), "):");

    // Read user input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Initialize vectors for valid and invalid inputs
    let mut ascii_codes = Vec::new();
    let mut invalid_inputs = Vec::new();

    // Parse the input
    for part in input.trim().split_whitespace() {
        match part.parse::<u8>() {
            Ok(code) => ascii_codes.push(code), // Valid ASCII value
            Err(_) => invalid_inputs.push(part.to_string()), // Invalid input
        }
    }

    // Check if there are invalid inputs
    if !invalid_inputs.is_empty() {
        let invalid: String = invalid_inputs.join(", ");
        let error_message = format!(
            "Invalid inputs detected: {}. Only numbers between 0 and 255 are valid ASCII codes.",
            invalid
        );
        println!("{}", error_message.red());
        return; // Exit early if invalid input is found
    }

    // Convert ASCII codes to a string
    let result: String = ascii_codes
        .iter()
        .map(|&code| code as char) // Convert each ASCII code to a char
        .collect();

    println!("Converted String: {}", result);
}
