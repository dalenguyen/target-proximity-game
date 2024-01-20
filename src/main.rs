use std::io;
use crate::utils::helper::generate_number;
use crate::utils::helper::generate_number_async;

pub mod utils;

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
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        
        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => continue
        }
    }
}

/// Define a function to get the players
fn collect_players() -> Vec<Player> {
    let mut players: Vec<Player> = Vec::new();
    let mut num_players = 0;

    loop {
        num_players = collect_input::<u32>("How may players (>=2)?");
        if num_players < 2 {
            println!("Invalid X no! Please try a gain!");
            continue;
        } else {
            break; 
        }
    }

    for i in 1..=num_players {
        let name = collect_input(format!("Player {} name:", i).as_str());
        players.push(Player {name, score: 0});
    }
    players
}

/// Define a function to get the max number
fn create_max_range(players: &Vec<Player>) -> u32 {
    players.len() as u32 * 50
}

// /// Define a function to generate a random number
// /// M-1: via `rand` library
// fn generate_number(max_range: u32) -> u32 {
//     rand::thread_rng().gen_range(1..max_range)
// }

/// Define the main function to run the game
fn main() {
    // collect_players();
    let test = generate_number(100);
    println!("Value {}", &test);

    let test2 = generate_number_async(100);
    // println!("Value {}", &test2);
}