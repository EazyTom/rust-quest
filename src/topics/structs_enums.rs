//! Quest 4 — Structs, enums, and pattern matching.
//!
//! LEARN: custom enums (QuestStatus) mirror std enums like Option<T> — same match rules.

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
         variants — perfect for quest states like Locked, Open, or Done. \
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

    // GAME: if let is sugar for match with one arm — reused on Option below.
    if let QuestStatus::Open = status {
        out.push_str(
            "Step 3 — if let (one-pattern sugar)\n  if let QuestStatus::Open = status {{ ... }}\n  \
             Shorthand when you only care about one variant.\n\n",
        );
    }

    // LEARN: Option<T> is a built-in enum — preview here; the Errors quest goes deeper.
    let bonus_xp: Option<u32> = Some(25);
    let no_bonus: Option<u32> = None;
    let bonus_label = match bonus_xp {
        Some(xp) => format!("bonus +{xp} XP"),
        None => "no bonus".to_string(),
    };
    out.push_str(&format!(
        "Step 4 — Option<T> preview (enum in the standard library)\n  \
         Rust has no null. Instead, Option<T> is an enum with two variants:\n  \
           • Some(value) — a value is present\n  \
           • None       — no value (like \"missing loot\")\n  \
         let bonus_xp: Option<u32> = Some(25);\n  \
         let no_bonus: Option<u32> = None;\n  \
         match bonus_xp → \"{bonus_label}\"; no_bonus = {no_bonus:?}\n\n",
    ));

    if let Some(xp) = bonus_xp {
        out.push_str(&format!(
            "Step 5 — same patterns on Option\n  if let Some(xp) = bonus_xp → xp is {xp}\n  \
             match must handle both Some and None — same rule as QuestStatus.\n  \
             (The Errors quest teaches Result and ?; Collections uses Option from HashMap::get.)\n",
        ));
    }
    out
}

pub const MEMORY: &str =
    "Enums hold one variant at a time — match (or if let) must cover every case, \
     including Option's Some and None.";

static Q1: QuizQuestion = QuizQuestion::new(
    "What does an enum like QuestStatus represent?",
    &[
        "Many variants at the same time",
        "Exactly one of several variants",
        "Only numeric literals",
        "A module import path",
    ],
    1,
    "A value is Locked, Open, or Done — never all three.",
    "An enum value is always one variant, e.g. QuestStatus::Open.",
);

static Q2: QuizQuestion = QuizQuestion::new(
    "Option<T> has which two variants?",
    &["Ok and Err", "Some and None", "True and False", "Left and Right"],
    1,
    "Step 4 in Learn: Some(value) when present, None when absent.",
    "Option<T> is an enum: Some(T) or None — Rust's replacement for null.",
);

static Q3: QuizQuestion = QuizQuestion::new(
    "match on an enum must be…",
    &["Partial", "Exhaustive", "Runtime only", "Unsafe"],
    1,
    "Compiler checks all variants handled.",
    "Exhaustive matching catches missing cases at compile time.",
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
