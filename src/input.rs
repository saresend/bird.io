// This will contain logic for reading in input
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

    pub fn recv(&self) {
        /* Create our input stream, and pass in a function that
         * then writes data to our buffer
         */
        let _config: cpal::SupportedStreamConfig =
            self.device.default_input_config().unwrap().into();
        //let err_fn = |err| println!("{}", err);
        todo!()
    }
}

#[cfg(test)]
mod tests {}
