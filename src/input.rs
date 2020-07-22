// This will contain logic for reading in input
use crate::errors::DeviceNotFoundError;
use crate::frequency_modulation::NaiveFrequencyModulation;
use crate::traits::BirdReceiver;
use cpal::traits::{DeviceTrait, HostTrait};
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct BirdListener {}

impl BirdListener {
    pub fn default() -> Self {
        Self {}
    }

    fn create_stream_with_config(
        &self,
        config: cpal::SupportedStreamConfig,
    ) -> Result<cpal::Stream, Box<dyn std::error::Error>> {
        todo!()
    }

    fn setup_receiver(&self) -> Result<cpal::Stream, Box<dyn std::error::Error>> {
        let device = cpal::default_host()
            .default_input_device()
            .ok_or(DeviceNotFoundError)?;
        let config = device.default_input_config()?;
        let stream = self.create_stream_with_config(config)?;
        Ok(stream)
    }

    fn open_receive<T>(sender: Sender<T>) {
        todo!()
    }
}
use std::thread;
impl BirdReceiver<NaiveFrequencyModulation> for BirdListener {
    type BitFormat = u8;
    fn start(
        &self,
        strategy: NaiveFrequencyModulation,
    ) -> Result<Receiver<Self::BitFormat>, Box<dyn std::error::Error>> {
        let (sender, receiver) = channel::<Self::BitFormat>();
        let values = self.setup_receiver();
        let handler = thread::spawn(move || {
            Self::open_receive(sender);
        });

        todo!()
    }

    fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
