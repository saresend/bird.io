// Contains the trait for strategy Logic
//
use crate::instrumentation;
use dasp::{signal, Signal};
use yin::Yin;

pub trait Strategy {
    fn encode_bits<T: cpal::Sample>(&self, data: &[u8]) -> Vec<T>;
    fn decode_bit<T: cpal::Sample>(&mut self, data: &[T]) -> u8;
}

pub struct NaiveFrequencyModulation {
    estimator: Yin,
}

impl NaiveFrequencyModulation {
    pub fn default() -> Self {
        Self {
            estimator: Yin::init(0.1, 3000.0, 10000.0, 44100),
        }
    }

    pub fn convert_to_bit(frequency: f64) -> u8 {
        return 0;
    }
}

impl Strategy for NaiveFrequencyModulation {
    fn encode_bits<T: cpal::Sample>(&self, data: &[u8]) -> Vec<T> {
        let threshold = 1_000;
        let mut low_signal = signal::rate(44100.0).const_hz(4410.0).sine();
        let mut high_signal = signal::rate(44100.0).const_hz(8820.0).sine();
        let mut result_vec: Vec<T> = vec![];
        for val in data {
            if val != &0 {
                for _ in 0..threshold {
                    result_vec.push(cpal::Sample::from(&(high_signal.next() as f32)));
                }
            } else {
                for _ in 0..threshold {
                    result_vec.push(cpal::Sample::from(&(low_signal.next() as f32)));
                }
            }
        }
        return result_vec;
    }

    fn decode_bit<T: cpal::Sample>(&mut self, data: &[T]) -> u8 {
        let encoded_data: Vec<f64> = data.iter().map(|x| x.to_f32() as f64).collect();
        let frequency = self.estimator.estimate_freq(&encoded_data);
        // we then need to pad encoded_data to the nearest power of 2
        // we need some error handling here
        NaiveFrequencyModulation::convert_to_bit(frequency)
    }
}
