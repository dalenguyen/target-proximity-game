use rand::Rng;

/// Define a function to generate a random number
/// M-1: via `rand` library
pub fn generate_number(max_range: u32) -> u32 {
    rand::thread_rng().gen_range(1..max_range)
}
