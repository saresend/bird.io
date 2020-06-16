// Contains the trait for strategy Logic
//
use dasp::{signal, Signal};
use dft::{Operation, Plan};

pub trait Strategy {
    fn encode_bits<T: cpal::Sample>(&self, data: &[u8]) -> Vec<T>;
    fn decode_bits<T: cpal::Sample>(&self, data: &[T]) -> Vec<u8>;
}

pub struct NaiveFrequencyModulation {}

fn decode_chunk(mut value: Vec<f32>) -> u8 {
    let plan = Plan::new(Operation::Forward, value.len());
    dft::transform(&mut value, &plan);
    // Now in theory we have a frequency distribution for the given window

    return 0;
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

    fn decode_bits<T: cpal::Sample>(&self, data: &[T]) -> Vec<u8> {
        let encoded_data: Vec<f32> = data.iter().map(|x| x.to_f32()).collect();
        let threshold = 1_000;
        let owned_data: Vec<Vec<f32>> =
            encoded_data.chunks(threshold).map(|x| x.to_vec()).collect();
        owned_data.into_iter().map(|x| decode_chunk(x)).collect()
    }
}
