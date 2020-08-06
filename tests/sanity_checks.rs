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
        let example_bits = vec![1; 100];
        let strategy = NaiveFrequencyModulation::default();
        let output = output::BirdIOutput::default();
        output.transmit(strategy, &example_bits).unwrap();
        println!("Finished sending");
    });

    loop {
        let output = input_channel.recv().unwrap();
        println!("{:?}", output);
    }
}
