use birdio::*;
fn main() {
    let driver = output::BirdIOutput::default();
    driver.play_low_freq(); // This should store data
    let data = instrumentation::get_data(0);
    let _ = instrumentation::visualize_pcm(&data, "low_freq.png");

    driver.test_dasp_sine();
    let data = instrumentation::get_data(1);
    let _ = instrumentation::visualize_pcm(&data, "dasp_sine.png");
}
