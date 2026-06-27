//! Encouragement messages and rank-up copy.

use crate::game::xp::Rank;

pub fn quiz_fail() -> &'static str {
    "Not quite — read the explanation, check the book link, and try again!"
}

pub fn quiz_pass() -> &'static str {
    "Quest challenge cleared! Great work, Rustacean!"
}

pub fn rank_up(rank: Rank) -> String {
    format!(
        "Rank up! You are now {} {} — {}",
        rank.emoji(),
        rank.title(),
        "keep going!"
    )
}

pub fn session_quote() -> &'static str {
    const QUOTES: &[&str] = &[
        "Every compile error is the borrow checker teaching you.",
        "Read the source — this game is written in the Rust you are learning.",
        "One quest a day beats cramming fourteen in a night.",
    ];
    let idx = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0) as usize)
        % QUOTES.len();
    QUOTES[idx]
}
