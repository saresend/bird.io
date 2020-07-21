fn diff_function(audio_sample: &[f64], tau_max: usize) -> Vec<f64> {
    let mut diff_function = vec![0.0; tau_max];
    for tau in 1..tau_max {
        for j in 0..(audio_sample.len() - tau_max) {
            let tmp = audio_sample[j] - audio_sample[j + tau];
            diff_function[tau] += tmp * tmp;
        }
    }
    diff_function
}

fn compute_diff_min(diff_fn: &[f64], max_tau: usize, harm_threshold: f64) -> usize {
    let mut tau = 1;
    while tau < max_tau {
        if diff_fn[tau] < harm_threshold {
            while tau + 1 < max_tau && diff_fn[tau + 1] < diff_fn[tau] {
                tau += 1;
            }
            return tau;
        }
        tau += 1;
    }
    0
}

fn convert_to_frequency(sample_period: usize, sample_rate: usize) -> f64 {
    let value: f64 = sample_period as f64 / sample_rate as f64;
    1.0 / value
}

// should return a tau that gives the # of elements of offset in a given sample
fn compute_sample_frequency(audio_sample: Vec<f64>) -> f64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::{compute_diff_min, compute_sample_frequency, convert_to_frequency, diff_function};
    use dasp::{signal, Signal};

    #[test]
    fn diff_function_basic_sine() {
        let sample_rate = 12;
        let mut sine_sample = signal::rate(sample_rate as f64).const_hz(3.0).sine();
        let sample_frame: Vec<f64> = (0..sample_rate).map(|_| sine_sample.next()).collect();
        let result = diff_function(&sample_frame, 6);
        assert_eq!(result, vec![0.0, 6.0, 12.0, 6.0, 0.0, 6.0]);
    }

    #[test]
    fn dm_sr_12_f_3() {
        let sample_rate = 12;
        let mut sine_sample = signal::rate(sample_rate as f64).const_hz(3.0).sine();
        let sample_frame: Vec<f64> = (0..sample_rate).map(|_| sine_sample.next()).collect();
        let diff_fn = diff_function(&sample_frame, 6);
        let result = compute_diff_min(&diff_fn, 6, 0.1);
        assert_eq!(result, 4);
    }

    #[test]
    fn dm_sr_15_f_3() {
        let sample_rate = 15;
        let mut sine_sample = signal::rate(sample_rate as f64).const_hz(3.0).sine();
        let sample_frame: Vec<f64> = (0..sample_rate).map(|_| sine_sample.next()).collect();
        let diff_fn = diff_function(&sample_frame, 6);
        let result = compute_diff_min(&diff_fn, 6, 0.1);
        assert_eq!(result, 5);
    }

    #[test]
    fn dm_sr_150_f_3() {
        let sample_rate = 150;
        let mut sine_sample = signal::rate(sample_rate as f64).const_hz(3.0).sine();
        let sample_frame: Vec<f64> = (0..sample_rate).map(|_| sine_sample.next()).collect();
        let diff_fn = diff_function(&sample_frame, 55);
        println!("{:?}", diff_fn);
        let result = compute_diff_min(&diff_fn, 55, 0.1);
        assert_eq!(result, 50);
    }

    #[test]
    fn conv_sr_150_f_5() {
        let sample_rate = 150;
        let mut sine_sample = signal::rate(sample_rate as f64).const_hz(5.0).sine();
        let sample_frame: Vec<f64> = (0..sample_rate).map(|_| sine_sample.next()).collect();
        let diff_fn = diff_function(&sample_frame, 55);
        println!("{:?}", diff_fn);
        let sample_period = compute_diff_min(&diff_fn, 55, 0.1);
        let freq = convert_to_frequency(sample_period, sample_rate);
        assert_eq!(freq, 5.0);
    }

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
