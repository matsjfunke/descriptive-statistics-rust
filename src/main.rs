use std::error::Error;
use std::fs::File;
use csv::Reader;
use serde_derive::Deserialize;
use plotters::prelude::*;

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "MarketValue")]
    market_value: i32, // Assuming market value is an integer
    #[serde(rename = "Goals")]
    goals: i32, // Assuming goals is an integer
}

fn read_csv(file_path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = Reader::from_reader(file);

    let mut data = Vec::new();

    for result in rdr.deserialize() {
        let record: Record = result?;
        // Filter out goalkeepers
        if record.position != "Goalkeeper" {
            data.push(record);
        }
    }

    Ok(data)
}

// Function to print player details
fn print_players(records: &[Record]) {
    for record in records {
        println!("Name: {}, Market Value: {}, Goals: {}", record.name, record.market_value, record.goals);
    }
}

// Function to plot scatter plot
fn plot_scatter(records: &[Record]) -> Result<(), Box<dyn Error>> {
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
        records.iter().map(|record| Circle::new((record.goals, record.market_value), 3, BLUE.filled())),
    )?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read data from CSV
    let records = read_csv("euro2024_players.csv")?;

    // Print player details
    print_players(&records);

    // Plot scatter plot
    plot_scatter(&records)?;

    Ok(())
}
