use std::error::Error;
use std::fs::File;
use csv::Reader;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "MarketValue")]
    market_value: String,
    #[serde(rename = "Goals")]
    goals: String,
}

fn read_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = Reader::from_reader(file);

    // Deserialize each record in the CSV file into a Record struct and print it.
    for result in rdr.deserialize() {
        let record: Record = result?; // `result?` extracts the Record if deserialization was successful, or returns an error.
        
         // Filter out players who are goalkeepers
        if record.position != "Goalkeeper" {
            println!("Name: {}, Market Value: {}, Goals: {}", record.name, record.market_value, record.goals);
        }
    }

    Ok(())
}

fn main() {
    if let Err(err) = read_csv("euro2024_players.csv") {
        println!("Error reading CSV: {}", err);
    }
}
