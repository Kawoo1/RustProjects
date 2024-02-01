use std::collections::HashMap;
use rocket::{Rocket, get, post};
use rocket::response::Redirect;
use rocket::request::Form;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

//  Authored by: Kyle Shanahan
//  This Rust program takes a URL input and shortens it to a short url
/*
    Instructions to run the program:
    1) Open BASH Termnal
    ** Instructions based off users utilizing VSCode **
    2) In BASH terminal enter the following commands: 
    - code url_shortener
    - locate and open the Cargo.toml file. (You can do this by navigating to the src directory and finding the Cargo.toml file.)
    3) Add the following dependencies under the [dependencies] section of the Cargo.toml file
    - [dependencies]
    - rocket = "0.5.0"
    - rocket_codegen = "0.5.0"
    - rand = "0.8.4"
    4) Open a new BASH terminal
    5) Enter the following command: cargo run
    The program is now set up. Use the following command to shorten URL's:
   
    curl -X POST -d "long_url= >REPLACE THIS HERE WITH THE URL<" http://localhost:8000/shorten
*/

#[derive(FromForm)]
struct UrlForm {
    long_url: String,
}

struct State {
    url_mapping: HashMap<String, String>,
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to the URL Shortener!"
}

#[post("/shorten", data = "<url_form>")]
fn shorten(state: &rocket::State<State>, url_form: Form<UrlForm>) -> String {
    let short_id: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .collect();

    let long_url = url_form.into_inner().long_url;
    state.url_mapping.insert(short_id.clone(), long_url.clone());

    let shortened_url = format!("http://localhost:8000/go/{}", short_id);
    shortened_url
}

#[get("/go/<short_id>")]
fn redirect(state: &rocket::State<State>, short_id: String) -> Redirect {
    match state.url_mapping.get(&short_id) {
        Some(long_url) => Redirect::to(long_url.clone()),
        None => Redirect::to("/"),
    }
}

fn rocket() -> Rocket {
    let mut url_mapping = HashMap::new();
    url_mapping.insert("example".to_string(), "http://www.example.com".to_string());

    rocket::ignite()
        .mount("/", routes![index, shorten, redirect])
        .manage(State { url_mapping })
}

fn main() {
    rocket().launch();
}
