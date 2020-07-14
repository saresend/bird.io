fn diff_function(audio_sample: &[f64], tau_max: usize) -> Vec<f64> {
    let mut diff_vec = vec![0.0; tau_max];
    for tau in 1..tau_max {
        for j in 0..(audio_sample.len() - tau_max) {
            let tmp = audio_sample[j] - audio_sample[j + tau];
            diff_vec[tau] += tmp as f64 * tmp as f64;
        }
    }
    diff_vec
}

fn cum_mean_norm_diff_fn(df: Vec<f64>) -> Vec<f64> {
    let mut iter1 = df.iter().clone();
    iter1.next();
    let total: f64 = iter1.sum();
    vec![1.0]
        .into_iter()
        .chain(df.into_iter().map(|y| y / total))
        .collect()
}

fn get_pitch(cmdf: Vec<f64>, tau_min: usize, tau_max: usize) -> usize {
    let mut tau = tau_min;
    while tau < tau_max {
        if cmdf[tau] < 0.1 {
            while tau + 1 < tau_max && cmdf[tau + 1] < cmdf[tau] {
                tau += 1;
            }
            return tau;
        }
        tau += 1;
    }
    return 0;
}

pub fn compute_yin_for_frame(
    signal: Vec<f64>,
    sr: usize,
    freq_min: usize,
    freq_max: usize,
) -> usize {
    let tau_max = sr / freq_min;
    let tau_min = sr / freq_max;
    let df = diff_function(&signal, tau_max);
    let cmdf = cum_mean_norm_diff_fn(df);

    get_pitch(cmdf, tau_min, tau_max)
}

#[cfg(test)]
mod tests {
    use super::compute_yin_for_frame;
    use dasp::{signal, Signal};
    #[test]
    fn test_sine_frequency() {
        let sample_rate = 14400;
        let mut sine_sample = signal::rate(sample_rate as f64).const_hz(3000.0).sine();
        let sample_frame = (0..sample_rate).map(|_| sine_sample.next()).collect();
        let result = compute_yin_for_frame(sample_frame, sample_rate, 1000, 5000);
        assert_eq!(result, 3000);
    }
}
