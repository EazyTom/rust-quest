//! Rust Quest binary entry point.
//!
//! LEARN: main runs setup → load save → hub loop → save on exit.

use rust_quest::game::ui::terminal;
use rust_quest::game::{GameState, MusicHandle, load_progress, run_hub, save_progress};

fn main() {
    terminal::setup();
    let mut state = load_progress();
    let music = MusicHandle::start();
    MusicHandle::launch_music(&mut state, &music);
    if let Err(e) = run_game(&mut state, &music) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run_game(state: &mut GameState, music: &MusicHandle) -> std::io::Result<()> {
    run_hub(state, music)?;
    save_progress(state)?;
    Ok(())
}
