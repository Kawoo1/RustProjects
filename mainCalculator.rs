use std::io::{self, Write};

// main is the entry point of every rust program, this main begins with creating an infinite loop that
// holds the "calculator" and all the user options. This calculator can do the 4 basic mathematical operations.

// To run the program:
// 1) In BASH Terminal, type rustc mainCalculator.rs
// 2) Then type, ./mainCalculator
fn main() {
    loop {
        println!("Simple Calculator");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Exit");

        let choice = get_user_input("Enter your choice: ");

        match choice.trim().parse::<u32>() {
            Ok(1) => perform_operation("Addition", |a, b| a.checked_add(b)),
            Ok(2) => perform_operation("Subtraction", |a, b| a.checked_sub(b)),
            Ok(3) => perform_operation("Multiplication", |a, b| a.checked_mul(b)),
            Ok(4) => perform_operation("Division", |a, b| {
                if b == 0 {
                    println!("Cannot divide by zero");
                    None
                } else {
                    a.checked_div(b)
                }
            }),
            Ok(5) => {
                println!("Exiting the calculator. Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please enter a valid option."),
        }
    }
}

fn perform_operation(operation: &str, op: fn(i64, i64) -> Option<i64>) {
    let num1 = get_user_input("Enter the first number: ").trim().parse::<i64>();
    let num2 = get_user_input("Enter the second number: ").trim().parse::<i64>();

    match (num1, num2) {
        (Ok(num1), Ok(num2)) => {
            // Change made to use Option<i64> instead of i32 for result. Allowing for a larger scope.
            if let Some(result) = op(num1, num2) {
                println!("Result of {}: {} {} = {}", operation, num1, num2, result);
            } else {
                // New overflow error message
                println!("Overflow error");
            }
        }
        _ => println!("Invalid input. Please enter valid numbers."),
    }
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}

