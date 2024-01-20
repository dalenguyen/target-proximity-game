use rand::Rng;
use reqwest::*;

/// Define a function to generate a random number
/// M-1: via `rand` library
pub fn generate_number(max_range: u32) -> u32 {
    rand::thread_rng().gen_range(1..max_range)
}

#[tokio::main]
// M-2: via API
pub async fn generate_number_async(max_range: u32) -> Result<u32> {
    // Get URL value from the environment
    dotenv::from_path("./.env").expect("Failed to load .env file");
    let url = std::env::var("URL")
        .expect("URL var not found")
        .replace("{MAX}", &max_range.to_string());

    let body = reqwest::get(url)
    .await?
    .text()
    .await?;

    let val = body.trim().parse::<u32>().expect("Error in parsing");
    println!("value = {:?}", val);

    Ok(val)
}