use crate::encoding::encode_bits;
use async_trait::async_trait;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Device;

type Meters = f64;

pub struct ReceiverInfo {
    pub id: String,
    pub distance: Meters,
}

// This will contain logic for outbound data transmission
#[async_trait]
pub trait BirdSender {
    async fn nearby_receivers(&self) -> Result<Vec<ReceiverInfo>, Box<dyn std::error::Error>>;
    async fn broadcast_data(&self, info: &[u8]) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct BirdIOutput {
    device: Device,
}

fn write_data<T>(output: &mut [T], channels: usize, samples: &[T])
where
    T: cpal::Sample,
    T: std::marker::Send,
{
    for frame in output.chunks_mut(channels) {
        for val in samples {
            for sample in frame.iter_mut() {
                *sample = *val;
            }
        }
    }
}

impl BirdIOutput {
    pub fn default() -> Self {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("Can't find audio device on this system");
        BirdIOutput { device }
    }

    pub fn play_low_freq(&self) {
        let mut values = vec![];
        let mut sample_clock = 0.0;
        let config: cpal::SupportedStreamConfig = self.device.default_output_config().unwrap().into();
        let sample_rate = config.sample_rate().0 as f32;
        for _ in 0..100000 {
            sample_clock = sample_clock + 1.0 % sample_rate; 
            values.push((sample_clock * 44000.0 * 2.0 * 3.14159 / sample_rate).sin());
        }
        let _ = self.play_encoded_bits(values);
    }

    fn play_encoded_bits<T>(&self, data: Vec<T>) -> Result<(), Box<dyn std::error::Error>>
    where
        T: cpal::Sample,
        T: 'static,
        T: std::marker::Send,
    {
        let err_fn = |err| println!("Error occurred: {}", err);
        let config: cpal::SupportedStreamConfig = self.device.default_output_config()?.into();
        let channels = config.channels() as usize;

        let output_stream = self.device.build_output_stream(
            &config.config(),
            move |input: &mut [T], _: &cpal::OutputCallbackInfo| {
                write_data::<T>(input, channels, &data)
            },
            err_fn,
        )?;
        output_stream.play()?;
        std::thread::sleep(std::time::Duration::from_millis(1000));
        Ok(())
    }
}

#[async_trait]
impl BirdSender for BirdIOutput {
    async fn nearby_receivers(&self) -> Result<Vec<ReceiverInfo>, Box<dyn std::error::Error>> {
        todo!()
    }

    async fn broadcast_data(&self, info: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let fmt = self.device.default_output_config()?;
        match fmt.sample_format() {
            cpal::SampleFormat::U16 => {
                let data = encode_bits::<u16>(info);
                self.play_encoded_bits(data)
            }
            cpal::SampleFormat::I16 => {
                let data = encode_bits::<i16>(info);
                self.play_encoded_bits(data)
            }
            cpal::SampleFormat::F32 => {
                let data = encode_bits::<f32>(info);
                self.play_encoded_bits(data)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::output::*;
    #[tokio::test]
    async fn sanity_sound_output() {
        let driver = BirdIOutput::default();
        let test_data = vec![200; 1000];
        let _ = driver.broadcast_data(&test_data).await;
    }

    #[tokio::test]
    async fn test_low_freq() {
        let driver = BirdIOutput::default();
        let test_data = driver.play_low_freq();
    }
}
