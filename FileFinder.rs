use std::fs::{File, remove_file};
use std::io::prelude::*;

// This file was authored by Kyle Shanahan
// The program does the following:
// - This program locates a file on a local computer system (Modify the path if needed)

fn main() -> std::io::Result<()> {
    // Specify the file path
    let file_path = "path/to/your/file.txt";

    // Attempt to open the file
    let mut file = File::open(file_path)?;

    // Read the contents of the file into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Print the contents of the file
    println!("File Contents:\n{}", contents);


    
    // "Ok" in Rust is returned to indicate that the program completed successfully
    Ok(())
}
