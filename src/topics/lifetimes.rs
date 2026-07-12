//! Quest 8 — Lifetimes.

use crate::game::quiz::QuizQuestion;
use crate::resources::links::ResourceLinks;
use crate::topics::registry::Quest;

struct BookExcerpt<'a> {
    title: &'a str,
}

fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

pub fn demo() -> String {
    let mut out = String::new();
    out.push_str("=== Lifetimes ===\n\n");
    out.push_str(
        "A reference (&T) must not outlive the data it points to. \
         *Lifetime parameters* like `'a` are labels the compiler uses to \
         prove references stay valid — they are mostly compile-time, not runtime.\n\n",
    );

    let s1 = "Rust Quest";
    let s2 = "Ayush";
    let long = longest(s1, s2);
    out.push_str(&format!(
        "Step 1 — shared lifetime on references\n  fn longest<'a>(a: &'a str, b: &'a str) -> &'a str\n  \
         longest(\"{s1}\", \"{s2}\") → \"{long}\"\n  \
         `'a` says: the returned reference lives no longer than BOTH inputs.\n\n",
    ));

    let book = BookExcerpt {
        title: "The Rust Book",
    };
    out.push_str(&format!(
        "Step 2 — struct holding a reference\n  struct BookExcerpt<'a> {{ title: &'a str }}\n  \
         title borrows from a string owned elsewhere: \"{}\"\n  \
         The struct cannot outlive the string it borrows from.\n\n",
        book.title
    ));

    out.push_str(
        "Step 3 — why the compiler cares\n  \
         If a function returned a reference to a local variable, that memory would be \
         freed when the function returns — a *dangling pointer*. Rust rejects this at compile time.\n\n",
    );
    out.push_str(
        "Step 4 — when you write lifetimes\n  \
         Often elided (inferred). You annotate when the compiler needs help \
         linking input and output reference lifetimes — common in functions returning &str.\n",
    );
    out
}

pub const MEMORY: &str = "Lifetime annotations tell the compiler how long references must remain valid — prevents dangling refs.";

static Q1: QuizQuestion = QuizQuestion::new(
    "Lifetime parameter 'a means…",
    &[
        "Reference valid for scope 'a",
        "Allocate on heap",
        "Async runtime",
        "Macro name",
    ],
    0,
    "It links references to their owners.",
    "'a is a generic lifetime tying borrows together.",
);

static Q2: QuizQuestion = QuizQuestion::new(
    "Dangling reference is…",
    &[
        "Safe in Rust",
        "Compile-time error",
        "Runtime GC fix",
        "Only in unsafe",
    ],
    1,
    "Borrow checker rejects invalid lifetimes.",
    "Rust prevents returning refs to dropped data.",
);

static Q3: QuizQuestion = QuizQuestion::new(
    "Struct with &str field usually needs…",
    &[
        "No annotations ever",
        "Lifetime on struct",
        "Box only",
        "Clone derive",
    ],
    1,
    "Reference must not outlive source.",
    "Structs holding references need lifetime parameters.",
);

static BOSS: QuizQuestion = QuizQuestion::new(
    "Why can't this work? fn bad() -> &str { let s = String::from(\"x\"); &s }",
    &[
        "String too short",
        "s dropped at end of fn — returned ref would dangle",
        "Missing mut",
        "Needs async",
    ],
    1,
    "Local variable dies when function returns.",
    "Returning reference to local variable creates dangling reference.",
);

static LINKS: ResourceLinks = ResourceLinks {
    book: "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html",
    rust_by_example: "https://doc.rust-lang.org/rust-by-example/scope/lifetime.html",
    std_docs: None,
    reference: Some("https://doc.rust-lang.org/reference/lifetime-elision.html"),
    youtube: &["https://www.youtube.com/watch?v=J3_like30nyI"],
};

pub const QUEST: Quest = Quest {
    id: "lifetimes",
    order: 8,
    emoji: "⏳",
    title: "Lifetimes",
    demo,
    memory_note: MEMORY,
    questions: &[Q1, Q2, Q3],
    boss: BOSS,
    links: LINKS,
};
