// This will contain logic for reading in input
use crate::encoding;
use async_trait::async_trait;
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::Device;

pub struct BirdIInput {
    device: Device,
}

type Callback = fn(Vec<u8>);

pub trait Receiver {
    fn receive_data(&self, callback: Callback) -> Result<(), Box<dyn std::error::Error>>;
    fn clear_listener(&self);
}

impl BirdIInput {
    pub fn default() -> BirdIInput {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .expect("Can't find audio device on this system");
        BirdIInput { device }
    }
}

impl Receiver for BirdIInput {
    fn receive_data(&self, callback: Callback) -> Result<(), Box<dyn std::error::Error>> {
        let config = self.device.default_input_config()?;
        let err_fn = |err| eprintln!("Error occurred during receive: {}", err);
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => self.device.build_input_stream(
                &config.into(),
                move |data: &[f32], _: &_| {
                    callback(encoding::decode_bits::<f32>(data, &cpal::SampleFormat::F32))
                },
                err_fn,
            )?,
            cpal::SampleFormat::I16 => self.device.build_input_stream(
                &config.into(),
                move |data: &[i16], _: &_| {
                    callback(encoding::decode_bits::<i16>(data, &cpal::SampleFormat::I16))
                },
                err_fn,
            )?,
            cpal::SampleFormat::U16 => self.device.build_input_stream(
                &config.into(),
                move |data: &[u16], _: &_| {
                    callback(encoding::decode_bits::<u16>(data, &cpal::SampleFormat::U16))
                },
                err_fn,
            )?,
        };
        let receiver_thread = std::thread::spawn(move || {
            use cpal::traits::StreamTrait;
            stream.play();
        });
        todo!()
    }

    fn clear_listener(&self) {
        todo!()
    }
}
