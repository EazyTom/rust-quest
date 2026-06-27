//! Game engine: state, progress, UI, and hub loop.

pub mod achievements;
pub mod hub;
pub mod progress;
pub mod quiz;
pub mod state;
pub mod ui;
pub mod xp;

pub use hub::run_hub;
pub use progress::{load_progress, save_progress};
pub use state::GameState;
