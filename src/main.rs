use std::io;
use crate::utils::helper::generate_number;
use crate::utils::helper::generate_number_async;
use crate::utils::structs::Player;

pub mod utils;

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

/// Define a function to get the proximity from the players
/// [proximity = abs_diff(guess, target)]
fn collect_guesses_into_proximities(players: &Vec<Player>, max_range: u32) -> Vec<(String, u32)> {
    let mut player_proximities = Vec::<(String,u32)>::new();
    let target = generate_number(create_max_range(players)); //.expect("Failure to generate number");
    println!("Target: {}", target);

    for player in players {
        println!("{}'s turn", player.name);
        let guess = collect_input::<u32>(&format!("Guess the number (1-{max_range}):"));
        player_proximities.push((player.name.clone(), guess.abs_diff(target)));
    }

    player_proximities
}

/// Define a function to get the winner
fn get_winner(player_proximities: &Vec<(String, u32)>) -> String {
    player_proximities[0].0.to_owned()
}

/// Define a function to update the scores
fn update_scores(players: &mut Vec<Player>, winner: &str) {
    for player in players {
        if player.name == winner {
            player.score += 1
        }
    }
}

/// Define a function to print the scores
fn print_scores(players: &Vec<Player>) {
    println!("Scores: ");
    for player in players {
        println!("- {}", player.to_string());
    }
}

/// Define the main function to run the game
fn main() {
    // collect_players();
    let test = generate_number(100);
    println!("Value {}", test);

    let test2 = generate_number_async(100).expect("Failed to get random number");
    println!("Value {}", test2);
}