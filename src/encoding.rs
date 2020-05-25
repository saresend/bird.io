// This contains logic for taking raw bits and encoding it into audio
use cpal::Format;
use cpal::SampleFormat;

pub enum EncodedBits {
    I16(Vec<i16>),
    U16(Vec<u16>),
    F32(Vec<f32>),
}

pub fn encode_bits_with_format(data: &[u8], _format: &Format) -> Vec<u8> {
    data.clone().to_owned()
}

pub fn package_bits(data: &[u8], format: &Format) -> EncodedBits {
    match format.data_type {
        SampleFormat::I16 => EncodedBits::I16(data.iter().map(|x| *x as i16).collect()),
        SampleFormat::U16 => EncodedBits::U16(data.iter().map(|x| *x as u16).collect()),
        SampleFormat::F32 => EncodedBits::F32(data.iter().map(|x| (*x as f32).sin()).collect()),
    }
}
