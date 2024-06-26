use crate::models::{Player, SortingCriteria}; // Import types from models
use std::collections::HashMap;

// Helper function to extract the relevant data based on the SortingCriteria
pub fn extract_data(players: &[Player], criteria: &SortingCriteria) -> Vec<i32> {
    match criteria {
        SortingCriteria::MarketValue => players.iter().map(|p| p.market_value).collect(),
        SortingCriteria::Goals => players.iter().map(|p| p.goals).collect(),
    }
}

// Measures of central tendency
pub fn mean(players: &[Player], criteria: &SortingCriteria) -> Option<f64> {
    let data = extract_data(players, criteria);
    if data.is_empty() {
        return None; // Avoid division by zero
    }
    let sum: i64 = data.iter().map(|&v| v as i64).sum(); // Convert to i64 to avoid overflow
    Some(sum as f64 / data.len() as f64)
}

pub fn median(players: &[Player], criteria: &SortingCriteria) -> Option<f64> {
    let mut data = extract_data(players, criteria);
    if data.is_empty() {
        return None;
    }
    data.sort();
    let len = data.len();
    if len % 2 == 0 {
        Some((data[len / 2 - 1] as f64 + data[len / 2] as f64) / 2.0)
    } else {
        Some(data[len / 2] as f64)
    }
}

pub fn mode(players: &[Player], criteria: &SortingCriteria) -> Option<i32> {
    let data = extract_data(players, criteria);
    if data.is_empty() {
        return None;
    }
    let mut occurrences = HashMap::new();
    for &value in &data {
        *occurrences.entry(value).or_insert(0) += 1;
    }
    occurrences.into_iter().max_by_key(|&(_, count)| count).map(|(val, _)| val)
}

// Measures of variability
pub fn range(players: &[Player], criteria: &SortingCriteria) -> Option<i32> {
    let data = extract_data(players, criteria);
    if data.is_empty() {
        return None;
    }
    let max = *data.iter().max().unwrap();
    let min = *data.iter().min().unwrap();
    Some(max - min)
}

pub fn variance(players: &[Player], criteria: &SortingCriteria) -> Option<f64> {
    let data = extract_data(players, criteria);
    if data.is_empty() {
        return None;
    }
    let mean = mean(players, criteria)?;
    let sum_of_squared_diffs: f64 = data.iter().map(|&value| {
        let diff = value as f64 - mean;
        diff * diff
    }).sum();
    Some(sum_of_squared_diffs / data.len() as f64)
}

pub fn standard_deviation(players: &[Player], criteria: &SortingCriteria) -> Option<f64> {
    variance(players, criteria).map(|var| var.sqrt())
}
