// linear function -> y = m * x + b
use csv::Reader;
use crate::models::Player;

pub fn prep_csv(file_path: &str) -> Result<Vec<Player>, Box<dyn std::error::Error>> {
    let mut rdr = Reader::from_path(file_path)?;
    let mut players = Vec::new();
    for result in rdr.deserialize() {
        let player: Player = result?;
        // Filter out goalkeepers, players with over 50 goals, and market_value over 100,000,000
        if player.position != "Goalkeeper" && player.goals <= 50 && player.market_value <= 100_000_000 {
            players.push(player);
        }
    }
    Ok(players)
}

pub fn extract_goals_and_market_value(file_path: &str) -> Vec<(i32, i32)> {
    let players: Vec<Player> = prep_csv(file_path).expect("Failed to read players.csv");

    let mut extracted_data = Vec::new();

    for player in &players {
        let goals = player.goals;
        let market_value = player.market_value;
        extracted_data.push((goals, market_value));
    }

    extracted_data
}

pub fn train_linear_regression(data: &[(i32, i32)], learning_rate: f64, n_iterations: usize) -> (f64, f64) {
    let x: Vec<f64> = data.iter().map(|&(goals, _)| goals as f64).collect();
    let y: Vec<f64> = data.iter().map(|&(_, market_value)| market_value as f64).collect();

    // Initialize parameters
    let mut m = 0.0;
    let mut b = 0.0;

    // Number of samples
    let n = x.len() as f64;

    // Gradient descent algorithm
    for _ in 0..n_iterations {
        let mut m_gradient = 0.0;
        let mut b_gradient = 0.0;

        for i in 0..x.len() {
            let prediction = m * x[i] + b;
            m_gradient += -2.0 * x[i] * (y[i] - prediction);
            b_gradient += -2.0 * (y[i] - prediction);
        }

        m -= (m_gradient / n) * learning_rate;
        b -= (b_gradient / n) * learning_rate;
    }

    (m, b)
}

pub fn predict(x: f64, m: f64, b: f64) -> f64 {
    m * x + b
}
