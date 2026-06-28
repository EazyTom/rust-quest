//! XP values and rank titles shown in the hub and profile.
//!
//! Ranks are tied to **quest completion**, not XP alone. XP fills the progress bar.

use std::collections::HashSet;

/// XP granted once per step type.
pub const XP_LEARN: u32 = 15;
pub const XP_CHALLENGE: u32 = 25;
pub const XP_DUNGEON_BOSS: u32 = 50;
pub const MAX_XP: u32 = 14 * (XP_LEARN + XP_CHALLENGE) + 4 * XP_DUNGEON_BOSS;

/// Display rank derived from completed quests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    Initiate,
    CargoRunner,
    MemoryKeeper,
    PatternKnight,
    ErrorHandler,
    CollectionHero,
    TraitMaster,
    LifetimeSage,
    ModuleArchitect,
    Champion,
}

impl Rank {
    pub fn emoji(self) -> &'static str {
        match self {
            Rank::Initiate => "🥚",
            Rank::CargoRunner => "📦",
            Rank::MemoryKeeper => "🦀",
            Rank::PatternKnight => "⚔️",
            Rank::ErrorHandler => "🛡️",
            Rank::CollectionHero => "📚",
            Rank::TraitMaster => "⚡",
            Rank::LifetimeSage => "⏳",
            Rank::ModuleArchitect => "🗂️",
            Rank::Champion => "👑",
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Rank::Initiate => "Initiate",
            Rank::CargoRunner => "Cargo Runner",
            Rank::MemoryKeeper => "Memory Keeper",
            Rank::PatternKnight => "Pattern Knight",
            Rank::ErrorHandler => "Error Handler",
            Rank::CollectionHero => "Collection Hero",
            Rank::TraitMaster => "Trait Master",
            Rank::LifetimeSage => "Lifetime Sage",
            Rank::ModuleArchitect => "Module Architect",
            Rank::Champion => "Rust Quest Champion",
        }
    }
}

pub fn rank_for_completed(completed: &HashSet<String>) -> Rank {
    if completed.len() >= 14 {
        return Rank::Champion;
    }
    if completed.contains("modules_prelude") {
        return Rank::ModuleArchitect;
    }
    if completed.contains("lifetimes") {
        return Rank::LifetimeSage;
    }
    if completed.contains("traits_generics") {
        return Rank::TraitMaster;
    }
    if completed.contains("collections") {
        return Rank::CollectionHero;
    }
    if completed.contains("errors") {
        return Rank::ErrorHandler;
    }
    if completed.contains("structs_enums") {
        return Rank::PatternKnight;
    }
    if completed.contains("ownership") {
        return Rank::MemoryKeeper;
    }
    if completed.contains("cargo") {
        return Rank::CargoRunner;
    }
    Rank::Initiate
}

pub fn xp_bar(xp: u32, width: usize) -> String {
    let capped = xp.min(MAX_XP);
    let filled = ((capped as f64 / MAX_XP as f64) * width as f64).round() as usize;
    let filled = filled.min(width);
    format!("{}{}", "█".repeat(filled), "░".repeat(width - filled))
}
