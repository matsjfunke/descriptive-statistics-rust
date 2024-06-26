use std::error::Error;
use std::fs::File;
use csv::Reader;
use plotters::prelude::*;

mod top_ten;
use crate::top_ten::print_top_10;  // Import the print_top_10 function from the top_ten module

mod models; 
use models::{Player, SortingCriteria}; // Import types from models

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

// Function to plot scatter plot
fn plot_scatter(players : &[Player]) -> Result<(), Box<dyn Error>> {
    let root_area = BitMapBackend::new("images/scatter_plot.png", (1000, 800)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Goals vs Market Value", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..100,1_000_000..180_000_000)?;

    chart.configure_mesh().draw()?;

    // Draw the scatter plot
    chart.draw_series(
        players.iter().map(|player| Circle::new((player.goals, player.market_value), 3, RED.filled())),
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
