//! Quest 6 — Collections: Vec, String, HashMap.

use std::collections::HashMap;

use crate::game::quiz::QuizQuestion;
use crate::resources::links::ResourceLinks;
use crate::topics::registry::Quest;

pub fn demo() -> String {
    let mut out = String::new();
    out.push_str("=== Collections: Vec, String, HashMap ===\n\n");
    out.push_str(
        "Most programs store lists of things. Rust's standard collections live on the \
         heap and grow as needed. Indexing with `[i]` panics if out of bounds — \
         prefer `.get(i)` when input comes from users.\n\n",
    );

    let mut scores = vec![10, 20, 30];
    scores.push(40);
    out.push_str(&format!(
        "Step 1 — Vec<T> (growable array)\n  let mut scores = vec![10, 20, 30];\n  \
         scores.push(40);  → {scores:?}\n  \
         Vec owns its elements on the heap; `mut` lets you push/pop.\n\n",
    ));

    let s = String::from("rust");
    out.push_str(&format!(
        "Step 2 — String vs &str\n  String::from(\"rust\") owns UTF-8 bytes on the heap.\n  \
         len = {}, slice &s[..] = \"{}\"\n  \
         &str is a borrowed view — often from a String or string literal.\n\n",
        s.len(),
        &s[..]
    ));

    let mut map = HashMap::new();
    map.insert("xp", 100);
    out.push_str(&format!(
        "Step 3 — HashMap (key → value)\n  map.insert(\"xp\", 100);\n  \
         map.get(\"xp\") → {:?}\n  \
         Returns Option<&V> because the key might not exist.\n\n",
        map.get("xp")
    ));

    out.push_str(
        "Step 4 — safe indexing\n  scores.get(99) → None (no crash)\n  \
         scores[99]      → panic at runtime\n  \
         When the index comes from player input, always use .get().\n",
    );
    out
}

pub const MEMORY: &str = "Bounds-checked access on slices/Vec prevents buffer overruns; growable heap storage for Vec/String.";

static Q1: QuizQuestion = QuizQuestion::new(
    "Vec<T> stores data…",
    &[
        "On stack fixed",
        "On heap growable",
        "Only in static",
        "As raw pointers only",
    ],
    1,
    "push/pop change length at runtime.",
    "Vec is a growable heap-allocated array.",
);

static Q2: QuizQuestion = QuizQuestion::new(
    "String vs &str?",
    &[
        "&str owns heap data",
        "String owns; &str is borrowed text slice",
        "They are identical",
        "String is always static",
    ],
    1,
    "One owns UTF-8 bytes, one borrows.",
    "String owns UTF-8 data; &str is an immutable view.",
);

static Q3: QuizQuestion = QuizQuestion::new(
    "HashMap::get returns…",
    &[
        "Option<&V>",
        "V directly always",
        "Result<V,E>",
        "usize index",
    ],
    0,
    "Key might be missing.",
    "get returns Option reference to value if key exists.",
);

static BOSS: QuizQuestion = QuizQuestion::new(
    "Why prefer slice.get(i) over [i] for user input index?",
    &[
        "[i] is faster always",
        "get returns Option instead of panicking",
        "get allocates heap",
        "[i] is deprecated",
    ],
    1,
    "User indices can be out of range.",
    "get avoids panic on invalid index.",
);

static LINKS: ResourceLinks = ResourceLinks {
    book: "https://doc.rust-lang.org/book/ch08-00-common-collections.html",
    rust_by_example: "https://doc.rust-lang.org/rust-by-example/std/vec.html",
    std_docs: Some("https://doc.rust-lang.org/std/collections/struct.HashMap.html"),
    reference: None,
    youtube: &["https://www.youtube.com/watch?v=TFsZy11AK8g"],
};

pub const QUEST: Quest = Quest {
    id: "collections",
    order: 6,
    emoji: "📚",
    title: "Collections",
    demo,
    memory_note: MEMORY,
    questions: &[Q1, Q2, Q3],
    boss: BOSS,
    links: LINKS,
};
