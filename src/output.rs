use cpal::Device;
use async_trait::async_trait;
use crate::encoding::{encode_bits_with_format, package_bits, EncodedBits};
use cpal::traits::DeviceTrait;

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
    async fn play_encoded_bits(&self, data: EncodedBits) -> Result<(), Box<dyn std::error::Error>> {

    }
}

#[async_trait]
impl Sender for BirdIOutput {

    async fn nearby_receivers(&self) -> Result<Vec<ReceiverInfo>, Box<dyn std::error::Error>> {
        todo!()
    }

    async fn broadcast_data(&self, info: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let fmt = self.device.default_output_format()?;
        let data = encode_bits_with_format(info, &fmt);
        let transmit_data = package_bits(&data, &fmt); 
        self.play_encoded_bits(transmit_data).await
    }
}
