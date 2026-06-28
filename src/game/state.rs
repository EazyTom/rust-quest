//! Pure game rules: unlocks, XP-once, ranks, streaks.
//!
//! No printing here — easy to unit test. Read after `main.rs` startup flow.

use std::collections::HashSet;

use crate::topics::registry;

use super::audio::MusicMode;
use super::xp::{self, Rank, XP_CHALLENGE, XP_LEARN};

/// Quest step identifiers stored in `completed_steps`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuestStep {
    Learn,
    Challenge,
}

impl QuestStep {
    pub fn as_key(self, quest_id: &str) -> String {
        format!("{quest_id}:{}", self.storage_name())
    }

    fn storage_name(self) -> &'static str {
        match self {
            QuestStep::Learn => "learn",
            QuestStep::Challenge => "challenge",
        }
    }
}

/// Result of completing a step — hub uses this for messages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepResult {
    XpGained { amount: u32, total: u32 },
    RankUp { rank: Rank },
    AlreadyDone,
    QuestCompleted { quest_id: String },
}

#[derive(Debug, Clone, Default)]
pub struct GameState {
    pub player_name: String,
    pub xp: u32,
    pub completed_steps: HashSet<String>,
    pub completed_quests: HashSet<String>,
    pub achievements: HashSet<String>,
    pub practice_unlock_all: bool,
    pub streak_days: u32,
    pub last_played_date: String,
    pub ownership_passed_first_try: bool,
    pub errors_challenge_picked_unwrap: bool,
    /// Phase dungeon boss ids defeated (cellar, archives, forge, summit).
    pub dungeon_bosses: HashSet<String>,
    /// Champion victory screen already shown.
    pub victory_celebrated: bool,
    pub music_muted: bool,
    pub music_mode: MusicMode,
    /// Saved track stem for [`MusicMode::Fixed`].
    pub music_track: String,
    /// Last stem in [`MusicMode::CycleOnQuest`] — advances on each quest entry.
    pub music_last_stem: String,
    /// Current session playback stem (not saved).
    pub music_playing_stem: String,
}

impl GameState {
    pub fn rank(&self) -> Rank {
        xp::rank_for_completed(&self.completed_quests)
    }

    pub fn step_done(&self, quest_id: &str, step: QuestStep) -> bool {
        self.completed_steps.contains(&step.as_key(quest_id))
    }

    pub fn quest_completed(&self, quest_id: &str) -> bool {
        self.completed_quests.contains(quest_id)
    }

    // GAME: practice mode skips the sequential unlock chain for replay.
    pub fn is_unlocked(&self, quest_id: &str) -> bool {
        if self.practice_unlock_all {
            return true;
        }
        let quests = registry::all();
        let Some(index) = quests.iter().position(|q| q.id == quest_id) else {
            return false;
        };
        if index == 0 {
            return true;
        }
        // GAME: quest N unlocks only after quest N-1 challenge is passed.
        let prev = quests[index - 1].id;
        self.quest_completed(prev)
    }

    pub fn touch_streak(&mut self, today: &str) {
        if self.last_played_date.is_empty() {
            self.streak_days = 1;
            self.last_played_date = today.to_string();
            return;
        }
        if self.last_played_date == today {
            return;
        }
        if is_next_day(&self.last_played_date, today) {
            self.streak_days += 1;
        } else {
            self.streak_days = 1;
        }
        self.last_played_date = today.to_string();
    }

    pub fn complete_step(&mut self, quest_id: &str, step: QuestStep, today: &str) -> StepResult {
        self.touch_streak(today);
        let key = step.as_key(quest_id);
        // LEARN: HashSet::contains prevents awarding XP twice for the same step.
        if self.completed_steps.contains(&key) {
            return StepResult::AlreadyDone;
        }

        let old_rank = self.rank();
        self.completed_steps.insert(key);

        let xp_amount = match step {
            QuestStep::Learn => XP_LEARN,
            QuestStep::Challenge => XP_CHALLENGE,
        };
        self.xp += xp_amount;

        if step == QuestStep::Learn && quest_id == "cargo" {
            self.achievements.insert("first_steps".into());
        }

        if step == QuestStep::Challenge {
            self.completed_quests.insert(quest_id.to_string());
            self.check_achievements(quest_id);
        }

        let new_rank = self.rank();
        if new_rank != old_rank {
            return StepResult::RankUp { rank: new_rank };
        }

        if step == QuestStep::Challenge {
            return StepResult::QuestCompleted {
                quest_id: quest_id.to_string(),
            };
        }

        StepResult::XpGained {
            amount: xp_amount,
            total: self.xp,
        }
    }

    fn check_achievements(&mut self, quest_id: &str) {
        if quest_id == "ownership" && self.ownership_passed_first_try {
            self.achievements.insert("borrow_slayer".into());
        }
        if quest_id == "errors" && !self.errors_challenge_picked_unwrap {
            self.achievements.insert("no_panic".into());
        }
        if quest_id == "iterators_closures" {
            self.achievements.insert("iterator_hero".into());
        }
        if quest_id == "concurrency" {
            self.achievements.insert("thread_safe".into());
        }
        if self.completed_quests.len() >= 14 {
            self.achievements.insert("full_stack_rustacean".into());
        }
    }

    pub fn defeat_dungeon_boss(&mut self, phase_id: &str) {
        self.dungeon_bosses.insert(phase_id.to_string());
        let achievement = match phase_id {
            "cellar" => Some("cellar_boss"),
            "archives" => Some("archives_boss"),
            "forge" => Some("forge_boss"),
            "summit" => Some("summit_boss"),
            _ => None,
        };
        if let Some(id) = achievement {
            self.achievements.insert(id.into());
        }
    }

    pub fn mark_victory_celebrated(&mut self) {
        self.victory_celebrated = true;
        self.achievements.insert("champion_victory".into());
    }

    pub fn reset(&mut self) {
        let name = self.player_name.clone();
        let music_muted = self.music_muted;
        let music_mode = self.music_mode;
        let music_track = self.music_track.clone();
        let music_last_stem = self.music_last_stem.clone();
        *self = GameState {
            player_name: name,
            music_muted,
            music_mode,
            music_track,
            music_last_stem,
            ..Default::default()
        };
    }

    pub fn is_champion(&self) -> bool {
        self.completed_quests.len() >= 14
    }
}

fn is_next_day(previous: &str, today: &str) -> bool {
    fn day_num(d: &str) -> Option<u32> {
        let parts: Vec<_> = d.split('-').collect();
        if parts.len() != 3 {
            return None;
        }
        let y: u32 = parts[0].parse().ok()?;
        let m: u32 = parts[1].parse().ok()?;
        let day: u32 = parts[2].parse().ok()?;
        Some(y * 372 + m * 31 + day)
    }
    match (day_num(previous), day_num(today)) {
        (Some(p), Some(t)) => t == p + 1,
        _ => false,
    }
}

pub fn today_string() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let days = secs / 86_400;
    let y = 1970 + days / 365;
    let rem = days % 365;
    let m = rem / 30 + 1;
    let d = rem % 30 + 1;
    format!("{y}-{m:02}-{d:02}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_quest_unlocked() {
        let state = GameState::default();
        assert!(state.is_unlocked("cargo"));
        assert!(!state.is_unlocked("types"));
    }

    #[test]
    fn xp_once() {
        let mut state = GameState::default();
        let today = "2026-06-27";
        let r1 = state.complete_step("cargo", QuestStep::Learn, today);
        assert!(matches!(r1, StepResult::XpGained { amount: 15, .. }));
        let r2 = state.complete_step("cargo", QuestStep::Learn, today);
        assert_eq!(r2, StepResult::AlreadyDone);
    }
}
