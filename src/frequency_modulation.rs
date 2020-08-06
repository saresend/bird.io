use crate::traits::Strategy;
use dasp::{signal, Signal};
use yin::Yin;

#[derive(Clone)]
pub struct NaiveFrequencyModulation {
    estimator: Yin,
    low_bit_frequency: f64,
    high_bit_frequency: f64,
    sample_count: usize,
}

impl NaiveFrequencyModulation {
    pub fn default() -> Self {
        Self {
            estimator: Yin::init(0.1, 3000.0, 10000.0, 44100),
            low_bit_frequency: 4410.0,
            high_bit_frequency: 8820.0,
            sample_count: 1000,
        }
    }

    pub fn convert_to_bit(&self, frequency: f64) -> u8 {
        println!("{}", frequency);
        let ld = frequency - self.low_bit_frequency;
        let hd = frequency - self.high_bit_frequency;
        let ld = ld.abs();
        let hd = hd.abs();
        if ld < hd {
            0
        } else {
            1
        }
    }
}

impl Strategy for NaiveFrequencyModulation {
    fn create_encoding(&self, sample_rate: u32) -> Box<dyn FnMut(&[u8]) -> Vec<f64> + Send> {
        let mut low_signal = signal::rate(sample_rate as f64)
            .const_hz(self.low_bit_frequency)
            .sine();

        let mut high_signal = signal::rate(sample_rate as f64)
            .const_hz(self.high_bit_frequency)
            .sine();
        let sample_count = self.sample_count;
        Box::new(move |data| {
            let mut result_vec = vec![];
            for val in data {
                if val != &0 {
                    for _ in 0..sample_count {
                        result_vec.push(high_signal.next());
                    }
                } else {
                    for _ in 0..sample_count {
                        result_vec.push(low_signal.next());
                    }
                }
            }

            result_vec
        })
    }

    fn create_decoding(&self) -> Box<dyn FnMut(&[f64]) -> Vec<u8> + Send> {
        let new_freq_mod = self.clone();
        Box::new(move |data| {
            data.chunks(new_freq_mod.sample_count)
                .map(|x| new_freq_mod.estimator.estimate_freq(x))
                .map(|x| new_freq_mod.convert_to_bit(x))
                .collect()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[allow(non_snake_case)]
    fn NaiveFrequencyModulation_simple_case() {
        let test_bits = vec![0, 1, 0, 1, 0];
        let strat = NaiveFrequencyModulation::default();
        let mut encoded_value_func = strat.create_encoding(44100);
        let encoded_bits = encoded_value_func(&test_bits);
        let mut decode_fn = strat.create_decoding();
        let decoded_bits = decode_fn(&encoded_bits);
        assert_eq!(test_bits, decoded_bits);
    }
}
