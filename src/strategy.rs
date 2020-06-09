// Contains the trait for strategy Logic
//
use dasp::{signal, Signal};

pub trait Strategy {
    fn encode_bits<T: cpal::Sample>(&self, data: &[u8]) -> Vec<T>;
    fn decode_bits<T: cpal::Sample>(&self, data: &[T]) -> Vec<u8>;
}

pub struct NaiveFrequencyModulation {}

impl Strategy for NaiveFrequencyModulation {
    fn encode_bits<T: cpal::Sample>(&self, data: &[u8]) -> Vec<T> {
        let threshold = 1_000; // Take 20,000 samples per bit
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
    fn decode_bits<T: cpal::Sample>(&self, data: &[T]) -> Vec<u8> {
        todo!()
    }
}
