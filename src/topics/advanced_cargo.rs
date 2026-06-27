//! Quest 14 — Advanced Cargo: features and workspaces.

use crate::game::quiz::QuizQuestion;
use crate::resources::links::ResourceLinks;
use crate::topics::registry::Quest;

pub fn demo() -> String {
    let mut out = String::new();
    out.push_str("=== Advanced Cargo ===\n\n");
    out.push_str(
        "As projects grow, Cargo.toml grows with them: optional features, \
         test-only dependencies, and workspaces with multiple crates. \
         These keep release builds lean while staying flexible in development.\n\n",
    );
    out.push_str(
        "Step 1 — [features] (optional compile-time flags)\n  \
         [features]\n  \
         serde = [\"dep:serde\"]\n  \
         Enable with: cargo build --features serde\n  \
         Use cfg!(feature = \"serde\") or #[cfg(feature = \"serde\")] in code.\n\n",
    );
    out.push_str(
        "Step 2 — [dev-dependencies]\n  \
         Crates used only for tests and examples — NOT shipped in release binaries.\n  \
         Rust Quest's test deps stay out of the player-facing binary.\n\n",
    );
    out.push_str(
        "Step 3 — workspaces\n  \
         [workspace] members = [\"crate-a\", \"crate-b\"]\n  \
         One repo, shared target/ folder, coordinated versions.\n\n",
    );
    out.push_str(
        "Step 4 — profiles\n  \
         [profile.release] opt-level = 3  — faster/smaller release builds\n  \
         cargo build --release uses the release profile.\n\n",
    );
    out.push_str(
        "Step 5 — you made it!\n  \
         This is the final quest. You now have a map of Rust from Cargo basics \
         to advanced project layout. Keep the book links handy and build something real.\n",
    );
    out
}

pub const MEMORY: &str =
    "Feature flags compile code conditionally — keep default builds lean and explicit.";

static Q1: QuizQuestion = QuizQuestion::new(
    "[dev-dependencies] are for…",
    &[
        "Production binaries",
        "Tests and examples only",
        "docs.rs only",
        "Git hooks",
    ],
    1,
    "Not linked into release binary by default.",
    "dev-dependencies support testing without bloating releases.",
);

static Q2: QuizQuestion = QuizQuestion::new(
    "Cargo workspace is…",
    &[
        "A single file crate",
        "Multiple related crates in one repo",
        "A Docker container",
        "A Rust edition",
    ],
    1,
    "Root Cargo.toml with [workspace] members.",
    "Workspaces share target/ and lockfile across crates.",
);

static Q3: QuizQuestion = QuizQuestion::new(
    "Optional feature in Cargo.toml enables…",
    &[
        "Runtime plugins",
        "Conditional compilation via cfg",
        "Automatic docs",
        "Git push",
    ],
    1,
    "#[cfg(feature = \"...\")] on modules.",
    "Features gate code at compile time.",
);

static BOSS: QuizQuestion = QuizQuestion::new(
    "Why use default = [] for optional features?",
    &[
        "Required by Rust",
        "Minimal default build; users opt in",
        "Disables borrow checker",
        "Speeds up tests always",
    ],
    1,
    "Explicit is better for compile times.",
    "Empty default keeps baseline builds fast and simple.",
);

static LINKS: ResourceLinks = ResourceLinks {
    book: "https://doc.rust-lang.org/book/ch14-00-more-about-cargo.html",
    rust_by_example: "https://doc.rust-lang.org/cargo/reference/features.html",
    std_docs: None,
    reference: Some("https://doc.rust-lang.org/cargo/reference/workspaces.html"),
    youtube: &["https://www.youtube.com/watch?v=BfC0E1Xx3n0"],
};

pub const QUEST: Quest = Quest {
    id: "advanced_cargo",
    order: 14,
    emoji: "🚀",
    title: "Advanced Cargo",
    demo,
    memory_note: MEMORY,
    questions: &[Q1, Q2, Q3],
    boss: BOSS,
    links: LINKS,
};
