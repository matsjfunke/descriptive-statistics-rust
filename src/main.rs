use std::error::Error;
use std::fs::File;
use csv::Reader;
use serde_derive::Deserialize;
use plotters::prelude::*;
use std::collections::HashMap; // import from std library

#[derive(Debug, Deserialize)]
struct Player {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "MarketValue")]
    market_value: i32, // Assuming market value is an integer
    #[serde(rename = "Goals")]
    goals: i32, // Assuming goals is an integer
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

    println!("{:?}", data);
    Ok(data)
}

// Function to print player details
fn print_all_players(players: &[Player]) {
    for player in players {
        println!("Name: {}, Market Value: {}, Goals: {}", player.name, player.market_value, player.goals);
    }
}

// Function to print top 10 goals & market_value
fn print_top_10(players: &[Player]) {
    let mut top_players = HashMap::new();

    for player in players {
        top_players.insert(String::from(player.name.clone()), player.market_value);
    }

    let player_name = String::from("Florian Wirtz");
    let top_player = top_players.get(&player_name).copied().unwrap_or(0);
    println!("{:?}", top_player);
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
     print_top_10(&players);
    // print top 10 players with most national team goals
    // print_top_10(&players);

    Ok(())
}
