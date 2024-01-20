use std::io;

/// Define a struct to represent a Player
struct Player {
    name: String,
    score: u32
}

/// Define a trait to represent printable objects
trait Printable {
    fn to_string(&self) -> String;
}

/// Implement the Printable trait for the Play struct
impl Printable for Player {
    fn to_string(&self) -> String {
        format!("{} ({})", self.name, self.score)
    }
}

// Define a generic function to get user input
fn collect_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        println!("{}", prompt);
        let mut input = String::new()

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input")
        
        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => continue
        }
    }
}
/// Define the main function to run the game
fn main() {
    
}