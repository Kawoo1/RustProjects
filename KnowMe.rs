use std::collections::HashMap;
use std::io;

// To run this code, open terminal and type the following two lines:
// 1) $ rustc KnowMe.rs
// 2) $ ./KnowMe


fn main() {
    // Create a HashMap to store user information
    // A Hashmap, in rust, is a collection type that allows you to store key-value pairs
    // In this code, the key is a string and the value is a string
    let mut profiles: Vec<HashMap<&str, String>> = Vec::new();

    // Loop until user types 'exit', allowing the user to add more profiles
    loop {
        let mut user_table: HashMap<&str, String> = HashMap::new();

        println!("Enter your Name (or type 'exit' to finish):");
        let name = get_user_input();

        // Exit to the loop
        if name.to_lowercase() == "exit" {
            break;
        }

        println!("Enter your Age:");
        let age = get_user_input();

        println!("Enter your Height:");
        let height = get_user_input();

        println!("Enter your Weight:");
        let weight = get_user_input();

        println!("Enter your Race:");
        let race = get_user_input();

        println!("Enter your Hair color:");
        let hair_color = get_user_input();

        user_table.insert("Name", name);
        user_table.insert("Age", age);
        user_table.insert("Height", height);
        user_table.insert("Weight", weight);
        user_table.insert("Race", race);
        user_table.insert("HairColor", hair_color);

        profiles.push(user_table);
    }

    // Display profiles in alphabetical order
    profiles.sort_by(|a, b| a["Name"].cmp(&b["Name"]));

    println!("\nUser Profiles (Alphabetical Order):");
    for profile in &profiles {
        for (key, value) in profile {
            println!("{}: {}", key, value);
        }
        println!(); // Separate profiles with an empty line
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

// Character constraints 
// | Adds constraints so only letters (chars) are allowed in the fields:
// - Name
// - Race
// - Hair color
// | Adds constraints so that only numbers (ints) are allowed to be entered in the fields:
// - Age
// - Height
// - Weight
fn get_valid_input(field_name: &str) -> String {
    loop {
        let input = get_user_input();
        if input.chars().all(char::is_alphabetic) {
            return input;
        } else {
            println!("Invalid input for {}. Please enter only letters.", field_name);
        }
    }
}
