//! Quest 12 — Concurrency.

use std::sync::{Arc, Mutex};
use std::thread;

use crate::game::quiz::QuizQuestion;
use crate::resources::links::ResourceLinks;
use crate::topics::registry::Quest;

pub fn demo() -> String {
    let mut out = String::new();
    out.push_str("=== Concurrency ===\n\n");
    out.push_str(
        "Rust threads share memory carefully. The type system marks types as \
         Send (safe to move to another thread) and Sync (safe to share via reference). \
         Data races are prevented at compile time — or serialized with Mutex.\n\n",
    );

    let counter = Arc::new(Mutex::new(0));
    let c = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut n = c.lock().unwrap();
        *n += 1;
    });
    let _ = handle.join();
    out.push_str(&format!(
        "Step 1 — thread + Arc<Mutex<T>>\n  \
         Arc = shared ownership across threads\n  \
         Mutex = only one thread mutates at a time\n  \
         After spawn increments: counter = {}\n\n",
        *counter.lock().unwrap()
    ));

    out.push_str(
        "Step 2 — move into the thread\n  \
         thread::spawn(move || ...) takes ownership of captured variables.\n  \
         We clone the Arc (cheap — just bumps a counter), not the Mutex inner data.\n\n",
    );
    out.push_str(
        "Step 3 — join waits for completion\n  \
         handle.join() blocks until the thread finishes — avoids use-after-free.\n\n",
    );
    out.push_str(
        "Step 4 — why not plain &mut from two threads?\n  \
         The borrow checker forbids it. Arc<Mutex> is the standard pattern \
         for shared mutable state between threads.\n",
    );
    out
}

pub const MEMORY: &str =
    "Rust's type system prevents data races: Send/Sync + Mutex/Arc patterns for shared state.";

static Q1: QuizQuestion = QuizQuestion::new(
    "Mutex<T> ensures…",
    &[
        "Parallel &mut from all threads",
        "One thread mutates at a time",
        "No locking",
        "GC pauses",
    ],
    1,
    "Lock before accessing inner data.",
    "Mutex provides mutual exclusion for shared data.",
);

static Q2: QuizQuestion = QuizQuestion::new(
    "Arc allows…",
    &[
        "Multiple owners across threads",
        "Only stack sharing",
        "Replacing borrow checker",
        "Single-thread only",
    ],
    0,
    "Atomic reference counting.",
    "Arc shares ownership of heap data between threads.",
);

static Q3: QuizQuestion = QuizQuestion::new(
    "Data race means…",
    &[
        "Two threads read only",
        "Concurrent access with at least one write without sync",
        "Using Arc",
        "Compiling with cargo",
    ],
    1,
    "Undefined behavior in many languages; error in Rust.",
    "Unsynchronized concurrent mutation is a data race.",
);

static BOSS: QuizQuestion = QuizQuestion::new(
    "Why is Rc<Mutex<i32>> not Send to another thread?",
    &[
        "Mutex blocks Send",
        "Rc refcount is not thread-safe",
        "i32 is too small",
        "Threads require async",
    ],
    1,
    "Use Arc for cross-thread sharing.",
    "Rc is single-threaded; use Arc for thread-safe sharing.",
);

static LINKS: ResourceLinks = ResourceLinks {
    book: "https://doc.rust-lang.org/book/ch16-00-concurrency.html",
    rust_by_example: "https://doc.rust-lang.org/rust-by-example/std_misc/threads.html",
    std_docs: Some("https://doc.rust-lang.org/std/thread/"),
    reference: None,
    youtube: &["https://www.youtube.com/watch?v=7aoNZ9M6xCE"],
};

pub const QUEST: Quest = Quest {
    id: "concurrency",
    order: 12,
    emoji: "🧵",
    title: "Concurrency",
    demo,
    memory_note: MEMORY,
    questions: &[Q1, Q2, Q3],
    boss: BOSS,
    links: LINKS,
};
