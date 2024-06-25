use std::error::Error;
use std::fs::File;
use csv::Reader;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    // Use Serde's rename attribute to map the original CSV column names to the new struct field names.
    // This allows your code to read the CSV file correctly while avoiding naming convention warnings.
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "Age")]
    age: String,
}

fn read_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut rdr = Reader::from_reader(file);

    // Deserialize each record in the CSV file into a Record struct and print it.
    for result in rdr.deserialize() {
        // `deserialize` method reads each row of the CSV file, converts it from its CSV to instance of the Record struct
        // by the field types and serde attributes.
        
        let record: Record = result?; // `result?` extracts the Record if deserialization was successful, or returns an error.
        println!("{:?}", record); // Print the deserialized Record struct.
    }

    Ok(())
}

fn main() {
    // Call the read_csv function with the path to the CSV file.
    // Print an error message if the file cannot be read.
    if let Err(err) = read_csv("euro2024_players.csv") {
        println!("Error reading CSV: {}", err);
    }
}
