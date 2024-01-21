use rand::Rng;
use reqwest::*;

/// Define a function to generate a random number

/// M-1: via `rand` library
pub (crate) fn generate_number(max_range: u32) -> u32 {
    rand::thread_rng().gen_range(1..max_range)
}

#[tokio::main]
// M-2: via API
pub (crate) async fn generate_number_async(max_range: u32) -> Result<u32> {
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
    println!("Async value = {:?}", val);

    Ok(val)
}

#[cfg(test)]
/// Define unit tests for the functions
mod tests {
    use super::*;

    #[test]
    /// Test if generated random number is valid
    fn test_valid_rng() {
        let max_val = 100;
        let rand_value = generate_number(max_val);

        assert!(rand_value >= 1 && rand_value <= max_val);
    }
}