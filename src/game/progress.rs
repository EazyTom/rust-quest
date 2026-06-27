//! Save/load player progress to `.rust-test/progress.json`.
//!
//! All paths are relative to `CARGO_MANIFEST_DIR` — nothing outside the repo.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::state::GameState;

pub const SAVE_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// LEARN: #[derive(Serialize, Deserialize)] — see Testing/Docs quest; auto-implements JSON save format.
pub struct SaveData {
    pub version: u32,
    pub player_name: String,
    pub xp: u32,
    pub completed_steps: HashSet<String>,
    pub completed_quests: HashSet<String>,
    pub achievements: HashSet<String>,
    pub practice_unlock_all: bool,
    pub streak_days: u32,
    pub last_played_date: String,
}

impl Default for SaveData {
    fn default() -> Self {
        Self {
            version: SAVE_VERSION,
            player_name: String::new(),
            xp: 0,
            completed_steps: HashSet::new(),
            completed_quests: HashSet::new(),
            achievements: HashSet::new(),
            practice_unlock_all: false,
            streak_days: 0,
            last_played_date: String::new(),
        }
    }
}

impl From<&GameState> for SaveData {
    fn from(state: &GameState) -> Self {
        SaveData {
            version: SAVE_VERSION,
            player_name: state.player_name.clone(),
            xp: state.xp,
            completed_steps: state.completed_steps.clone(),
            completed_quests: state.completed_quests.clone(),
            achievements: state.achievements.clone(),
            practice_unlock_all: state.practice_unlock_all,
            streak_days: state.streak_days,
            last_played_date: state.last_played_date.clone(),
        }
    }
}

impl From<SaveData> for GameState {
    fn from(data: SaveData) -> Self {
        GameState {
            player_name: data.player_name,
            xp: data.xp,
            completed_steps: data.completed_steps,
            completed_quests: data.completed_quests,
            achievements: data.achievements,
            practice_unlock_all: data.practice_unlock_all,
            streak_days: data.streak_days,
            last_played_date: data.last_played_date,
            ownership_passed_first_try: false,
            errors_challenge_picked_unwrap: false,
        }
    }
}

pub fn data_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(".rust-test")
}

pub fn progress_path() -> PathBuf {
    data_dir().join("progress.json")
}

pub fn test_work_dir() -> PathBuf {
    data_dir().join("test-work")
}

pub fn progress_path_in(base: &Path) -> PathBuf {
    base.join("progress.json")
}

pub fn load_progress() -> GameState {
    load_progress_from(&progress_path()).unwrap_or_default()
}

pub fn load_progress_from(path: &Path) -> Option<GameState> {
    let text = fs::read_to_string(path).ok()?;
    let data: SaveData = serde_json::from_str(&text).ok()?;
    if data.version != SAVE_VERSION {
        return None;
    }
    Some(GameState::from(data))
}

pub fn save_progress(state: &GameState) -> std::io::Result<()> {
    save_progress_to(state, &progress_path())
}

pub fn save_progress_to(state: &GameState, path: &Path) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let data = SaveData::from(state);
    let json = serde_json::to_string_pretty(&data)?;
    fs::write(path, json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_roundtrip() {
        let base = test_work_dir();
        let _ = fs::remove_dir_all(&base);
        let path = progress_path_in(&base);
        let state = GameState {
            player_name: "Ayush".into(),
            xp: 40,
            ..Default::default()
        };
        save_progress_to(&state, &path).unwrap();
        let loaded = load_progress_from(&path).unwrap();
        assert_eq!(loaded.player_name, "Ayush");
        assert_eq!(loaded.xp, 40);
        let _ = fs::remove_dir_all(&base);
    }
}
