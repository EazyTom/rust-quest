//! Rust Quest library — quests, game engine, and learning content.
//!
//! Run the game with `cargo run`. Read `main.rs` first, then `game/state.rs`.

pub mod game;
pub mod resources;
pub mod topics;
pub mod version;

pub use game::{GameState, load_progress, run_hub, save_progress};
pub use version::VERSION;
