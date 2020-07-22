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
}

#[cfg(test)]
mod tests {
    //TODO: Write tests once the APIs start finalizing
}
