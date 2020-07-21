// Contains the trait for strategy Logic
//
use crate::instrumentation;
use dasp::{signal, Signal};

pub trait Strategy {
    fn encode_bits<T: cpal::Sample>(&self, data: &[u8]) -> Vec<T>;
    fn decode_bit<T: cpal::Sample>(&mut self, data: &[T]) -> u8;
}

pub struct NaiveFrequencyModulation {}

impl NaiveFrequencyModulation {
    pub fn default() -> Self {
        Self {}
    }

    fn pad_vec(mut original: Vec<f32>) -> Vec<f32> {
        let length = original.len();
        let mut containing_size = 1;
        while containing_size < length {
            containing_size *= 2;
        }
        let remainder = containing_size - length;
        for _ in 0..remainder {
            original.push(0.0);
        }
        original
    }

    fn convert_to_bit(sample: Vec<f32>) -> u8 {
        instrumentation::save_data(&sample, 0);
        0
    }
}

impl Strategy for NaiveFrequencyModulation {
    fn encode_bits<T: cpal::Sample>(&self, data: &[u8]) -> Vec<T> {
        let threshold = 1_000;
        let mut low_signal = signal::rate(44100.0).const_hz(5000.0).sine();
        let mut high_signal = signal::rate(44100.0).const_hz(10000.0).sine();
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
        let encoded_data: Vec<f32> = data.iter().map(|x| x.to_f32()).collect();
        // we then need to pad encoded_data to the nearest power of 2
        NaiveFrequencyModulation::convert_to_bit(encoded_data)
    }
}
