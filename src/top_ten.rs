use crate::models::{Player, SortingCriteria}; // Import types from models

pub fn print_top_10(players: &[Player], criteria: SortingCriteria) {
    let mut sorted_players = players.to_vec();
    match criteria {
        SortingCriteria::MarketValue => {
            sorted_players.sort_by(|a, b| b.market_value.partial_cmp(&a.market_value).unwrap_or(std::cmp::Ordering::Equal));
        },
        SortingCriteria::Goals => {
            sorted_players.sort_by(|a, b| b.goals.cmp(&a.goals));
        },
    }

    let num_players_to_print = std::cmp::min(10, sorted_players.len());
    match criteria {
        SortingCriteria::MarketValue => {
            println!("Top {} most valuable players:", num_players_to_print);
            for (rank, player) in sorted_players.iter().take(num_players_to_print).enumerate() {
                println!("{}. {} - Market Value: {}", rank + 1, player.name, player.market_value);
            }
        },
        SortingCriteria::Goals => {
            println!("Top {} players with most goals:", num_players_to_print);
            for (rank, player) in sorted_players.iter().take(num_players_to_print).enumerate() {
                println!("{}. {} - Goals: {}", rank + 1, player.name, player.goals);
            }
        },
    }
}
