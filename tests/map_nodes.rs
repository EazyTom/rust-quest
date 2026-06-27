//! Integration tests for quest map node status (pure UI logic).
//!
//! LEARN: integration tests live in `tests/` and link the crate like an external user would.

use rust_quest::game::state::GameState;
use rust_quest::game::ui::{NodeStatus, node_status};

#[test]
fn locked_second_quest_by_default() {
    let state = GameState::default();
    assert_eq!(node_status(&state, "cargo"), NodeStatus::Available);
    assert_eq!(node_status(&state, "types"), NodeStatus::Locked);
}

#[test]
fn completed_shows_checkmark_state() {
    let state = GameState {
        completed_quests: ["cargo".into()].into_iter().collect(),
        ..Default::default()
    };
    assert_eq!(node_status(&state, "cargo"), NodeStatus::Completed);
}

#[test]
fn initial_selection_advances_to_next_quest() {
    use rust_quest::game::ui::initial_map_selection;

    let fresh = GameState::default();
    assert_eq!(initial_map_selection(&fresh), 0);

    let after_cargo = GameState {
        completed_quests: ["cargo".into()].into_iter().collect(),
        ..Default::default()
    };
    assert_eq!(initial_map_selection(&after_cargo), 1);
}

#[test]
fn practice_unlock_all_available() {
    let state = GameState {
        practice_unlock_all: true,
        ..Default::default()
    };
    assert_eq!(node_status(&state, "concurrency"), NodeStatus::Available);
}
