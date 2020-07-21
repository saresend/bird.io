fn diff_function(audio_sample: &[f64], tau_max: usize) -> Vec<f64> {
    todo!()
}

// should return a tau that gives the # of elements of offset in a given sample
fn compute_sample_frequency(audio_sample: Vec<f64>) -> f64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::compute_sample_frequency;
    use dasp::{signal, Signal};
    #[test]
    fn test_sine_frequency() {
        let sample_rate = 1000;
        let mut sine_sample = signal::rate(sample_rate as f64).const_hz(3000.0).sine();
        let sample_frame = (0..sample_rate).map(|_| sine_sample.next()).collect();
        let result = compute_sample_frequency(sample_frame);
        assert_eq!(result, 3000.0);
    }

    #[test]
    fn test_small_sine() {
        let sample_rate = 14;
        let mut sine_sample = signal::rate(sample_rate as f64).const_hz(5.0).sine();
        let sample_frame = (0..sample_rate).map(|_| sine_sample.next()).collect();
        println!("{:?}", sample_frame);
        let result = compute_sample_frequency(sample_frame);
        assert_eq!(result, 4.0);
    }
}
