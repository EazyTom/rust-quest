//! Rust Quest binary entry point.
//!
//! LEARN: main runs setup → load save → hub loop → save on exit.

use rust_quest::game::ui::terminal;
use rust_quest::game::{GameState, load_progress, run_hub, save_progress};

fn main() {
    terminal::setup();
    let mut state = load_progress();
    if let Err(e) = run_game(&mut state) {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run_game(state: &mut GameState) -> std::io::Result<()> {
    let quit = run_hub(state)?;
    if quit {
        save_progress(state)?;
    }
    Ok(())
}
