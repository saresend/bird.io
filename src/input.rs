// This will contain logic for reading in input
use crate::encoding;
use async_trait::async_trait;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Device;
use std::sync::Mutex;
use std::thread::Thread;

pub struct BirdIInput {
    device: Device,
    bit_threshold: usize,
    buffer: Mutex<Vec<u8>>,
    thread_handle: Option<Thread>,
}

impl BirdIInput {
    pub fn default() -> BirdIInput {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .expect("Can't find audio device on this system");
        BirdIInput {
            device,
            bit_threshold: 100,
            buffer: Mutex::new(vec![]),
            thread_handle: None,
        }
    }

    fn write_fn<T>(&self) -> impl FnMut(&[T], &cpal::InputCallbackInfo) + '_
    where
        T: cpal::Sample,
    {
        return move |data: &[T], _: &cpal::InputCallbackInfo| {
            let values = encoding::decode_frame::<T>(data);
            let mut buffer = self.buffer.lock().unwrap();
            buffer.clear();
        };
    }

    pub fn recv(&self) {
        /* Create our input stream, and pass in a function that
         * then writes data to our buffer
         */
        let config: cpal::SupportedStreamConfig =
            self.device.default_input_config().unwrap().into();
        let err_fn = |err| println!("{}", err);
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => {
                self.device
                    .build_input_stream(&config.config(), self.write_fn::<f32>(), err_fn)
            }
            cpal::SampleFormat::I16 => {
                self.device
                    .build_input_stream(&config.config(), self.write_fn::<i16>(), err_fn)
            }
            cpal::SampleFormat::U16 => {
                self.device
                    .build_input_stream(&config.config(), self.write_fn::<u16>(), err_fn)
            }
        };
    }
}

#[cfg(test)]
mod tests {}
