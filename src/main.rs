use anyhow::Context as _;
use clap::Parser as _;
use rodio::Source as _;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

#[derive(clap::Parser)]
#[clap(author, version, about)]
struct Options {
    /// Frequency in Hz
    #[clap(value_parser)]
    frequency: f32,
    #[clap(default_value_t = 1.0, value_parser)]
    amplitude: f32,
}

fn main() -> anyhow::Result<()> {
    let options = Options::parse();
    if !(0.0..=20000.0).contains(&options.frequency) {
        anyhow::bail!("Frequency must be in the range [0, 20000]");
    }
    if !(options.amplitude >= 0.0) {
        anyhow::bail!("Amplitude must be >= 0");
    }

    let (_handler, audio_output) =
        rodio::OutputStream::try_default().context("Couldn't select audio device")?;
    audio_output
        .play_raw(rodio::source::SineWave::new(options.frequency).amplify(options.amplitude))
        .context("Failed to play the audio")?;

    // Catch Ctrl-C.
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    if let Err(e) = ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }) {
        eprintln!("Warning: Couldn't set Ctrl-C handler: {}", e);
    }

    // Sleep forever
    while running.load(Ordering::SeqCst) {
        sleep(Duration::from_millis(100));
    }
    println!("Exiting");
    Ok(())
}
