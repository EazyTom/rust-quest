//! Integration tests for JSON progress save/load round-trips.
//!
//! LEARN: tests write under `.rust-test/test-work/` so saves never leave the repo.

use std::fs;

use rust_quest::game::progress::{
    load_progress_from, progress_path_in, save_progress_to, test_work_dir,
};
use rust_quest::game::state::GameState;

#[test]
fn roundtrip_save() {
    let base = test_work_dir().join("progress_save_test");
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).unwrap();
    let path = progress_path_in(&base);
    let state = GameState {
        player_name: "Ayush".into(),
        ..Default::default()
    };
    save_progress_to(&state, &path).unwrap();
    let loaded = load_progress_from(&path).unwrap();
    assert_eq!(loaded.player_name, "Ayush");
    let _ = fs::remove_dir_all(&base);
}

#[test]
fn corrupt_file_returns_none() {
    let base = test_work_dir().join("corrupt_test");
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).unwrap();
    let path = progress_path_in(&base);
    fs::write(&path, "not json").unwrap();
    assert!(load_progress_from(&path).is_none());
    let _ = fs::remove_dir_all(&base);
}

#[test]
fn fixture_version_matches() {
    let text = include_str!("fixtures/progress_v1.json");
    let v: serde_json::Value = serde_json::from_str(text).unwrap();
    assert_eq!(v["version"].as_u64().unwrap(), 1);
    let base = test_work_dir().join("v1_fixture_load");
    let _ = fs::remove_dir_all(&base);
    fs::create_dir_all(&base).unwrap();
    let path = progress_path_in(&base);
    fs::write(&path, text).unwrap();
    let loaded = load_progress_from(&path).unwrap();
    assert_eq!(loaded.player_name, "Ayush");
    let _ = fs::remove_dir_all(&base);
}
