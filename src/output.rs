use crate::encoding::{encode_bits_with_format, package_bits, EncodedBits};
use async_trait::async_trait;
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use cpal::Device;

type Meters = f64;

pub struct ReceiverInfo {
    pub id: String,
    pub distance: Meters,
}

// This will contain logic for outbound data transmission
#[async_trait]
pub trait Sender {
    async fn nearby_receivers(&self) -> Result<Vec<ReceiverInfo>, Box<dyn std::error::Error>>;
    async fn broadcast_data(&self, info: &[u8]) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct BirdIOutput {
    device: Device,
}

fn write_data<T>(
    output: &mut [T],
    channels: usize,
    mut next_sample: impl std::iter::Iterator<Item = T>,
) where
    T: cpal::Sample,
{
    for frame in output.chunks_mut(channels) {
        let val = next_sample.next();
        match val {
            Some(value) => {
                for sample in frame.iter_mut() {
                    *sample = value;
                }
            }
            None => break,
        }
    }
}

impl BirdIOutput {

    pub fn default() -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("Can't find audio device on this system");
        BirdIOutput { device } 
    }
    fn play_encoded_bits(&self, data: EncodedBits) -> Result<(), Box<dyn std::error::Error>> {
        let err_fn = |err| println!("Error occurred: {}", err);
        let config: cpal::StreamConfig = self.device.default_output_config()?.into();
        let channels = config.channels as usize;
        let output_stream = match data {
            EncodedBits::I16(ref val) => {
                let new_ref: Vec<i16> = val.clone();
                self.device.build_output_stream(
                    &config,
                    move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                        write_data(data, channels, new_ref.clone().into_iter());
                    },
                    err_fn,
                )?
            }
            EncodedBits::U16(val) => {
                let new_ref: Vec<u16> = val.clone();
                self.device.build_output_stream(
                    &config,
                    move |data: &mut [u16], _: &cpal::OutputCallbackInfo| {
                        write_data(data, channels, new_ref.clone().into_iter());
                    },
                    err_fn,
                )?
            }
            EncodedBits::F32(val) => {
                let new_ref: Vec<f32> = val.clone();
                self.device.build_output_stream(
                    &config,
                    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                        write_data(data, channels, new_ref.clone().into_iter());
                    },
                    err_fn,
                )?
            }
        };
        output_stream.play()?;
        Ok(()) 
    }
}

#[async_trait]
impl Sender for BirdIOutput {
    async fn nearby_receivers(&self) -> Result<Vec<ReceiverInfo>, Box<dyn std::error::Error>> {
        todo!()
    }

    async fn broadcast_data(&self, info: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let fmt = self.device.default_output_config()?;
        let data = encode_bits_with_format(info, &fmt.sample_format());
        let transmit_data = package_bits(&data, &fmt.sample_format());
        self.play_encoded_bits(transmit_data)
    }
}


#[cfg(test)]
mod tests {
    use crate::output::*;
    #[tokio::test]
    async fn sanity_sound_output() {
        let driver = BirdIOutput::default(); 
        let test_data = vec![200; 1000];
        driver.broadcast_data(&test_data);
    }


}
