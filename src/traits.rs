use std::sync::mpsc::Receiver;
/// This handles the strategy interface that will be used to handle all bit encoding and decoding
pub trait Strategy {
    fn encode_bits<T: cpal::Sample>(&self, data: &[u8]) -> Vec<T>;
    fn decode_bits<T: cpal::Sample>(&mut self, data: &[T]) -> Vec<u8>;
}
/// This should handle all methods associated with a given implementation of a receiver
/// Here the type parameter
pub trait BirdReceiver<T>
where
    T: Strategy,
{
    type BitFormat;
    fn start(&self, strategy: T) -> Result<Receiver<Self::BitFormat>, Box<dyn std::error::Error>>;
    fn close(&self) -> Result<(), Box<dyn std::error::Error>>;
}

/// The BirdSender trait provides all the methods associated with a given
/// implementation of a sender type
pub trait BirdSender<T>
where
    T: Strategy,
{
    fn transmit(&self, strategy: T, data: &[u8]) -> Result<(), Box<dyn std::error::Error>>;
}
