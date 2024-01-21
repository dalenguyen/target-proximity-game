use utils::traits::Printable;

use crate::utils::helper::generate_number;
use crate::utils::inputs::collect_input;
use crate::utils::scores::{update_scores, print_scores};
// use crate::utils::helper::generate_number_async;
use crate::utils::structs::Player;

pub mod utils;

/// Implement the Printable trait for the Play struct
impl Printable for Player {
    fn to_string(&self) -> String {
        format!("{} ({})", self.name, self.score)
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
    // println!("Target: {} \n", target);

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

/// Define a function to play the game
/// This function starts the Target Proximity Game. It gets the players, creates the max range
/// and runs the game in a loop until the players decide to stop playing. It prints the winner
/// and updates the scores of the players
fn play_game() {
    println!("Welcome to the Target Proximity Game! 🎮");
    let mut players = collect_players();
    let max_range = create_max_range(&players);

    loop {
        let mut player_proximities = collect_guesses_into_proximities(&players, max_range);
        player_proximities.sort_by_key(|&(_, v) | v);
        let winner = get_winner(&player_proximities);

        println!("Congratulations, {}! You are the winner! 🏆🎉 \n", winner);
        update_scores(&mut players, &winner);
        print_scores(&players);

        let play_again: String = collect_input("Play again? (y/n)");

        // if input is anything other than "y", it breaks
        if play_again.to_ascii_lowercase() != "y" {
            break;
        }
     }
}

/// Define the main function to run the game
fn main() {
    play_game();
}

#[cfg(test)]
/// Define unit tests for the functions
mod tests {
    use super::*;

    #[test]
    /// Test if the max_range for given players is invalid
    fn test_create_max_range() {
        let players = vec![
            Player {
                name: "Dale".to_string(),
                score: 0,
            },
            Player {
                name: "Yen".to_string(),
                score: 0
            }
        ];

        let max_range = create_max_range(&players);
        assert_eq!(max_range, players.len() as u32 * 50);
    }

    #[test]
    /// Test if the player is correctly displayed
    fn test_player_to_string() {
        let player = Player {
            name: "Xoai".to_string(),
            score: 3
        };

        assert_eq!(player.to_string(), "Xoai (3)");
    }
}