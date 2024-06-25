use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define your data points
    let data = vec![(1.0, 1.5), (2.0, 2.5), (3.0, 3.5), (4.0, 4.5), (5.0, 5.5)];

    // Create a drawing area
    let root = BitMapBackend::new("scatter_plot.png", (600, 400)).into_drawing_area();
    root.fill(&WHITE)?;

    // Create a chart context
    let mut chart = ChartBuilder::on(&root)
        .caption("Scatter Plot", ("sans-serif", 30))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..6.0, 0.0..6.0)?;

    // Draw the scatter plot
    chart
        .configure_mesh()
        .draw()?;
    
    chart
        .draw_series(
            data.iter().map(|(x, y)| Circle::new((*x, *y), 5, BLUE.filled())),
        )?;

    Ok(())
}
