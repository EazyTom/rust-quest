//! Quest 1 — Cargo workflow.

use crate::game::quiz::QuizQuestion;
use crate::resources::links::ResourceLinks;
use crate::topics::registry::Quest;

pub fn demo() -> String {
    let mut out = String::new();
    out.push_str("=== Cargo: Rust's Project Tool ===\n\n");
    out.push_str(
        "Every Rust program lives in a *crate* managed by Cargo. \
         You rarely call the compiler (rustc) directly — Cargo handles \
         dependencies, builds, tests, and docs for you.\n\n",
    );
    out.push_str("--- Key files ---\n");
    out.push_str("  Cargo.toml  — project manifest (name, version, dependencies)\n");
    out.push_str("  src/main.rs — binary entry point (programs with fn main)\n");
    out.push_str("  src/lib.rs  — library root (what other crates import)\n");
    out.push_str("  target/     — build output (gitignored; safe to delete)\n\n");
    out.push_str("--- Commands you will use daily ---\n");
    out.push_str("  cargo build   — compile; first run downloads deps from crates.io\n");
    out.push_str("  cargo run     — build + run the default binary\n");
    out.push_str("  cargo check   — type-check only (faster than full build)\n");
    out.push_str("  cargo test    — run unit, integration, and doc tests\n");
    out.push_str("  cargo doc     — build API documentation from /// comments\n\n");
    out.push_str("--- Why this matters ---\n");
    out.push_str(
        "Rust Quest itself is a Cargo project: `cargo run` starts the game, \
         `cargo test` runs the test harness in scripts/run_tests.ps1. \
         When the compiler rejects code, fix errors before `cargo run` succeeds — \
         that is Rust catching bugs before players ever see them.\n",
    );
    out
}

pub const MEMORY: &str =
    "Rust checks your code at compile time before run — many bugs never reach runtime.";

static Q1: QuizQuestion = QuizQuestion::new(
    "Which file declares crate name and dependencies?",
    &["main.rs", "Cargo.toml", "README.md", "lib.rs"],
    1,
    "Look for [package] and [dependencies].",
    "Cargo.toml is the manifest for every Rust project.",
);

static Q2: QuizQuestion = QuizQuestion::new(
    "Which command runs all tests?",
    &["cargo run", "cargo check", "cargo test", "cargo doc"],
    2,
    "Tests live in #[cfg(test)] and tests/ folder.",
    "cargo test runs unit, integration, and doc tests.",
);

static Q3: QuizQuestion = QuizQuestion::new(
    "Where do release binaries go by default?",
    &["src/", "target/", ".rust-test/", "examples/"],
    1,
    "Build artifacts stay in one gitignored folder.",
    "target/ holds debug and release builds.",
);

static BOSS: QuizQuestion = QuizQuestion::new(
    "What happens if cargo check fails?",
    &[
        "Program runs anyway",
        "Compile errors must be fixed first",
        "Tests auto-fix code",
        "Only warnings appear",
    ],
    1,
    "check compiles without linking a full binary.",
    "You fix compile errors before cargo run succeeds.",
);

static LINKS: ResourceLinks = ResourceLinks {
    book: "https://doc.rust-lang.org/book/ch01-03-hello-cargo.html",
    rust_by_example: "https://doc.rust-lang.org/rust-by-example/cargo.html",
    std_docs: None,
    reference: None,
    youtube: &["https://www.youtube.com/watch?v=BfC0E1Xx3n0"],
};

pub const QUEST: Quest = Quest {
    id: "cargo",
    order: 1,
    emoji: "📦",
    title: "Cargo Workflow",
    demo,
    memory_note: MEMORY,
    questions: &[Q1, Q2, Q3],
    boss: BOSS,
    links: LINKS,
};
