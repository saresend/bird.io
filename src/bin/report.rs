use birdio::*;
fn main() {
    let driver = output::BirdIOutput::default();
    let test_bits = [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1,
        1, 1, 1,
    ];
    driver.play_bits(&test_bits, strategy::NaiveFrequencyModulation::default()); // This should store data
    let data = instrumentation::get_data(0);
    let _ = instrumentation::visualize_pcm(&data, "tone_output.png");
}
