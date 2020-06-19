// Contains the trait for strategy Logic
//
use dasp::{signal, Signal};
use rustfft::{num_complex::Complex, num_traits::Zero, FFTplanner};

pub trait Strategy {
    fn encode_bits<T: cpal::Sample>(&self, data: &[u8]) -> Vec<T>;
    fn decode_bits<T: cpal::Sample>(&mut self, data: &[T]) -> Vec<u8>;
}

pub struct NaiveFrequencyModulation {
    planner: FFTplanner<f32>,
}

impl NaiveFrequencyModulation {
    pub fn default() -> Self {
        Self {
            planner: FFTplanner::new(false),
        }
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

    fn decode_bits<T: cpal::Sample>(&mut self, data: &[T]) -> Vec<u8> {
        let mut encoded_data: Vec<f32> = data.iter().map(|x| x.to_f32()).collect();
        let mut output: Vec<Complex<f32>> = vec![Zero::zero(); encoded_data.len()];
        let fft = self.planner.plan_fft(encoded_data.len());
        fft.process(&mut encoded_data, &mut output);
        todo!()
    }
}
