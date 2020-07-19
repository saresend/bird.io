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
// should return a tau that gives the # of elements of offset in a given sample
fn simple_diff_function(audio_sample: Vec<f64>) -> f64 {
    let max_tau = audio_sample.len() * 25 / 1000;
    let mut difference_function = vec![0.0; max_tau];
    for tau in 1..max_tau {
        for j in 0..(audio_sample.len() - tau - 1) {
            let difference = audio_sample[j] - audio_sample[j + tau];
            difference_function[tau] += difference * difference;
        }
    }
    // Here, we want to look for the smallest values in our difference_function
    let mut diff_fn_it = difference_function.iter();
    diff_fn_it.next();
    let smallest = diff_fn_it
        .enumerate()
        .min_by(|&(_, item), &(_, item2)| item.partial_cmp(item2).unwrap());
    let tau_value = smallest.unwrap().0 + 1;
    let period = tau_value as f64 / audio_sample.len() as f64;
    1.0 / period
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
    use super::simple_diff_function;
    use dasp::{signal, Signal};
    #[test]
    fn test_sine_frequency() {
        let sample_rate = 1000;
        let mut sine_sample = signal::rate(sample_rate as f64).const_hz(3000.0).sine();
        let sample_frame = (0..sample_rate).map(|_| sine_sample.next()).collect();
        let result = simple_diff_function(sample_frame);
        assert_eq!(result, 3000.0);
    }

    #[test]
    fn test_small_sine() {
        let sample_rate = 16;
        let mut sine_sample = signal::rate(sample_rate as f64).const_hz(5.0).sine();
        let sample_frame = (0..sample_rate).map(|_| sine_sample.next()).collect();
        println!("{:?}", sample_frame);
        let result = simple_diff_function(sample_frame);
        assert_eq!(result, 4.0);
    }
}
