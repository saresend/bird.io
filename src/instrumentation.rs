use lazy_static::lazy_static;
use plotters::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref DATASTORE: Mutex<HashMap<usize, Vec<f32>>> = Mutex::new(HashMap::new());
}

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
        .build_ranged(0f32..data.len() as f32, -1f32..1f32)?;
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

pub fn save_data<T>(data: &[T], id: usize)
where
    T: cpal::Sample,
{
    let mut datastore_ref = DATASTORE.lock().unwrap();
    let conv_data: Vec<f32> = data.iter().map(|x| x.to_f32()).collect();
    datastore_ref.insert(id, conv_data);
}

pub fn get_data(id: usize) -> Vec<f32> {
    let datastore_ref = DATASTORE.lock().unwrap();
    match datastore_ref.get(&id) {
        Some(element) => element.clone(),
        None => vec![],
    }
}
