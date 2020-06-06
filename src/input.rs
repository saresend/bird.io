// This will contain logic for reading in input
use crate::encoding;
use async_trait::async_trait;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Device;

pub struct BirdIInput {
    device: Device,
}

type Callback = fn(Vec<u8>);

#[async_trait]
pub trait Receiver {
    async fn receive_data(&self, callback: Callback) -> Result<(), Box<dyn std::error::Error>>;
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

#[async_trait]
impl Receiver for BirdIInput {
    async fn receive_data(&self, callback: Callback) -> Result<(), Box<dyn std::error::Error>> {
        let config = self.device.default_input_config()?;
        let err_fn = |err| eprintln!("Error occurred during receive: {}", err);
        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => self.device.build_input_stream(
                &config.into(),
                move |data: &[f32], _: &_| callback(encoding::decode_bits::<f32>(data)),
                err_fn,
            )?,
            cpal::SampleFormat::I16 => self.device.build_input_stream(
                &config.into(),
                move |data: &[i16], _: &_| callback(encoding::decode_bits::<i16>(data)),
                err_fn,
            )?,
            cpal::SampleFormat::U16 => self.device.build_input_stream(
                &config.into(),
                move |data: &[u16], _: &_| callback(encoding::decode_bits::<u16>(data)),
                err_fn,
            )?,
        };
        stream.play()?;
        std::thread::sleep(std::time::Duration::from_millis(1000));
        Ok(())
    }

    fn clear_listener(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::input::*;
    use crate::output::*;

    #[tokio::test]
    async fn sanity_test_receiving_data() {
        let output_driver = BirdIOutput::default();
        let test_data = vec![100; 100];
        let output = output_driver.broadcast_data(&test_data);

        let input_driver = BirdIInput::default();
        let input = input_driver.receive_data(move |data| println!("{:?}", data));
        let _ = tokio::join!(input, output);
    }
}
