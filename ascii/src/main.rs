use std::{io};
use std::fs::File;
use std::io::{BufRead, BufReader};
use prettytable::{row, Table};

fn main() {
    //let ascii_value:u8 = 65;
    //let ascii_char = ascii_value as char;
    //println!("The ASCII value {} is the character {}", ascii_value, ascii_char);

    println!("The ASCII table:");

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

fn load_descriptions(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)
        .expect("Could not open file");

    let reader = BufReader::new(file);

    let descriptions:Vec<String> = reader.lines()
        .collect::<Result<_, _>>()?;

    Ok(descriptions)
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
