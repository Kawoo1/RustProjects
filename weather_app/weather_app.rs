/* 
 This Rust program "weather_app" requires the following steps in BASH Terminal to create and run the wwether app correctly:
 1) cd /path/to/your_project_directory
 2) mkdir weather_app
 3) cd weather_app
 4) Creation of the weather app's cargo.toml file by entering the following code:
 [package]
 name = "weather-app"
 version = "0.1.0"
 edition = "2021"
 [dependencies]
 reqwest = "0.11"
 serde = { version = "1.0", features = ["derive"] }
 serde_json = "1.0"
 [build-dependencies]
 tokio = { version = "1", features = ["full"] }
 5) cd ..
 6) mkdir src
 7) Inside the src directory, create a file named main.rs with the Rust code provided below:
*/

use serde::Deserialize;
  
#[derive(Debug, Deserialize)]
struct WeatherData {
    main: Main,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Main {
    temp: f32,
    humidity: f32,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let api_key = "YOUR_OPENWEATHERMAP_API_KEY";
    let city_name = "CITY_NAME"; // Replace with the desired city name

    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        city_name, api_key
    );

    let response = reqwest::get(&url).await?;
    let weather_data: WeatherData = response.json().await?;

    println!("Weather in {}: ", weather_data.name);
    println!("Temperature: {:.2}°C", kelvin_to_celsius(weather_data.main.temp));
    println!("Humidity: {:.2}%", weather_data.main.humidity);

    Ok(())
}

fn kelvin_to_celsius(kelvin: f32) -> f32 {
    kelvin - 273.15
}

/* 8) cd ..
   9) The program is now integrated and setup correctly. You can run the weather app by entering: cargo run
   from the root directory.
*/
