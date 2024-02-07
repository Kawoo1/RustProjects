// Filename: mainArray.rs
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug)]
struct User {
    name: String,
    email: String,
}

fn generate_random_name() -> String {
    let names = ["Alice", "Bob", "Charlie", "David", "Eve", "Frank", "Grace", "Hank", "Ivy", "Jack"];
    let mut rng = rand::thread_rng();
    *names.choose(&mut rng).unwrap_or(&"Unknown").to_string()
}

fn generate_random_email(name: &str) -> String {
    format!("{}@example.com", name.to_lowercase())
}

fn main() {
    const ARRAY_SIZE: usize = 100;

    let mut users: Vec<User> = Vec::with_capacity(ARRAY_SIZE);

    for _ in 0..ARRAY_SIZE {
        let name = generate_random_name();
        let email = generate_random_email(&name);

        let user = User { name, email };
        users.push(user);
    }

    // Print the generated users
    for (index, user) in users.iter().enumerate() {
        println!("User {}: {:?}", index + 1, user);
    }
}
