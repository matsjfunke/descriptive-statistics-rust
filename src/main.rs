use std::error::Error;
use std::fs::File;
use csv::Reader;
use serde_derive::Deserialize;
use plotters::prelude::*;

#[derive(Debug, Deserialize, Clone)]
struct Player {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "MarketValue")]
    market_value: i32,
    #[serde(rename = "Goals")]
    goals: i32,
}

// Enum to specify the sorting criteria
enum SortingCriteria {
    MarketValue,
    Goals,
}

fn read_csv(file_path: &str) -> Result<Vec<Player>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = Reader::from_reader(file);

    let mut data = Vec::new();

    for result in rdr.deserialize() {
        let player: Player = result?;
        // Filter out goalkeepers
        if player.position != "Goalkeeper" {
            data.push(player);
        }
    }

    Ok(data)
}

// Function to print player details
fn print_all_players(players: &[Player]) {
    for player in players {
        println!("Name: {}, Market Value: {}, Goals: {}", player.name, player.market_value, player.goals);
    }
}

// Function to print top 10 goals & market_value
fn print_top_10(players: &[Player], criteria: SortingCriteria) {
    // Sort players based on the specified criteria
    let mut sorted_players = players.to_vec();
    match criteria {
        SortingCriteria::MarketValue => {
            sorted_players.sort_by(|a, b| b.market_value.partial_cmp(&a.market_value).unwrap_or(std::cmp::Ordering::Equal));
        },
        SortingCriteria::Goals => {
            sorted_players.sort_by(|a, b| b.goals.cmp(&a.goals));
        },
    }

    // Print the top 10 players or less if fewer than 10 players exist
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

// Function to plot scatter plot
fn plot_scatter(players : &[Player]) -> Result<(), Box<dyn Error>> {
    let root_area = BitMapBackend::new("scatter_plot.png", (1000, 800)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Goals vs Market Value", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..50, 0..100_000_000)?;

    chart.configure_mesh().draw()?;

    // Draw the scatter plot
    chart.draw_series(
        players.iter().map(|player| Circle::new((player.goals, player.market_value), 3, BLUE.filled())),
    )?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read data from CSV
    let players = read_csv("euro2024_players.csv")?;

    // Print player details
    // print_all_players(&players);

    // Plot scatter plot
    plot_scatter(&players)?;

    // print top 10 most valuable players
    print_top_10(&players, SortingCriteria::MarketValue);
    // print top 10 players with most national team goals
    print_top_10(&players, SortingCriteria::Goals);

    Ok(())
}
