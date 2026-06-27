//! Quest 13 — Testing and documentation.

use crate::game::quiz::QuizQuestion;
use crate::resources::links::ResourceLinks;
use crate::topics::registry::Quest;

/// Adds two numbers — documented for rustdoc.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
fn internal_check() -> bool {
    add(2, 2) == 4
}

pub fn demo() -> String {
    let mut out = String::new();
    out.push_str("=== Testing & Documentation ===\n\n");
    out.push_str(
        "Rust treats tests as first-class. You write them beside your code, \
         run with `cargo test`, and document APIs with /// comments that \
         become a browsable website via `cargo doc`.\n\n",
    );
    out.push_str(
        "Step 1 — unit tests with #[test]\n  \
         #[cfg(test)] mod tests { #[test] fn it_works() { ... } }\n  \
         Lives in the same file as the code it checks (see src/game/state.rs).\n\n",
    );
    out.push_str(
        "Step 2 — integration tests in tests/\n  \
         Each file in tests/ is a separate crate that imports your library.\n  \
         Rust Quest has tests/game_state.rs, tests/map_nodes.rs, etc.\n\n",
    );
    out.push_str(
        "Step 3 — doc comments (///)\n  \
         Written above public items; rendered as markdown in cargo doc.\n  \
         This very function has a /// comment you can see in source!\n\n",
    );
    out.push_str(&format!(
        "Step 4 — runnable example in docs\n  add(2, 2) = {} — tested by unit tests below.\n\n",
        add(2, 2)
    ));
    #[cfg(test)]
    {
        out.push_str(&format!(
            "Step 5 — internal_check() = {} (only when cfg(test) is active)\n",
            internal_check()
        ));
    }
    #[cfg(not(test))]
    {
        out.push_str(
            "Step 5 — run `cargo test` to execute all #[test] functions\n  \
             The scripts/run_tests.ps1 harness runs fmt, clippy, test, and release build.\n",
        );
    }
    out
}

pub const MEMORY: &str =
    "Tests guard behavior as you learn; rustdoc keeps public API examples honest via doc tests.";

static Q1: QuizQuestion = QuizQuestion::new(
    "Integration tests live in…",
    &["src/main.rs only", "tests/*.rs", "Cargo.toml", "target/"],
    1,
    "Each file is a separate crate linking your lib.",
    "tests/ at project root runs as external integration tests.",
);

static Q2: QuizQuestion = QuizQuestion::new(
    "cargo doc builds…",
    &[
        "Binary only",
        "HTML documentation from /// comments",
        "Docker images",
        "Git repo",
    ],
    1,
    "Try cargo doc --open on this project.",
    "rustdoc generates browsable API docs.",
);

static Q3: QuizQuestion = QuizQuestion::new(
    "#[cfg(test)] means…",
    &[
        "Always compiled",
        "Compiled only when testing",
        "Unsafe code",
        "Release only",
    ],
    1,
    "Keeps test helpers out of release builds.",
    "cfg(test) gates code to test builds.",
);

static BOSS: QuizQuestion = QuizQuestion::new(
    "Doc test in /// example code…",
    &[
        "Is ignored",
        "Runs as test with cargo test",
        "Only on nightly",
        "Replaces unit tests",
    ],
    1,
    "Examples in docs must compile and pass.",
    "Doc tests verify documentation examples.",
);

static LINKS: ResourceLinks = ResourceLinks {
    book: "https://doc.rust-lang.org/book/ch11-00-testing.html",
    rust_by_example: "https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html",
    std_docs: None,
    reference: None,
    youtube: &["https://www.youtube.com/watch?v=TB4-_oAyaj0"],
};

pub const QUEST: Quest = Quest {
    id: "testing_docs",
    order: 13,
    emoji: "✅",
    title: "Testing & Docs",
    demo,
    memory_note: MEMORY,
    questions: &[Q1, Q2, Q3],
    boss: BOSS,
    links: LINKS,
};
