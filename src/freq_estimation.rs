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

fn cmndf(raw_diff: &[f64]) -> Vec<f64> {
    use crate::instrumentation::*;
    let mut cmndf_diff: Vec<f64> = raw_diff[1..]
        .iter()
        .enumerate()
        .scan(0.0, |state, x| {
            *state = *state + x.1;
            let result = *x.1 * (x.0 + 1) as f64 / *state;
            Some(result)
        })
        .collect();
    let thing: Vec<f32> = cmndf_diff.clone().iter().map(|x| *x as f32).collect();
    cmndf_diff.insert(0, 1.0);
    println!("{:?}", &raw_diff[1..]);
    println!("{:?}", cmndf_diff);
    save_data(&thing, 2);
    cmndf_diff
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
    let tau_max = audio_sample.len() * 25 / 1000;
    let tau_max = 50;
    let diff_fn = diff_function(&audio_sample, tau_max);
    let cmndf = cmndf(&diff_fn);
    let sample_period = compute_diff_min(&cmndf, tau_max, 0.1);
    convert_to_frequency(sample_period, audio_sample.len())
}

#[cfg(test)]
mod tests {
    use super::super::instrumentation::*;
    use super::{compute_diff_min, compute_sample_frequency, convert_to_frequency, diff_function};
    use dasp::{signal, Signal};

    #[test]
    fn visualize_cmndf() {
        let sample_rate = 120;
        let mut sine_sample = signal::rate(sample_rate as f64).const_hz(3.0).sine();
        let sample_frame: Vec<f64> = (0..sample_rate).map(|_| sine_sample.next()).collect();
        let result = compute_sample_frequency(sample_frame);
        let cmndf_data = get_data(2);
        visualize_pcm(&cmndf_data, "cmndf_output.png");
        assert_eq!(result, 3.0);
    }

    #[test]
    fn diff_function_basic_sine() {
        let sample_rate = 12;
        let mut sine_sample = signal::rate(sample_rate as f64).const_hz(3.0).sine();
        let sample_frame: Vec<f64> = (0..sample_rate).map(|_| sine_sample.next()).collect();
        let result = diff_function(&sample_frame, 6);
        assert_eq!(result, vec![0.0, 6.0, 12.0, 6.0, 0.0, 6.0]);
    }

    #[test]
    fn conv_noisey_sr_12_f_3() {
        use rand::Rng;
        let sample_rate = 120;
        let mut sine_sample = signal::rate(sample_rate as f64).const_hz(3.0).sine();
        let mut rng = rand::thread_rng();
        let sample_frame: Vec<f64> = (0..sample_rate)
            .map(|_| sine_sample.next() + rng.gen::<f64>())
            .collect();
        let result = compute_sample_frequency(sample_frame);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn conv_noisey_sine() {
        use rand::Rng;
        let sample_rate = 44100;
        let mut sine_sample = signal::rate(sample_rate as f64).const_hz(441.0).sine();
        let mut rng = rand::thread_rng();
        let sample_frame: Vec<f64> = (0..sample_rate)
            .map(|_| sine_sample.next() + rng.gen::<f64>() * 0.005)
            .collect();
        let result = compute_sample_frequency(sample_frame);
        assert_eq!(result, 441.0);
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
    fn test_full_freq() {
        let sample_rate = 44100;
        let mut sine_sample = signal::rate(sample_rate as f64).const_hz(8820.0).sine();
        let sample_frame: Vec<f64> = (0..sample_rate).map(|_| sine_sample.next()).collect();
        let freq = compute_sample_frequency(sample_frame);
        assert_eq!(freq, 8820.0);
    }

    #[test]
    fn test_sine_frequency() {
        let sample_rate = 3000;
        let mut sine_sample = signal::rate(sample_rate as f64).const_hz(300.0).sine();
        let sample_frame = (0..sample_rate).map(|_| sine_sample.next()).collect();
        let result = compute_sample_frequency(sample_frame);
        assert_eq!(result, 300.0);
    }
}
