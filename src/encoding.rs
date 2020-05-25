// This contains logic for taking raw bits and encoding it into audio
use cpal::Format;

pub fn encode_bits_with_format(data: &[u8], _format: &Format) -> Vec<u8> {
    data.clone().to_owned()
}

pub fn package_bits(data: &[u8], _format: &Format) -> Vec<f32> {
    todo!()
}
