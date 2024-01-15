use std::io;
use tokio::time::sleep;
use std::time::Duration;

#[tokio::main]
async fn main() {
    loop {
        println!("Choose an option:");
        println!("1. Dance");
        println!("2. Exit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => {
                dance().await;
            }
            "2" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please enter 1 or 2."),
        }
    }
}

async fn dance() {
    println!("Dancing!");
    
    // Sleep for 3 seconds
    sleep(Duration::from_secs(3)).await;

    // Clear the screen after 3 seconds
    print!("\x1B[2J\x1B[H");
}
