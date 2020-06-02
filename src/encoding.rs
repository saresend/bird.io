// This contains logic for taking raw bits and encoding it into audio

pub fn encode_bits<T>(data: &[u8]) -> Vec<T>
where
    T: cpal::Sample,
{
    let conv_data: Vec<u16> = data.iter().map(|x| *x as u16).collect();
    conv_data.iter().map(|x| cpal::Sample::from(x)).collect()
}

pub(crate) fn decode_bits<T>(samples: &[T]) -> Vec<u8>
where
    T: cpal::Sample,
{
    samples.iter().map(|x| x.to_u16() as u8).collect()
}
