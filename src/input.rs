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

    fn create_input_handler<'a, T>(&self) -> impl FnMut(&[T], &cpal::InputCallbackInfo)
    where
        T: cpal::Sample,
    {
        |_values, _thing| {}
    }

    fn create_error_handler(&self) -> impl Fn(cpal::StreamError) {
        |err| println!("{:?}", err)
    }

    fn create_stream_with_config<'a, S>(
        &self,
        device: cpal::Device,
        config: cpal::SupportedStreamConfig,
        _strategy: S,
    ) -> Result<cpal::Stream, Box<dyn std::error::Error>>
    where
        S: Strategy + 'a,
    {
        let error_fn = self.create_error_handler();
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_input_stream(
                &config.config(),
                self.create_input_handler::<f32>(),
                error_fn,
            )?,
            cpal::SampleFormat::I16 => device.build_input_stream(
                &config.config(),
                self.create_input_handler::<i16>(),
                error_fn,
            )?,
            cpal::SampleFormat::U16 => device.build_input_stream(
                &config.config(),
                self.create_input_handler::<u16>(),
                error_fn,
            )?,
        };
        Ok(stream)
    }

    fn setup_receiver<'a, T>(&self, strategy: T) -> Result<cpal::Stream, Box<dyn std::error::Error>>
    where
        T: Strategy + 'a,
    {
        let device = cpal::default_host()
            .default_input_device()
            .ok_or(DeviceNotFoundError)?;
        let config = device.default_input_config()?;
        let stream = self.create_stream_with_config(device, config, strategy)?;
        Ok(stream)
    }

    fn open_receive<T>(sender: Sender<T>) {
        todo!()
    }
}
use std::thread;
impl BirdReceiver<NaiveFrequencyModulation> for BirdListener {
    type BitFormat = u8;
    fn start<'a>(
        &self,
        strategy: NaiveFrequencyModulation,
    ) -> Result<Receiver<Self::BitFormat>, Box<dyn std::error::Error>> {
        let (sender, receiver) = channel::<Self::BitFormat>();
        let play_stream = self.setup_receiver(strategy);
        let handler = thread::spawn(move || {
            Self::open_receive(sender);
        });

        todo!()
    }

    fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
