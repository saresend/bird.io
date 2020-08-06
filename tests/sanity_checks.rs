use birdio::frequency_modulation::*;
use birdio::traits::*;
use birdio::{input, output};

#[test]
fn test_basics() {
    use std::thread;
    use std::time::Duration;

    let mut input = input::BirdListener::default();
    let strategy = NaiveFrequencyModulation::default();
    let input_channel = input.start(strategy).unwrap();
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(100));
        let mut example_bits = vec![0; 100];
        let mut example_bits1 = vec![1; 100];
        let mut example_bits2 = vec![0; 100];
        let mut example_bits3 = vec![1; 100];
        example_bits.append(&mut example_bits1);
        example_bits.append(&mut example_bits2);
        example_bits.append(&mut example_bits3);
        let strategy = NaiveFrequencyModulation::default();
        let output = output::BirdIOutput::default();
        output.transmit(strategy, &example_bits).unwrap();
    });
    let output = input_channel.recv().unwrap();
    println!("{:?}", output);
}
