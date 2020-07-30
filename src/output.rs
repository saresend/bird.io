use crate::errors::DeviceNotFoundError;
use crate::frequency_modulation::NaiveFrequencyModulation;
use crate::traits::BirdSender;
use crate::traits::Strategy;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Device;

pub struct BirdIOutput {}

impl BirdIOutput {
    pub fn default() -> Self {
        BirdIOutput {}
    }

    fn create_fn<T>(
        &self,
        data: Vec<f64>,
    ) -> impl Fn(&mut [T], &cpal::OutputCallbackInfo) + Send + 'static
    where
        T: cpal::Sample,
    {
        let index = 0;
        move |input, _| {
            for sample in input.iter_mut() {
                *sample = cpal::Sample::from(&(data[index] as f32));
            }
        }
    }

    fn create_err_fn(&self) -> impl Fn(cpal::StreamError) {
        |err| eprintln!("{}", err)
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
        let stream = device.build_output_stream(
            &config.config(),
            self.create_fn::<T>(data),
            self.create_err_fn(),
        )?;
        stream.play()?;
        Ok(())
    }
}

impl BirdSender<NaiveFrequencyModulation> for BirdIOutput {
    fn transmit(
        &self,
        strategy: NaiveFrequencyModulation,
        data: &[u8],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut encoder = strategy.create_encoding();
        let encoded_data = encoder(data);
        let host = cpal::default_host();
        let device = host.default_output_device().ok_or(DeviceNotFoundError)?;
        let config = device.default_output_config()?;
        match config.sample_format() {
            cpal::SampleFormat::F32 => self.play_bits::<f32>(encoded_data, device, config),
            cpal::SampleFormat::I16 => self.play_bits::<i16>(encoded_data, device, config),
            cpal::SampleFormat::U16 => self.play_bits::<u16>(encoded_data, device, config),
        }
    }
}

#[cfg(test)]
mod tests {
    //TODO: Write tests once the APIs start finalizing
}
