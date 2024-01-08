use std::collections::HashMap;
use std::io;

// To run this code, open terminal and type the following two lines:
// 1) $ rustc KnowMe.rs
// 2) $ ./KnowMe


fn main() {
    // Create a HashMap to store user information
    // A Hashmap, in rust, is a collection type that allows you to store key-value pairs
    // In this code, the key is a string and the value is a string
    let mut user_table: HashMap<&str, String> = HashMap::new();

    // Prompt user for input and store it in the HashMap
    println!("Enter your Name:");
    let name = get_user_input();

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

    // This is where it stores the user data
    user_table.insert("Name", name);
    user_table.insert("Age", age);
    user_table.insert("Height", height);
    user_table.insert("Weight", weight);
    user_table.insert("Race", race);
    user_table.insert("HairColor", hair_color);

    // Display the stored information
    println!("\nUser Information:");
    for (key, value) in &user_table {
        println!("{}: {}", key, value);
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
