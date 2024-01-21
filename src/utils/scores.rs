use super::traits::Printable;
use super::structs::Player;

/// Define a function to update the scores
pub fn update_scores(players: &mut Vec<Player>, winner: &str) {
    for player in players {
        if player.name == winner {
            player.score += 1
        }
    }
}

/// Define a function to print the scores
pub fn print_scores(players: &Vec<Player>) {
    println!("Scores: 📊");
    for player in players {
        println!("- {}", player.to_string());
    }
}