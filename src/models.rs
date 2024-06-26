// models.rs defines the data structures (models) used throughout your application. 
// These models represent the core entities that your program manipulates.
use serde_derive::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Player {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Position")]
    pub position: String,
    #[serde(rename = "MarketValue")]
    pub market_value: i32,
    #[serde(rename = "Goals")]
    pub goals: i32,
}

pub enum SortingCriteria {
    MarketValue,
    Goals,
}
