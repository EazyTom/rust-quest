//! Epic phases, dungeon bosses, book coverage notes, and champion victory screen.
//!
//! GAME: four dungeon bosses link quest groups to [The Rust Book](https://doc.rust-lang.org/book/).

use colored::Colorize;

use crate::game::quiz::{PresentedQuestion, QuizQuestion, score_presented};
use crate::game::state::GameState;
use crate::game::ui::retro;
use crate::topics::registry;

/// One story arc grouping several quests + a dungeon boss fight.
#[derive(Debug, Clone, Copy)]
pub struct EpicPhase {
    pub id: &'static str,
    pub name: &'static str,
    pub boss_name: &'static str,
    pub boss_emoji: &'static str,
    pub intro: &'static str,
    pub quest_ids: &'static [&'static str],
    /// Matching chapters in [The Rust Book](https://doc.rust-lang.org/book/).
    pub book_chapters: &'static str,
}

pub const PHASES: &[EpicPhase] = &[
    EpicPhase {
        id: "cellar",
        name: "The Cellar — Foundations",
        boss_name: "Borrow Checker Warden",
        boss_emoji: "👹",
        intro: "You mastered Cargo, types, ownership, enums, and errors. \
                 The Warden guards the exit — prove you understand memory and `Result`!",
        quest_ids: &["cargo", "types", "ownership", "structs_enums", "errors"],
        book_chapters: "Ch 1–6, 9 (Getting Started through Error Handling)",
    },
    EpicPhase {
        id: "archives",
        name: "The Archives — Abstractions",
        boss_name: "Generic Golem",
        boss_emoji: "🗿",
        intro: "Collections, traits, and lifetimes are the language of reusable Rust. \
                 The Golem tests whether your abstractions compile in the real world.",
        quest_ids: &["collections", "traits_generics", "lifetimes"],
        book_chapters: "Ch 8, 10 (Collections, Generics, Traits, Lifetimes)",
    },
    EpicPhase {
        id: "forge",
        name: "The Forge — Craft",
        boss_name: "Closure Phantom",
        boss_emoji: "👻",
        intro: "Modules organize your code; iterators and smart pointers shape how data flows. \
                 Defeat the Phantom to leave the Forge as a true craftsperson.",
        quest_ids: &["modules_prelude", "iterators_closures", "smart_pointers"],
        book_chapters: "Ch 7, 13, 15–16 (Modules, Iterators, Smart Pointers)",
    },
    EpicPhase {
        id: "summit",
        name: "The Summit — Mastery",
        boss_name: "Thread Dragon",
        boss_emoji: "🐉",
        intro: "Threads, tests, and Cargo features — the last climb. \
                 Slay the Thread Dragon to earn the crown of a Rust Quest Champion!",
        quest_ids: &["concurrency", "testing_docs", "advanced_cargo"],
        book_chapters: "Ch 11–12, 14, Advanced Cargo (Testing, Concurrency, Projects)",
    },
];

/// Topics beginners should still explore in [The Rust Book](https://doc.rust-lang.org/book/)
/// after finishing all 14 quests (we touch some in demos, but no dedicated quest yet).
pub const BOOK_GAPS: &[(&str, &str, &str)] = &[
    (
        "Control flow",
        "if, loop, while, for, match arms",
        "https://doc.rust-lang.org/book/ch03-05-control-flow.html",
    ),
    (
        "Functions & comments",
        "fn, parameters, return values, doc comments",
        "https://doc.rust-lang.org/book/ch03-03-how-functions-work.html",
    ),
    (
        "Slices & arrays",
        "array vs slice, string slices",
        "https://doc.rust-lang.org/book/ch04-03-slices.html",
    ),
    (
        "Common methods",
        "impl methods, associated functions",
        "https://doc.rust-lang.org/book/ch05-03-method-syntax.html",
    ),
    (
        "Pattern power",
        "match guards, destructuring, if let / while let",
        "https://doc.rust-lang.org/book/ch18-00-patterns.html",
    ),
    (
        "Macros (intro)",
        "macro_rules!, derive macros you have seen",
        "https://doc.rust-lang.org/book/ch19-06-macros.html",
    ),
];

pub fn is_phase_gate_quest(quest_id: &str) -> bool {
    PHASES.iter().any(|p| p.quest_ids.last() == Some(&quest_id))
}

/// Phase-end quest with all quests done and boss not yet defeated.
pub fn dungeon_ready_at(state: &GameState, quest_id: &str) -> bool {
    let Some(phase) = phase_containing(quest_id) else {
        return false;
    };
    if phase.quest_ids.last() != Some(&quest_id) {
        return false;
    }
    is_phase_cleared(state, phase) && !boss_defeated(state, phase.id)
}

pub fn phase_containing(quest_id: &str) -> Option<&'static EpicPhase> {
    PHASES.iter().find(|p| p.quest_ids.contains(&quest_id))
}

/// True when every quest in the phase has a passed challenge.
pub fn is_phase_cleared(state: &GameState, phase: &EpicPhase) -> bool {
    phase.quest_ids.iter().all(|id| state.quest_completed(id))
}

pub fn boss_defeated(state: &GameState, phase_id: &str) -> bool {
    state.dungeon_bosses.contains(phase_id)
}

/// Phase whose last quest was just `quest_id`, if the whole phase is now cleared.
pub fn newly_cleared_phase(state: &GameState, quest_id: &str) -> Option<&'static EpicPhase> {
    let phase = phase_containing(quest_id)?;
    let last = *phase.quest_ids.last()?;
    if last != quest_id || !is_phase_cleared(state, phase) {
        return None;
    }
    Some(phase)
}

pub fn all_quests_complete(state: &GameState) -> bool {
    state.completed_quests.len() >= registry::all().len()
}

pub fn dungeon_boss_questions(phase: &EpicPhase) -> Vec<(&'static str, QuizQuestion)> {
    phase
        .quest_ids
        .iter()
        .filter_map(|id| registry::find(id).map(|q| (*id, q.boss)))
        .collect()
}

pub fn print_dungeon_intro(phase: &EpicPhase) {
    println!(
        "\n{}",
        retro::section_header(&format!(
            "{} {} — DUNGEON BOSS",
            phase.boss_emoji, phase.boss_name
        ))
    );
    println!("{}", phase.name.bright_magenta().bold());
    println!("{}\n", retro::dungeon_master_says(phase.intro));
    println!(
        "{}",
        retro::lore_scroll(&format!("Rust Book: {}", phase.book_chapters))
    );
}

pub fn print_book_gaps_guide() {
    println!("\n{}", retro::section_header("📜 Rust Book — explore next"));
    println!(
        "{}",
        "Rust Quest covers the core path of The Rust Programming Language.".dimmed()
    );
    println!(
        "{}\n",
        "These topics are worth a dedicated read after you finish the quests:".dimmed()
    );
    for (topic, detail, _url) in BOOK_GAPS {
        println!("  • {} — {}", topic.bright_white(), detail);
    }
    println!(
        "\n{}\n",
        "Full book: https://doc.rust-lang.org/book/"
            .bright_cyan()
            .underline()
    );
}

/// Epic victory when all 14 quests are complete — show once, then encourage replay.
pub fn celebrate_champion(state: &GameState) {
    let name = &state.player_name;
    let coins = "🪙💰🪙💰🪙💰🪙💰🪙💰🪙💰";
    println!();
    println!(
        "{}",
        retro::box_top(&format!("👑  {name} — RUST QUEST CHAMPION!  👑"))
    );
    println!("{}", retro::box_line(coins));
    println!("{}", retro::box_line("You cleared every quest on the map!"));
    println!(
        "{}",
        retro::box_line("The borrow checker bows to your skill.")
    );
    println!(
        "{}",
        retro::box_line(&format!(
            "👑 Champion rank · {} XP · {} quests cleared",
            state.xp,
            state.completed_quests.len()
        ))
    );
    println!(
        "{}",
        retro::box_line("🪙💰 Treasure hoard: gold coins beyond count!")
    );
    println!(
        "{}",
        retro::box_line("🍺🍺 The tavern roars — legendary win! 🍺🍺")
    );
    println!(
        "{}",
        retro::box_line("📜 Scrolls · 🧪 potions · 💎 Rust gems · 🪄 wands")
    );
    println!("{}", retro::box_bottom());
    println!();
    println!(
        "{}",
        "👑 🪙💰 LEGENDARY VICTORY! 🍺🍺 💰🪙 👑"
            .bright_yellow()
            .bold()
    );
    println!();
    println!("{}", retro::tavern_cheer());
    println!("{}", "Your adventure continues:".bright_white().bold());
    println!("  🧭 Quest Map — replay any demo or challenge for practice");
    println!("  📜 Resources — open every quest’s Rust Book & scrolls");
    println!("  🪄 Sandbox — rerun demos without XP pressure");
    println!("  📖 Book study guide — chapters we skimmed along the way");
    println!();
    print_book_gaps_guide();
    println!(
        "{}",
        format!(
            "Hero {name}, the realm of Rust is yours. 👑🪙 🍺🍺 Keep building! 🧙⚔️",
        )
        .bright_green()
        .bold()
    );
    println!();
}

pub fn score_dungeon_answers(questions: &[PresentedQuestion], answers: &[usize]) -> bool {
    score_presented(questions, answers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::state::{GameState, QuestStep};

    #[test]
    fn four_phases_cover_all_quests() {
        let mut seen = std::collections::HashSet::new();
        for phase in PHASES {
            for id in phase.quest_ids {
                assert!(seen.insert(*id), "duplicate quest {id} in phases");
            }
        }
        assert_eq!(seen.len(), registry::all().len());
    }

    #[test]
    fn cellar_unlocks_after_errors() {
        let mut state = GameState::default();
        let today = "2026-06-27";
        for id in ["cargo", "types", "ownership", "structs_enums"] {
            state.complete_step(id, QuestStep::Challenge, today);
        }
        assert!(newly_cleared_phase(&state, "errors").is_none());
        state.complete_step("errors", QuestStep::Challenge, today);
        let phase = newly_cleared_phase(&state, "errors").unwrap();
        assert_eq!(phase.id, "cellar");
    }
}
