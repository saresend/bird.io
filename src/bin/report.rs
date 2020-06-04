use birdio::*;
fn main() {
    let driver = output::BirdIOutput::default();
    driver.play_tone(); // This should store data
}
