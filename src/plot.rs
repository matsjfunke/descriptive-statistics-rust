use plotters::prelude::*;
use std::error::Error;

use crate::models::{Player};

pub fn plot_all_scatter(players : &[Player]) -> Result<(), Box<dyn Error>> {
    let root_area = BitMapBackend::new("images/scatter_plot.png", (1000, 800)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Goals vs Market Value", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0..130,1_000_000..190_000_000)?;

    chart.configure_mesh().draw()?;

    // Draw the scatter plot
    chart.draw_series(
        players.iter().map(|player| Circle::new((player.goals, player.market_value), 3, RED.filled())),
    )?;

    Ok(())
}

// Function to plot scatter plot with linear regression line
pub fn plot_scatter_with_regression(players: &[Player], m: f64, b: f64) -> Result<(), Box<dyn Error>> {
    let root_area = BitMapBackend::new("images/lin_reg_scatter.png", (1200, 1000)).into_drawing_area();
    root_area.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root_area)
        .caption("Goals vs Market Value of Players < 50 Goals & < 100m Market Value", ("sans-serif", 30))
        .margin(40)
        .x_label_area_size(40)
        .y_label_area_size(70)
        .build_cartesian_2d(0..50, 1_000_000..100_000_000)?;

    chart.configure_mesh()
        .x_desc("Number of Goals")
        .y_desc("Market Value in Euros")
        .draw()?;

    // Draw the scatter plot
    chart.draw_series(
        players.iter().map(|player| Circle::new((player.goals, player.market_value), 3, RED.filled())),
    )?;

    // Draw the regression line
    let regression_line = LineSeries::new(
        (0..60).map(|x| (x, (m * x as f64 + b) as i32)),
        BLUE.stroke_width(3),
    );
    chart.draw_series(regression_line)?;

    // Add a caption for the regression line
    let text_style = TextStyle::from(("sans-serif", 20).into_font()).color(&BLACK);
    root_area.draw_text(
        &format!("Blue Regression: y = {:.2} * x + {:.2}", m, b),
        &text_style,
        ((root_area.dim_in_pixel().1 as f32 * 0.75) as i32, (root_area.dim_in_pixel().1 as f32 * 0.95) as i32),
    )?;

    Ok(())
}
