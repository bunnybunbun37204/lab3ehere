use plotters::prelude::*;
use std::time::Duration;

pub struct DurationResult {
    pub name: String,
    pub duration: Duration,
}

pub fn plot_durations(results: &[DurationResult]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("0.png", (1024, 512)).into_drawing_area();
    root.fill(&WHITE)?;

    // Define colors for each method
    let colors = vec![BLUE, RED, GREEN, CYAN, MAGENTA];

    // Convert durations to f64 for plotting
    let max_duration = results
        .iter()
        .map(|result| result.duration.as_secs_f64())
        .fold(f64::MIN, |acc, x| acc.max(x));

    let mut chart = ChartBuilder::on(&root)
        .caption("Method Execution Times", ("sans-serif", 30).into_font())
        .margin(40)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .build_cartesian_2d(0.0..results.len() as f64, 0.0..max_duration)?;

    chart
        .configure_mesh()
        .x_desc("Method")
        .y_desc("Time (seconds)")
        .x_labels(results.len())
        .x_label_formatter(&|x| {
            let index = *x as usize;
            if index < results.len() {
                results[index].name.clone()
            } else {
                "".to_string()
            }
        })
        .draw()?;

    // Draw the bar series with different colors
    for (i, result) in results.iter().enumerate() {
        let duration = result.duration.as_secs_f64();
        let x = i as f64;
        let color = colors[i % colors.len()]; // Cycle through colors if there are more methods than colors

        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(x - 0.4, 0.0), (x + 0.4, duration)],
                color.filled(),
            )))?
            .label(&result.name)
            .legend(move |(x, y)| {
                Rectangle::new([(x - 10, y - 10), (x + 10, y + 10)], color.filled())
            });

        // Draw text labels on top of bars
        chart.draw_series(std::iter::once(Text::new(
            format!("{:.2} s", duration),
            (x, duration + 0.02), // Position slightly above the bar
            ("sans-serif", 15).into_font().color(&BLACK),
        )))?;
    }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
