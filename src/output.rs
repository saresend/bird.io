use crate::errors::DeviceNotFoundError;
use crate::frequency_modulation::NaiveFrequencyModulation;
use crate::traits::BirdSender;
use crate::traits::Strategy;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use std::time::Duration;

pub struct BirdIOutput {}

impl BirdIOutput {
    pub fn default() -> Self {
        BirdIOutput {}
    }

    fn create_fn<T>(
        &self,
        data: Vec<f64>,
    ) -> impl FnMut(&mut [T], &cpal::OutputCallbackInfo) + Send + 'static
    where
        T: cpal::Sample,
    {
        let mut index = 0;
        move |input, _| {
            for sample in input.iter_mut() {
                if index < data.len() {
                    *sample = cpal::Sample::from(&(data[index] as f32));
                    index += 1;
                } else {
                    *sample = cpal::Sample::from(&0.0);
                }
            }
        }
    }

    fn create_err_fn(&self) -> impl Fn(cpal::StreamError) {
        |err| eprintln!("{}", err)
    }

    fn estimate_time(config: &cpal::SupportedStreamConfig, num_samples: usize) -> u32 {
        let sample_rate: u32 = config.sample_rate().0;
        let time_length = 1000.0 * (num_samples as f32 / sample_rate as f32);
        return (time_length * 1.5) as u32;
    }

    fn play_bits<T>(
        &self,
        data: Vec<f64>,
        device: cpal::Device,
        config: cpal::SupportedStreamConfig,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        T: cpal::Sample + 'static,
    {
        let elapsed_time = Self::estimate_time(&config, data.len());
        let stream = device.build_output_stream(
            &config.config(),
            self.create_fn::<T>(data),
            self.create_err_fn(),
        )?;
        stream.play()?;
        std::thread::sleep(Duration::from_millis(elapsed_time as u64));
        Ok(())
    }
}

impl BirdSender<NaiveFrequencyModulation> for BirdIOutput {
    fn transmit(
        &self,
        strategy: NaiveFrequencyModulation,
        data: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let host = cpal::default_host();
        let device = host.default_output_device().ok_or(DeviceNotFoundError)?;
        let config = device.default_output_config()?;
        let mut encoder = strategy.create_encoding(config.sample_rate().0);
        let encoded_data = encoder(data);
        match config.sample_format() {
            cpal::SampleFormat::F32 => self.play_bits::<f32>(encoded_data, device, config),
            cpal::SampleFormat::I16 => self.play_bits::<i16>(encoded_data, device, config),
            cpal::SampleFormat::U16 => self.play_bits::<u16>(encoded_data, device, config),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::output;
    use super::*;
    //TODO: Write tests once the APIs start finalizing
    #[test]
    fn simple_output_test() {
        let sample_bits = vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ];
        let strategy = NaiveFrequencyModulation::default();
        let output = output::BirdIOutput::default();
        output.transmit(strategy, &sample_bits).unwrap();
    }
}
