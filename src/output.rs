use crate::encoding::{encode_bits_with_format, package_bits, EncodedBits};
use async_trait::async_trait;
use cpal::traits::DeviceTrait;
use cpal::Device;

type Meters = f64;

pub struct ReceiverInfo {
    pub id: String,
    pub distance: Meters,
}

// This will contain logic for outbound data transmission
#[async_trait]
pub trait Sender {
    async fn nearby_receivers(&self) -> Result<Vec<ReceiverInfo>, Box<dyn std::error::Error>>;
    async fn broadcast_data(&self, info: &[u8]) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct BirdIOutput {
    device: Device,
}

impl BirdIOutput {
    fn play_encoded_bits(&self, data: EncodedBits) -> Result<(), Box<dyn std::error::Error>> {
        let err_fn = |err| println!("Error occurred: {}", err);
        let config = self.device.default_output_config()?;
        let outputstream = match data {
            EncodedBits::I16(val) => {
                self.device.build_output_stream();
            
            }
            EncodedBits::U16(val) => {}
            EncodedBits::F32(val) => {}
        };
        todo!()
    }
}

#[async_trait]
impl Sender for BirdIOutput {
    async fn nearby_receivers(&self) -> Result<Vec<ReceiverInfo>, Box<dyn std::error::Error>> {
        todo!()
    }

    async fn broadcast_data(&self, info: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let fmt = self.device.default_output_config()?;
        let data = encode_bits_with_format(info, &fmt.sample_format());
        let transmit_data = package_bits(&data, &fmt.sample_format());
        self.play_encoded_bits(transmit_data)
    }
}
