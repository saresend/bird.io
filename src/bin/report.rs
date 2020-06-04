use birdio::*;
fn main() {
    let driver = output::BirdIOutput::default();
    driver.play_tone(); // This should store data
    let data = instrumentation::get_data(0);
    let _ = instrumentation::visualize_pcm(&data, "tone_output.png");
}
