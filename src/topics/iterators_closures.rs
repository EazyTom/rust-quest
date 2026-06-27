//! Quest 10 — Iterators and closures.

use crate::game::quiz::QuizQuestion;
use crate::resources::links::ResourceLinks;
use crate::topics::registry::Quest;

pub fn demo() -> String {
    let mut out = String::new();
    out.push_str("=== Iterators & Closures ===\n\n");
    out.push_str(
        "Instead of manual for-loops with indices, Rust encourages *iterators*: \
         lazy pipelines that transform data step by step. A *closure* is a small \
         anonymous function that can capture variables from its surroundings.\n\n",
    );

    let nums = [1, 2, 3, 4];
    let doubled: Vec<_> = nums.iter().map(|x| x * 2).collect();
    out.push_str(&format!(
        "Step 1 — map with a closure\n  [1,2,3,4].iter().map(|x| x * 2).collect()\n  \
         → {doubled:?}\n  \
         `|x| x * 2` is a closure — like lambda x: x*2 in other languages.\n\n",
    ));

    let sum: i32 = nums.iter().filter(|x| **x % 2 == 0).sum();
    out.push_str(&format!(
        "Step 2 — filter + sum (terminal adapter)\n  \
         filter keeps evens, sum adds them → {sum}\n  \
         `filter` is lazy; `sum` consumes the iterator and produces one value.\n\n",
    ));

    let add_one = |n: i32| n + 1;
    out.push_str(&format!(
        "Step 3 — closure stored in a variable\n  let add_one = |n| n + 1;\n  \
         add_one(5) = {}\n  \
         Closures can capture their environment (not shown here, but powerful).\n\n",
        add_one(5)
    ));

    out.push_str(
        "Step 4 — lazy until you collect\n  \
         .map().filter().take() build a pipeline; nothing runs until .collect(), \
         .sum(), or a for-loop consumes it. Often as fast as a hand-written loop.\n",
    );
    out
}

pub const MEMORY: &str =
    "Iterator chains often compile to tight loops — zero-cost abstraction when optimized.";

static Q1: QuizQuestion = QuizQuestion::new(
    "|x| x + 1 is…",
    &[
        "A macro",
        "A closure",
        "A struct literal",
        "An unsafe block",
    ],
    1,
    "Short anonymous function syntax.",
    "Closures capture environment and can be passed to iterators.",
);

static Q2: QuizQuestion = QuizQuestion::new(
    "iter().map(...).collect()…",
    &[
        "Runs at compile time only",
        "Builds lazy iterator then materializes Vec",
        "Always panics",
        "Requires async",
    ],
    1,
    "map is lazy until terminal collect.",
    "Adapters are lazy; collect consumes the iterator.",
);

static Q3: QuizQuestion = QuizQuestion::new(
    "filter keeps elements where…",
    &[
        "Predicate returns true",
        "Always first half",
        "Index is odd",
        "Value is Copy",
    ],
    0,
    "Closure returns bool per item.",
    "filter selects items matching the predicate.",
);

static BOSS: QuizQuestion = QuizQuestion::new(
    "fold vs map — fold…",
    &[
        "Only maps each item",
        "Accumulates a single value",
        "Requires HashMap",
        "Is unsafe",
    ],
    1,
    "Think reduce/aggregate.",
    "fold combines all elements into one accumulator value.",
);

static LINKS: ResourceLinks = ResourceLinks {
    book: "https://doc.rust-lang.org/book/ch13-00-functional-features.html",
    rust_by_example: "https://doc.rust-lang.org/rust-by-example/trait/iter.html",
    std_docs: Some("https://doc.rust-lang.org/std/iter/trait.Iterator.html"),
    reference: None,
    youtube: &["https://www.youtube.com/watch?v=rQWVPQqPy8A"],
};

pub const QUEST: Quest = Quest {
    id: "iterators_closures",
    order: 10,
    emoji: "🔄",
    title: "Iterators & Closures",
    demo,
    memory_note: MEMORY,
    questions: &[Q1, Q2, Q3],
    boss: BOSS,
    links: LINKS,
};
