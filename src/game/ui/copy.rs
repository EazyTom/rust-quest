//! Encouragement messages and rank-up copy — retro MUD / dungeon-master voice.
//!
//! GAME: keep strings here so hub/narrative stay free of hard-coded player-facing prose.

use crate::game::xp::Rank;

pub fn quiz_fail() -> &'static str {
    "The foe stands — study the scrolls and strike again!"
}

pub fn book_chapter_prompt(quest_title: &str) -> String {
    format!("📖 Open The Rust Book chapter for {quest_title}?")
}

pub fn next_quest_guidance(quest_emoji: &str, quest_title: &str, room_name: &str) -> String {
    format!(
        "🧭 The path opens to {quest_emoji} {quest_title} — {room_name}. Onward!"
    )
}

pub fn all_quests_cleared() -> &'static str {
    "Every quest room is cleared — revisit the map or train in the hub."
}

pub fn quiz_pass() -> &'static str {
    "🍺🍺 Foe vanquished! The path forward lies open."
}

pub fn rank_up(rank: Rank) -> String {
    format!(
        "⭐ Rank ascends! You are now {} {} — {}",
        rank.emoji(),
        rank.title(),
        "the dungeon trembles at your lore!"
    )
}

pub fn session_quote() -> &'static str {
    const QUOTES: &[&str] = &[
        "The Dungeon Master whispers: every compile error is a clue.",
        "🕯️ Torchlight flickers on source code — read what you play.",
        "One room per vigil beats sprinting fourteen in a night.",
        "The borrow checker is a warden — learn its law, earn trust.",
        "📜 Scrolls of the Rust Book lie in every quest — consult them.",
        "🎲 Roll for wisdom: one quest, one lesson, one victory.",
    ];
    let idx = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0) as usize)
        % QUOTES.len();
    QUOTES[idx]
}

/// Lines for the quit farewell box (player name on the first row).
pub fn farewell_lines(name: &str) -> [String; 3] {
    [
        format!("Fare thee well, {name}!"),
        "May thy journey through Rust".to_string(),
        "be swift, safe, and legendary.".to_string(),
    ]
}

pub fn quest_cleared(enemy_name: &str) -> String {
    format!("Room cleared — {enemy_name} is defeated!")
}

pub fn quest_locked() -> &'static str {
    "The door is barred — best the prior foe first, adventurer."
}

pub fn learn_complete() -> &'static str {
    "📜 Runes committed to memory. XP earned."
}

pub fn learn_already() -> &'static str {
    "These scrolls are already etched — no extra XP."
}

pub fn challenge_already() -> &'static str {
    "This foe already fell — the way lies open."
}

pub fn challenge_paused() -> &'static str {
    "You withdraw — the foe waits, patient and cruel."
}

pub fn wrong_answer_hint() -> &'static str {
    "💡 Glancing blow! The Dungeon Master offers a hint…"
}

pub fn heart_gained() -> &'static str {
    "❤️ A true strike — warmth returns to thy veins!"
}

pub fn heart_lost(current: u32) -> String {
    if current == 0 {
        "💔 Thy strength fails — the healers' potions run dry!".to_string()
    } else {
        format!("💔 A wound lands — {current} heart(s) remain.")
    }
}

pub fn hearts_depleted() -> &'static str {
    "🧪 Thou art weakening! Study runes (Learn), open book chapters (Resources), or read \
     quest scrolls for a healing potion — then return to face the foe."
}

pub fn too_weakened_to_fight() -> &'static str {
    "🧪 The Dungeon Master stays thy hand — thou art too weak to fight. \
     Study runes (Learn) or consult Resources / book chapters for a potion, then strike again."
}

pub fn lore_potion() -> &'static str {
    "🧪 The healers brew a lore potion from the scrolls — one heart restored!"
}

pub fn resource_potion() -> &'static str {
    "📖 Thy reading distills a healing draught from the scrolls — one heart restored!"
}

pub fn memory_safety_header() -> &'static str {
    "💡 Warden's warning (memory safety):"
}

/// First visit — invite the player to save the Kingdom of Rust.
pub fn hub_welcome_lines(name: &str) -> [String; 3] {
    [
        format!("Hail, {name}! The Kingdom of Rust needs you."),
        "Shadows of bad code threaten the realm — only lore and courage restore it.".to_string(),
        "Open the Quest Map and begin your legend!".to_string(),
    ]
}

/// Returning adventurer — point toward the next unfinished quest.
pub fn hub_quest_guidance_lines(
    name: &str,
    quest_emoji: &str,
    quest_title: &str,
    room_name: &str,
    enemy_emoji: &str,
    enemy_name: &str,
    study_first: bool,
) -> [String; 3] {
    let action = if study_first {
        format!("💡 Study the runes, then face {enemy_emoji} {enemy_name}.")
    } else {
        format!("⚔️ Face {enemy_emoji} {enemy_name} in the quiz!")
    };
    [
        format!("Welcome back, {name}. The kingdom still calls."),
        format!("🧭 Next: {quest_emoji} {quest_title} — {room_name}."),
        action,
    ]
}
