//! LEARN: test game rules without a terminal.

use rust_quest::game::state::{GameState, QuestStep, StepResult};
use rust_quest::game::xp::Rank;

#[test]
fn sequential_unlock() {
    let state = GameState::default();
    assert!(state.is_unlocked("cargo"));
    assert!(!state.is_unlocked("types"));
}

#[test]
fn completing_cargo_unlocks_types() {
    let mut state = GameState::default();
    let today = "2026-06-27";
    state.complete_step("cargo", QuestStep::Challenge, today);
    assert!(state.is_unlocked("types"));
}

#[test]
fn practice_unlock_all() {
    let state = GameState {
        practice_unlock_all: true,
        ..Default::default()
    };
    assert!(state.is_unlocked("advanced_cargo"));
}

#[test]
fn xp_awarded_once() {
    let mut state = GameState::default();
    let today = "2026-06-27";
    assert!(matches!(
        state.complete_step("cargo", QuestStep::Learn, today),
        StepResult::XpGained { amount: 15, .. }
    ));
    assert_eq!(
        state.complete_step("cargo", QuestStep::Learn, today),
        StepResult::AlreadyDone
    );
}

#[test]
fn rank_after_ownership() {
    let mut state = GameState::default();
    let today = "2026-06-27";
    for id in ["cargo", "types"] {
        state.complete_step(id, QuestStep::Challenge, today);
    }
    state.complete_step("ownership", QuestStep::Challenge, today);
    assert_eq!(state.rank(), Rank::MemoryKeeper);
}
