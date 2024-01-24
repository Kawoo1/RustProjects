use std::fs::{File, remove_file};
use std::io::prelude::*;

// This file was authored by Kyle Shanahan
// The program does the following:
// - This program locates a file on a local computer system (Modify the path if needed)
// - It also deletes the file (if desired, and if it exists)

// To run the program execute the following in BASH:
// 1) cargo new file_Finder
// 2) cd file_Finder
// 3) cargo run
// The program will start in a new terminal window.

fn main() -> std::io::Result<()> {
    // Specify the file path
    let file_path = "C:\Program Files (x86)";

    // Attempt to open the file
    let mut file = File::open(file_path)?;

    // Read the contents of the file into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Print the contents of the file
    println!("File Contents:\n{}", contents);

    // ---------- Delete the file --------------- 
    // remove_file(file_path)?;
    // println!("File deleted successfully.");
    // ------------------------------------------
    
    // "Ok" in Rust is returned to indicate that the program completed successfully
    Ok(())
}
