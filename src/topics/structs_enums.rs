//! Quest 4 — Structs, enums, and pattern matching.

use crate::game::quiz::QuizQuestion;
use crate::resources::links::ResourceLinks;
use crate::topics::registry::Quest;

#[derive(Debug, Clone, Copy)]
enum QuestStatus {
    Locked,
    Open,
    Done,
}

pub fn demo() -> String {
    let mut out = String::new();
    out.push_str("=== Structs, Enums & Pattern Matching ===\n\n");
    out.push_str(
        "Structs group related data into one named type. Enums express *one of several* \
         variants — perfect for quest states, errors, or optional values. \
         `match` forces you to handle every variant.\n\n",
    );

    struct Player {
        name: String,
        xp: u32,
    }
    let p = Player {
        name: "Ayush".into(),
        xp: 40,
    };
    out.push_str(&format!(
        "Step 1 — struct (custom data type)\n  struct Player {{ name: String, xp: u32 }}\n  \
         Player {{ name: \"{}\", xp: {} }}\n  \
         Fields are accessed with dot notation: p.name, p.xp\n\n",
        p.name, p.xp
    ));

    let status = QuestStatus::Open;
    let locked = QuestStatus::Locked;
    let done = QuestStatus::Done;
    let label = match status {
        QuestStatus::Locked => "locked",
        QuestStatus::Open => "open",
        QuestStatus::Done => "done",
    };
    out.push_str(&format!(
        "Step 2 — enum + exhaustive match\n  enum QuestStatus {{ Locked, Open, Done }}\n  \
         match Open → \"{label}\"; also Locked and Done exist ({locked:?}, {done:?})\n  \
         The compiler errors if you forget a variant — no silent null crashes.\n\n",
    ));

    if let QuestStatus::Open = status {
        out.push_str(
            "Step 3 — if let (one-pattern sugar)\n  if let QuestStatus::Open = status {{ ... }}\n  \
             Shorthand when you only care about one variant.\n\n",
        );
    }

    out.push_str(
        "Step 4 — Option<T> is an enum too\n  Some(value) means present, None means absent.\n  \
         Rust has no null — you must handle None explicitly with match or if let.\n",
    );
    out
}

pub const MEMORY: &str =
    "match must cover all enum variants — no null pointer surprises at runtime.";

static Q1: QuizQuestion = QuizQuestion::new(
    "Option<T> replaces…",
    &[
        "Exceptions",
        "Nullable pointers without safety",
        "Garbage collection",
        "Macros only",
    ],
    1,
    "Some(T) and None are the two variants.",
    "Option models absence explicitly in the type system.",
);

static Q2: QuizQuestion = QuizQuestion::new(
    "match must be…",
    &["Partial", "Exhaustive", "Runtime only", "Unsafe"],
    1,
    "Compiler checks all variants handled.",
    "Exhaustive matching catches missing cases at compile time.",
);

static Q3: QuizQuestion = QuizQuestion::new(
    "impl block is used to…",
    &[
        "Import modules",
        "Add methods to a type",
        "Start threads",
        "Open files",
    ],
    1,
    "Similar to methods on a type.",
    "impl defines associated functions and methods.",
);

static BOSS: QuizQuestion = QuizQuestion::new(
    "Why is `if let` useful?",
    &[
        "It skips the borrow checker",
        "Concise pattern match for one case",
        "It always panics",
        "Only works on integers",
    ],
    1,
    "Sugar for match with one arm.",
    "if let handles one pattern while ignoring others.",
);

static LINKS: ResourceLinks = ResourceLinks {
    book: "https://doc.rust-lang.org/book/ch06-00-enums.html",
    rust_by_example: "https://doc.rust-lang.org/rust-by-example/custom_types/structs.html",
    std_docs: Some("https://doc.rust-lang.org/std/option/enum.Option.html"),
    reference: None,
    youtube: &["https://www.youtube.com/watch?v=LMMuS4O0pcE"],
};

pub const QUEST: Quest = Quest {
    id: "structs_enums",
    order: 4,
    emoji: "🏗️",
    title: "Structs & Enums",
    demo,
    memory_note: MEMORY,
    questions: &[Q1, Q2, Q3],
    boss: BOSS,
    links: LINKS,
};
