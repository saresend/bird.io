// This will contain logic for reading in input
use crate::strategy::Strategy;
use cpal::traits::{DeviceTrait, HostTrait};
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

    pub fn recv<K: Strategy>(&self, strategy: K) {
        let _config: cpal::SupportedStreamConfig =
            self.device.default_input_config().unwrap().into();
        //let err_fn = |err| println!("{}", err);
        todo!()
    }
}

#[cfg(test)]
mod tests {}
