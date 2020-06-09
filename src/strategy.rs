// Contains the trait for strategy Logic
//

pub trait Strategy {
    fn encode_bits<T: cpal::Sample>(&self, data: &[u8]) -> Vec<T>;
    fn decode_bits<T: cpal::Sample>(&self, data: &[T]) -> Vec<u8>;
}
