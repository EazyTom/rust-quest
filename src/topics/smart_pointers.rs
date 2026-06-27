//! Quest 11 — Smart pointers.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use crate::game::quiz::QuizQuestion;
use crate::resources::links::ResourceLinks;
use crate::topics::registry::Quest;

pub fn demo() -> String {
    let mut out = String::new();
    out.push_str("=== Smart Pointers ===\n\n");
    out.push_str(
        "Sometimes plain references are not enough: you need heap allocation, \
         shared ownership, or mutation through a shared reference. \
         *Smart pointers* wrap raw pointers with safe, well-defined behavior.\n\n",
    );

    let b = Box::new(42);
    out.push_str(&format!(
        "Step 1 — Box<T> (single owner on heap)\n  let b = Box::new(42);  → {b}\n  \
         Use when size is unknown at compile time or for recursive types.\n  \
         When b goes out of scope, the heap memory is freed.\n\n",
    ));

    let rc = Rc::new("shared".to_string());
    let rc2 = Rc::clone(&rc);
    out.push_str(&format!(
        "Step 2 — Rc<T> (reference counted, single-threaded sharing)\n  \
         Rc::clone(&rc) increments a counter — does NOT deep-copy the string.\n  \
         strong_count = {} (both rc and rc2 point to same data)\n  \
         Data is dropped when the last Rc goes away.\n\n",
        Rc::strong_count(&rc2)
    ));

    let arc = Arc::new(100);
    out.push_str(&format!(
        "Step 3 — Arc<T> (atomic Rc — thread-safe sharing)\n  \
         Arc::strong_count = {}\n  \
         Use Arc when multiple threads need read-only shared ownership.\n\n",
        Arc::strong_count(&arc)
    ));

    let cell = RefCell::new(0);
    *cell.borrow_mut() += 1;
    out.push_str(&format!(
        "Step 4 — RefCell<T> (interior mutability)\n  \
         borrow_mut() at runtime checks borrow rules (panics if violated).\n  \
         Lets you mutate data through a shared reference when the compiler \
         cannot prove safety statically.\n  \
         Final value = {}\n",
        *cell.borrow()
    ));
    out
}

pub const MEMORY: &str = "Rc/Arc manage shared ownership counts; RefCell checks borrows at runtime for interior mutability.";

static Q1: QuizQuestion = QuizQuestion::new(
    "Box<T> is for…",
    &[
        "Shared thread ownership",
        "Single-owner heap allocation",
        "Async tasks",
        "Modules",
    ],
    1,
    "Simplest smart pointer.",
    "Box owns one value on the heap.",
);

static Q2: QuizQuestion = QuizQuestion::new(
    "Arc vs Rc?",
    &[
        "Identical",
        "Arc is thread-safe reference counting",
        "Rc is for threads",
        "Arc is stack only",
    ],
    1,
    "Arc = Atomic Rc for Send across threads.",
    "Arc uses atomic refcounts safe to share between threads.",
);

static Q3: QuizQuestion = QuizQuestion::new(
    "RefCell allows…",
    &[
        "Unlimited &mut always",
        "Interior mutability with runtime borrow checks",
        "No runtime cost",
        "Bypassing Send",
    ],
    1,
    "Rules checked at runtime if compiler can't prove statically.",
    "RefCell enforces borrow rules dynamically.",
);

static BOSS: QuizQuestion = QuizQuestion::new(
    "Why not Rc<RefCell<T>> across threads?",
    &[
        "Always recommended",
        "Rc is not Send — use Arc<Mutex<T>> instead",
        "RefCell is Send",
        "Threads don't exist",
    ],
    1,
    "Thread safety requires Send + Sync.",
    "Use Arc<Mutex<T>> for shared mutable state across threads.",
);

static LINKS: ResourceLinks = ResourceLinks {
    book: "https://doc.rust-lang.org/book/ch15-00-smart-pointers.html",
    rust_by_example: "https://doc.rust-lang.org/rust-by-example/std/rc.html",
    std_docs: Some("https://doc.rust-lang.org/std/sync/struct.Arc.html"),
    reference: None,
    youtube: &["https://www.youtube.com/watch?v=8M8R0lL4Z3Y"],
};

pub const QUEST: Quest = Quest {
    id: "smart_pointers",
    order: 11,
    emoji: "🧠",
    title: "Smart Pointers",
    demo,
    memory_note: MEMORY,
    questions: &[Q1, Q2, Q3],
    boss: BOSS,
    links: LINKS,
};
