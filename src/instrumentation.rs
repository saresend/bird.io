use plotters::prelude::*;

pub fn visualize_pcm<T>(data: &[T], name: &str) -> Result<(), Box<dyn std::error::Error>>
where
    T: cpal::Sample,
{
    let root = BitMapBackend::new(name, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("PCM data", ("sans-serif", 40).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(0f32..44100f32, -1f32..1f32)?;
    chart.configure_mesh().draw()?;
    let data: Vec<(f32, f32)> = data
        .iter()
        .enumerate()
        .map(|x| (x.0 as f32, x.1.to_f32()))
        .collect();
    let freq_series = LineSeries::new(data, &RED);
    chart.draw_series(freq_series)?.label("PCM data");
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    Ok(())
}
