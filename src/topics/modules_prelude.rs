//! Quest 9 — Modules, paths, and the prelude.

use crate::game::quiz::QuizQuestion;
use crate::resources::links::ResourceLinks;
use crate::topics::registry::Quest;

pub fn demo() -> String {
    let mut out = String::new();
    out.push_str("=== Modules, Paths & the Prelude ===\n\n");
    out.push_str(
        "Large programs split into *modules* — files and folders that control \
         what is public (`pub`) and what stays private. Paths tell the compiler \
         where to find each name.\n\n",
    );
    out.push_str(
        "Step 1 — module tree\n  mod game; mod topics;  (see src/lib.rs)\n  \
         Each folder can have mod.rs declaring submodules.\n  \
         Rust Quest layout: game/ (engine), topics/ (lessons), resources/ (links).\n\n",
    );
    out.push_str(
        "Step 2 — visibility\n  pub fn run_hub()  → callable from outside the module\n  \
         fn helper()       → private to this module only\n  \
         Privacy is enforced at compile time, not by convention.\n\n",
    );
    out.push_str(
        "Step 3 — use declarations\n  use std::collections::HashMap;\n  \
         Brings a path into scope so you write HashMap instead of the full path.\n  \
         use crate::game::state::GameState;  — `crate::` starts at this package root.\n\n",
    );
    out.push_str(
        "Step 4 — the standard prelude\n  \
         Rust auto-imports common items (Vec, Option, Result, println!, etc.)\n  \
         via std::prelude::v1 — that is why you never write `use std::vec::Vec`.\n\n",
    );
    out.push_str(
        "Step 5 — try it yourself\n  \
         Open src/game/hub.rs and trace `use crate::...` lines to see how \
         the game wires modules together.\n",
    );
    out
}

pub const MEMORY: &str = "Visibility is enforced at compile time — private items cannot be accessed from outside their module.";

static Q1: QuizQuestion = QuizQuestion::new(
    "pub fn means…",
    &[
        "Private function",
        "Public to parent modules",
        "Public API item",
        "Unsafe fn",
    ],
    2,
    "Other modules can use pub items.",
    "pub exports an item from its module.",
);

static Q2: QuizQuestion = QuizQuestion::new(
    "The prelude is…",
    &[
        "A Python import",
        "Auto-imported common std items",
        "A Cargo feature",
        "Only for tests",
    ],
    1,
    "You rarely write use std::vec::Vec manually.",
    "prelude::v1 brings common types into scope.",
);

static Q3: QuizQuestion = QuizQuestion::new(
    "crate:: in source refers to…",
    &[
        "External crate",
        "This package root",
        "Standard library",
        "Macro expansion",
    ],
    1,
    "Paths start at lib.rs or main.rs module root.",
    "crate:: is the path from current crate root.",
);

static BOSS: QuizQuestion = QuizQuestion::new(
    "Why split code into modules?",
    &[
        "Required for borrow checker",
        "Organization, privacy, and reuse",
        "Replaces Cargo.toml",
        "Only for binaries",
    ],
    1,
    "Rust Quest uses game/, topics/, resources/.",
    "Modules organize code and control visibility.",
);

static LINKS: ResourceLinks = ResourceLinks {
    book: "https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html",
    rust_by_example: "https://doc.rust-lang.org/rust-by-example/mod.html",
    std_docs: Some("https://doc.rust-lang.org/std/prelude/index.html"),
    reference: None,
    youtube: &["https://www.youtube.com/watch?v=5C_HPTJg5ek"],
};

pub const QUEST: Quest = Quest {
    id: "modules_prelude",
    order: 9,
    emoji: "🗂️",
    title: "Modules & Prelude",
    demo,
    memory_note: MEMORY,
    questions: &[Q1, Q2, Q3],
    boss: BOSS,
    links: LINKS,
};
