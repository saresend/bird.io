// This will contain logic for reading in input
use crate::strategy::Strategy;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::Device;

pub struct BirdIInput {
    device: Device,
}

impl BirdIInput {
    pub fn default() -> BirdIInput {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .expect("Can't find audio device on this system");
        BirdIInput { device }
    }

    pub fn create_handler_fn<K: Strategy, T: cpal::Sample>(
        mut strategy: K,
    ) -> impl FnMut(&[T], &cpal::InputCallbackInfo) {
        return move |data: &[T], _: &cpal::InputCallbackInfo| {
            let _result = strategy.decode_bit(data);
        };
    }

    pub fn recv<K: 'static + Strategy + Send>(&self, strategy: K) {
        let config: cpal::SupportedStreamConfig =
            self.device.default_input_config().unwrap().into();

        let err_fn = |err| println!("{}", err);
        let input_stream = match config.sample_format() {
            cpal::SampleFormat::F32 => self.device.build_input_stream(
                &config.config(),
                BirdIInput::create_handler_fn::<K, f32>(strategy),
                err_fn,
            ),
            cpal::SampleFormat::I16 => self.device.build_input_stream(
                &config.config(),
                BirdIInput::create_handler_fn::<K, i16>(strategy),
                err_fn,
            ),
            cpal::SampleFormat::U16 => self.device.build_input_stream(
                &config.config(),
                BirdIInput::create_handler_fn::<K, u16>(strategy),
                err_fn,
            ),
        }
        .unwrap();
        input_stream.play().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn sanity_input_test() {
        let receiver = input::BirdIInput::default();
        receiver.recv(strategy::NaiveFrequencyModulation::default());
        let output_data = instrumentation::get_data(0);
        let pre_fft_data = instrumentation::get_data(1);
        instrumentation::visualize_pcm(&output_data, "output.png");
        instrumentation::visualize_pcm(&pre_fft_data, "pre_fft_output.png");
    }
}
