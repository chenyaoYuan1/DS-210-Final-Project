// Here is the function that graph the distribution in the separate png file. 


use plotters::prelude::*;
use std::collections::{HashMap};

pub fn plot_distribution(distribution: &HashMap<u32, u32>, output_filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(output_filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_length = *distribution.keys().max().unwrap_or(&1);
    let max_count = *distribution.values().max().unwrap_or(&1);

    let mut chart = ChartBuilder::on(&root)
    .caption("Distribution of Shortest Path Lengths", ("Arial", 24).into_font())
    .margin(5)
    .x_label_area_size(30)
    .y_label_area_size(40)
    .build_cartesian_2d(0u32..max_length, 0u32..max_count)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(PointSeries::of_element(
        distribution
            .iter()
            .map(|(&length, &count)| (length, count))
            .map(|(length, count)| (length, count)),
        5,
        &RED,
        &|x, size, style| Circle::new(x, size, style),
    ))?;
    root.present()?;
    
    
    

    root.present()?;
    Ok(())
}