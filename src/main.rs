use std::error::Error;
use std::fs::File;
use csv::Reader;

mod top_ten;
use crate::top_ten::print_top_10;  // Import the print_top_10 function from the top_ten module
mod models; 
use models::{Player, SortingCriteria}; // Import types from models
mod statistics;
mod plot;
use crate::plot::plot_scatter;
mod linear_regression;
use linear_regression::{extract_goals_and_market_value, train_linear_regression, predict};

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
    println!("All non Goalkeepers in the EURO 2024");
    for player in players {
        println!("Name: {}, Market Value: {}, Goals: {}", player.name, player.market_value, player.goals);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read data from CSV
    let players = read_csv("euro2024_players.csv")?;

    // Print player details
    print_all_players(&players);
    println!();
    
    // Print statistical measures
    if let Some(mean_market_value) = statistics::mean(&players, &SortingCriteria::MarketValue) {
        println!("Mean Market Value: {}", mean_market_value);
    }
    if let Some(median_market_value) = statistics::median(&players, &SortingCriteria::MarketValue) {
        println!("Median Market Value: {}", median_market_value);
    }
    if let Some(mode_market_value) = statistics::mode(&players, &SortingCriteria::MarketValue) {
        println!("Mode Market Value: {}", mode_market_value);
    }
    if let Some(range_market_value) = statistics::range(&players, &SortingCriteria::MarketValue) {
        println!("Range Market Value: {}", range_market_value);
    }
    if let Some(variance_market_value) = statistics::variance(&players, &SortingCriteria::MarketValue) {
        println!("Variance Market Value: {}", variance_market_value);
    }
    if let Some(std_dev_market_value) = statistics::standard_deviation(&players, &SortingCriteria::MarketValue) {
        println!("Standard Deviation Market Value: {}", std_dev_market_value);
    }

    if let Some(mean_goals) = statistics::mean(&players, &SortingCriteria::Goals) {
        println!("Mean Goals: {}", mean_goals);
    }
    if let Some(median_goals) = statistics::median(&players, &SortingCriteria::Goals) {
        println!("Median Goals: {}", median_goals);
    }
    if let Some(mode_goals) = statistics::mode(&players, &SortingCriteria::Goals) {
        println!("Mode Goals: {}", mode_goals);
    }
    if let Some(range_goals) = statistics::range(&players, &SortingCriteria::Goals) {
        println!("Range Goals: {}", range_goals);
    }
    if let Some(variance_goals) = statistics::variance(&players, &SortingCriteria::Goals) {
        println!("Variance Goals: {}", variance_goals);
    }
    if let Some(std_dev_goals) = statistics::standard_deviation(&players, &SortingCriteria::Goals) {
        println!("Standard Deviation Goals: {}", std_dev_goals);
    }
    println!();

    // Plot scatter plot
    plot_scatter(&players)?;

    // Print top 10 most valuable players
    print_top_10(&players, SortingCriteria::MarketValue);
    println!();
    // Print top 10 players with most national team goals
    print_top_10(&players, SortingCriteria::Goals);
    println!();

    // linear regression training 
    let data = extract_goals_and_market_value("euro2024_players.csv");

    // Train linear regression model
    let learning_rate = 0.01;
    let n_iterations = 1000;
    let (m, b) = train_linear_regression(&data, learning_rate, n_iterations);
    println!("Final parameters: m = {}, b = {}", m, b);

    let goals = 5.5;
    let pred_mv = predict(goals, m, b);
    println!("Prediction: a player with {} goals has an expected market value of {:.2} €", goals, pred_mv);

    Ok(())
}
