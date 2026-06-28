//! Game engine: state, progress, UI, and hub loop.

pub mod achievements;
pub mod audio;
pub mod epic;
pub mod hub;
pub mod narrative;
pub mod progress;
pub mod quiz;
pub mod state;
pub mod ui;
pub mod xp;

pub use audio::{MusicHandle, MusicMode};
pub use hub::run_hub;
pub use progress::{load_progress, save_progress};
pub use state::GameState;
