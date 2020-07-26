// This will contain logic for reading in input
use crate::errors::{DeviceNotFoundError, StreamCloseError};
use crate::frequency_modulation::NaiveFrequencyModulation;
use crate::traits::{BirdReceiver, Strategy};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct BirdListener {
    kill_sender: Option<Sender<bool>>,
}

impl BirdListener {
    pub fn default() -> Self {
        Self { kill_sender: None }
    }
}

impl BirdReceiver<NaiveFrequencyModulation> for BirdListener {
    type BitFormat = Vec<u8>;
    fn start(
        &mut self,
        strategy: NaiveFrequencyModulation,
    ) -> Result<Receiver<Self::BitFormat>, Box<dyn std::error::Error>> {
        let (data_sender, data_receiver) = channel();
        let (kill_sender, kill_receiver) = channel();
        self.kill_sender = Some(kill_sender);
        std::thread::spawn(|| {
            let bli = BirdListenerInternal::default();
            let _ = bli.start(data_sender, kill_receiver, strategy);
        });
        Ok(data_receiver)
    }

    fn close(self) -> Result<(), Box<dyn std::error::Error>> {
        match self.kill_sender {
            Some(sender) => {
                sender.send(false)?;
                Ok(())
            }
            None => Err(Box::new(StreamCloseError)),
        }
    }
}

struct BirdListenerInternal {}

impl BirdListenerInternal {
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
            sender.send(decoded).unwrap();
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
}

impl BirdListenerInternal {
    // Idea is that start can be used within the thread to handle blocking
    // as well as conclusion of receiving input.
    fn start<'a, T>(
        &self,
        sender: Sender<Vec<u8>>,
        receiver: Receiver<bool>,
        strategy: T,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        T: Strategy,
    {
        let play_stream = self.setup_receiver(strategy, sender)?;
        play_stream.play()?;
        let value = receiver.recv()?;
        if value {
            return Ok(());
        }
        Ok(())
    }

    fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}
