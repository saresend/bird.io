use crate::errors::DeviceNotFoundError;
use crate::frequency_modulation::NaiveFrequencyModulation;
use crate::traits::BirdSender;
use crate::traits::Strategy;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Device;

pub struct BirdIOutput {
    device: Device,
}

impl BirdIOutput {
    pub fn default() -> Self {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .expect("Can't find audio device on this system");
        BirdIOutput { device }
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
        todo!()
    }
    /*
    pub fn play_bits<K: Strategy>(&self, data: &[u8], strategy: K) {
        let config: cpal::SupportedStreamConfig =
            self.device.default_output_config().unwrap().into();
        let err_fn = |err| eprintln!("{}", err);
        let o_stream = match config.sample_format() {
            cpal::SampleFormat::F32 => {
                let data = strategy.encode_bits::<f32>(data);
                self.device.build_output_stream(
                    &config.config(),
                    BirdIOutput::create_fn::<f32>(data),
                    err_fn,
                )
            }
            cpal::SampleFormat::I16 => {
                let data = strategy.encode_bits::<i16>(data);
                self.device.build_output_stream(
                    &config.config(),
                    BirdIOutput::create_fn::<i16>(data),
                    err_fn,
                )
            }
            cpal::SampleFormat::U16 => {
                let data = strategy.encode_bits::<i16>(data);
                self.device.build_output_stream(
                    &config.config(),
                    BirdIOutput::create_fn::<i16>(data),
                    err_fn,
                )
            }
        }
        .unwrap();
        o_stream.play().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(3000));
    }

    fn create_fn<T: cpal::Sample>(bits: Vec<T>) -> impl FnMut(&mut [T], &cpal::OutputCallbackInfo) {
        #[cfg(debug_assertions)]
        crate::instrumentation::save_data(&bits, 0);

        let mut bit_iter = bits.into_iter();
        return move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                match bit_iter.next() {
                    Some(value) => *sample = cpal::Sample::from(&value),
                    None => *sample = cpal::Sample::from(&0.0),
                }
            }
        };
    }
    */
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
