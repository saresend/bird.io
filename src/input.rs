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

    pub fn create_handler_fn<T: cpal::Sample>() -> impl Fn(&[T], &cpal::InputCallbackInfo) {
        return move |data: &[T], _: &cpal::InputCallbackInfo| {
            println!("{}", data.len());
        };
    }

    pub fn recv<K: Strategy>(&self, strategy: K) {
        let config: cpal::SupportedStreamConfig =
            self.device.default_input_config().unwrap().into();

        let err_fn = |err| println!("{}", err);
        let input_stream = match config.sample_format() {
            cpal::SampleFormat::F32 => self.device.build_input_stream(
                &config.config(),
                BirdIInput::create_handler_fn::<f32>(),
                err_fn,
            ),
            cpal::SampleFormat::I16 => self.device.build_input_stream(
                &config.config(),
                BirdIInput::create_handler_fn::<i16>(),
                err_fn,
            ),
            cpal::SampleFormat::U16 => self.device.build_input_stream(
                &config.config(),
                BirdIInput::create_handler_fn::<u16>(),
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
        receiver.recv(strategy::NaiveFrequencyModulation {});
    }
}
