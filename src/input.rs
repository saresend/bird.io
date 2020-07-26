// This will contain logic for reading in input
use crate::errors::DeviceNotFoundError;
use crate::frequency_modulation::NaiveFrequencyModulation;
use crate::traits::{BirdReceiver, Strategy};
use cpal::traits::{DeviceTrait, HostTrait};
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct BirdListener {}

impl BirdListener {
    pub fn default() -> Self {
        Self {}
    }

    fn create_input_handler<'a, T>(
        &self,
        sender: Sender<Vec<u8>>,
        mut decoder: Box<dyn FnMut(&[f64]) -> Vec<u8> + Send>,
    ) -> impl FnMut(&[T], &cpal::InputCallbackInfo)
    where
        T: cpal::Sample,
    {
        move |values, _| {
            let raw_values: Vec<f64> = values.iter().map(|x| x.to_f32() as f64).collect();
            let decoded = decoder(&raw_values);
            sender.send(decoded);
        }
    }

    fn create_error_handler(&self) -> impl Fn(cpal::StreamError) {
        |err| println!("{:?}", err)
    }

    fn create_stream_with_config<'a, S>(
        &self,
        device: cpal::Device,
        config: cpal::SupportedStreamConfig,
        sender: Sender<Vec<u8>>,
        strategy: S,
    ) -> Result<cpal::Stream, Box<dyn std::error::Error>>
    where
        S: Strategy + 'a,
    {
        let error_fn = self.create_error_handler();
        let decoding_fn = strategy.create_decoding();
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_input_stream(
                &config.config(),
                self.create_input_handler::<f32>(sender, decoding_fn),
                error_fn,
            )?,
            cpal::SampleFormat::I16 => device.build_input_stream(
                &config.config(),
                self.create_input_handler::<i16>(sender, decoding_fn),
                error_fn,
            )?,
            cpal::SampleFormat::U16 => device.build_input_stream(
                &config.config(),
                self.create_input_handler::<u16>(sender, decoding_fn),
                error_fn,
            )?,
        };
        Ok(stream)
    }

    fn setup_receiver<'a, T>(
        &self,
        strategy: T,
        sender: Sender<Vec<u8>>,
    ) -> Result<cpal::Stream, Box<dyn std::error::Error>>
    where
        T: Strategy + 'a,
    {
        let device = cpal::default_host()
            .default_input_device()
            .ok_or(DeviceNotFoundError)?;
        let config = device.default_input_config()?;
        let stream = self.create_stream_with_config(device, config, sender, strategy)?;
        Ok(stream)
    }

    fn open_receive(sender: cpal::Stream) {
        todo!()
    }
}
impl BirdReceiver<NaiveFrequencyModulation> for BirdListener {
    type BitFormat = Vec<u8>;
    fn start<'a>(
        &self,
        strategy: NaiveFrequencyModulation,
    ) -> Result<Receiver<Self::BitFormat>, Box<dyn std::error::Error>> {
        let (sender, receiver) = channel::<Self::BitFormat>();
        let play_stream = self.setup_receiver(strategy, sender)?;

        todo!()
    }

    fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
