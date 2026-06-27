//! Quest 3 — Ownership and borrowing.

use crate::game::quiz::QuizQuestion;
use crate::resources::links::ResourceLinks;
use crate::topics::registry::Quest;

pub fn demo() -> String {
    let mut out = String::new();
    out.push_str("=== Ownership & Borrowing ===\n\n");
    out.push_str(
        "Rust's core idea: each value has exactly one *owner* at a time. \
         When the owner goes out of scope, memory is freed automatically — \
         no garbage collector, no manual free(). The *borrow checker* enforces \
         these rules at compile time.\n\n",
    );

    let s1 = String::from("hello");
    let s2 = s1;
    out.push_str(&format!(
        "Step 1 — move (transfer ownership)\n  let s1 = String::from(\"hello\");\n  \
         let s2 = s1;  // s1 is MOVED into s2\n  \
         s2 now owns the heap data: \"{s2}\"\n  \
         Using s1 after this would NOT compile (use-after-move).\n\n",
    ));

    let s3 = s2.clone();
    out.push_str(&format!(
        "Step 2 — clone (explicit deep copy)\n  let s3 = s2.clone();\n  \
         Both s2 and s3 own separate copies: \"{s3}\"\n  \
         Clone can be expensive — only use when you need two owners.\n\n",
    ));

    let len = s3.len();
    out.push_str(&format!(
        "Step 3 — shared borrow (&T)\n  let len = s3.len();  // &s3 implicitly\n  \
         len = {len} — we read s3 without taking ownership.\n  \
         Many &T borrows are allowed if nobody has &mut.\n\n",
    ));

    let mut v = vec![1, 2, 3];
    v.push(4);
    out.push_str(&format!(
        "Step 4 — mutable borrow (&mut T)\n  let mut v = vec![1,2,3]; v.push(4);\n  \
         v is now {v:?}\n  \
         Only ONE &mut to data at a time — prevents data races in single-threaded code too.\n\n",
    ));

    let slice = &v[1..];
    out.push_str(&format!(
        "Step 5 — slice borrow\n  let slice = &v[1..];  → {slice:?}\n  \
         A slice is a view into contiguous elements — no copy, just a pointer + length.\n",
    ));
    out
}

pub const MEMORY: &str =
    "Each value has one owner. Moves prevent use-after-free; borrows limit aliasing of &mut.";

static Q1: QuizQuestion = QuizQuestion::new(
    "After `let b = a` where `a: String`, what happens to `a`?",
    &[
        "Still usable",
        "Moved — a is invalid",
        "Automatically cloned",
        "Becomes &str",
    ],
    1,
    "String does not implement Copy.",
    "Ownership transfers; the old binding cannot be used.",
);

static Q2: QuizQuestion = QuizQuestion::new(
    "How many &mut borrows of the same data at once?",
    &["Unlimited", "Exactly one", "Exactly two", "Zero allowed"],
    1,
    "Exclusive mutable access is core to safety.",
    "Only one &mut at a time in a scope (with exceptions later).",
);

static Q3: QuizQuestion = QuizQuestion::new(
    "&T is called…",
    &["A move", "A shared borrow", "A mutable borrow", "A clone"],
    1,
    "Multiple &T allowed if no &mut exists.",
    "&T is an immutable shared reference.",
);

static BOSS: QuizQuestion = QuizQuestion::new(
    "Why won't this compile? `let x = s; println!(\"{s}\")` after move?",
    &[
        "String can't print",
        "Use-after-move of s",
        "Missing semicolon",
        "println is unsafe",
    ],
    1,
    "s was moved into x.",
    "Using s after move violates ownership rules.",
);

static LINKS: ResourceLinks = ResourceLinks {
    book: "https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html",
    rust_by_example: "https://doc.rust-lang.org/rust-by-example/scope/move.html",
    std_docs: Some("https://doc.rust-lang.org/std/primitive.slice.html"),
    reference: None,
    youtube: &["https://www.youtube.com/watch?v=VFIOSWy93H0"],
};

pub const QUEST: Quest = Quest {
    id: "ownership",
    order: 3,
    emoji: "🦀",
    title: "Ownership & Borrowing",
    demo,
    memory_note: MEMORY,
    questions: &[Q1, Q2, Q3],
    boss: BOSS,
    links: LINKS,
};
