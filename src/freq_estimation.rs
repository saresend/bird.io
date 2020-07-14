/*
* use ndarray::prelude::*;
* use ndarray::Array;
*/

fn diff_function(audio_sample: &[u8], tau_max: usize) -> Vec<u64> {
    let mut diff_vec = vec![0; tau_max];
    for tau in 1..tau_max {
        for j in 0..(audio_sample.len() - tau_max) {
            let tmp = audio_sample[j] - audio_sample[j + tau];
            diff_vec[tau] += tmp as u64 * tmp as u64;
        }
    }
    diff_vec
}

fn cum_mean_norm_diff_fn(df: Vec<f64>) -> Vec<f64> {
    let mut iter1 = df.iter().clone();
    iter1.next();
    let total: f64 = iter1.sum();
    vec![1.0].into_iter().chain(df.into_iter()).collect()
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

fn compute_yin(signal: Vec<u8>, sr: usize, freq_min: usize, freq_max: usize) -> Vec<f64> {
    let tau_min = sr / freq_max;
    let tau_max = sr / freq_min;

    todo!()
}
