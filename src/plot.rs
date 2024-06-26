use plotters::prelude::*;
use std::error::Error;

use crate::models::{Player};

// Function to plot scatter plot
pub fn plot_scatter(players : &[Player]) -> Result<(), Box<dyn Error>> {
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
