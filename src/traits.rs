use std::sync::mpsc::Receiver;
/// This handles the strategy interface that will be used to handle all bit encoding and decoding
pub trait Strategy {
    fn create_decoding(&self) -> Box<dyn FnMut(&[f64]) -> Vec<u8> + Send>;
    fn create_encoding(&self, u32) -> Box<dyn FnMut(&[u8]) -> Vec<f64> + Send>;
}
/// This should handle all methods associated with a given implementation of a receiver
/// Here the type parameter
pub trait BirdReceiver<T>
where
    T: Strategy,
{
    type BitFormat;
    fn start(
        &mut self,
        strategy: T,
    ) -> Result<Receiver<Self::BitFormat>, Box<dyn std::error::Error>>;
    fn close(self) -> Result<(), Box<dyn std::error::Error>>;
}

/// The BirdSender trait provides all the methods associated with a given
/// implementation of a sender type
pub trait BirdSender<T>
where
    T: Strategy,
{
    fn transmit(&self, strategy: T, data: &[u8]) -> Result<(), Box<dyn std::error::Error>>;
}
