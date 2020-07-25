use crate::traits::Strategy;
use dasp::{signal, Signal};
use yin::Yin;

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
    fn create_encoding(&self) -> Box<dyn FnMut(&[u8]) -> Vec<f64> + Send> {
        let mut low_signal = signal::rate(44100.0)
            .const_hz(self.low_bit_frequency)
            .sine();

        let mut high_signal = signal::rate(44100.0)
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
        Box::new(|data| vec![])
    }

    /* this is just here for reference
    fn encode_bits<T: cpal::Sample>(&self, data: &[u8]) -> Vec<T> {
        let mut low_signal = signal::rate(44100.0)
            .const_hz(self.low_bit_frequency)
            .sine();
        let mut high_signal = signal::rate(44100.0)
            .const_hz(self.high_bit_frequency)
            .sine();
        let mut result_vec: Vec<T> = vec![];
        for val in data {
            if val != &0 {
                for _ in 0..self.sample_count {
                    result_vec.push(cpal::Sample::from(&(high_signal.next() as f32)));
                }
            } else {
                for _ in 0..self.sample_count {
                    result_vec.push(cpal::Sample::from(&(low_signal.next() as f32)));
                }
            }
        }
        return result_vec;
    }

    fn decode_bits<T: cpal::Sample>(&mut self, data: &[T]) -> Vec<u8> {
        data.iter()
            .map(|x| x.to_f32() as f64)
            .collect::<Vec<f64>>()
            .chunks(self.sample_count)
            .map(|x| self.estimator.estimate_freq(x))
            .map(|x| self.convert_to_bit(x))
            .collect()
    }
    */
}
