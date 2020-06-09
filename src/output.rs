use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Device;
use dasp::signal;
use dasp::signal::Signal;
use crate::strategy::Strategy;

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

    pub fn play_bits<K: 'static + Strategy>(&self, data: &[u8], strategy: K) {
        let config: cpal::SupportedStreamConfig =
            self.device.default_output_config().unwrap().into();
        let err_fn = |err| eprintln!("{}", err);
        let o_stream = match config.sample_format() {
            cpal::SampleFormat::F32 => self.device.build_output_stream(
                &config.config(),
                BirdIOutput::create_strategy_fn::<f32, K>(data.to_vec(), strategy),
                err_fn,
            ),
            cpal::SampleFormat::I16 => self.device.build_output_stream(
                &config.config(),
                BirdIOutput::create_strategy_fn::<f32, K>(data.to_vec(), strategy),
                err_fn,
            ),
            cpal::SampleFormat::U16 => self.device.build_output_stream(
                &config.config(),
                BirdIOutput::create_strategy_fn::<f32, K>(data.to_vec(), strategy),
                err_fn,
            ),
        }
        .unwrap();
        o_stream.play().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(3000));
    }

    fn create_strategy_fn<T: cpal::Sample, K: Strategy>(data: Vec<u8>, strategy: K) -> impl FnMut(&mut [T], &cpal::OutputCallbackInfo) 
    {
        let bits = strategy.encode_bits::<T>(&data);
        let mut bit_iter = bits.into_iter();
        return move | data: &mut [T], output: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                match bit_iter.next() {
                    Some(value) => {*sample = cpal::Sample::from(&value)},
                    None => { *sample = cpal::Sample::from(&0.0) },
                }
            }
        }
    }

    fn create_tonal_bit_encoding<T: cpal::Sample>(
        bits: Vec<u8>,
    ) -> impl FnMut(&mut [T], &cpal::OutputCallbackInfo) {
        let mut sample_iterator = bits.into_iter();
        return move |data: &mut [T], output: &cpal::OutputCallbackInfo| {
            if sample_iterator.next() == Some(0) {
                BirdIOutput::create_tone_fn::<T>(5000.0)(data, output);
            } else {
                BirdIOutput::create_tone_fn::<T>(10000.0)(data, output);
            }
        };
    }

    fn create_tone_fn<T: cpal::Sample>(freq: f64) -> impl Fn(&mut [T], &cpal::OutputCallbackInfo) {
        return move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            let mut signal = signal::rate(44100.0).const_hz(freq).sine();
            for sample in data.iter_mut() {
                *sample = cpal::Sample::from(&(signal.next() as f32));
            }
        };
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
