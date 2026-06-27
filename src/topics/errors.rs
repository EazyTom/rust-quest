//! Quest 5 — Error handling with Option and Result.

use crate::game::quiz::QuizQuestion;
use crate::resources::links::ResourceLinks;
use crate::topics::registry::Quest;

fn parse_level(s: &str) -> Result<u32, &'static str> {
    match s.parse::<u32>() {
        Ok(n) if n > 0 => Ok(n),
        Ok(_) => Err("level must be positive"),
        Err(_) => Err("not a number"),
    }
}

pub fn demo() -> String {
    let mut out = String::new();
    out.push_str("=== Errors: Result & Option ===\n\n");
    out.push_str(
        "Rust has no exceptions. Recoverable failures are ordinary values: \
         `Result<T, E>` for success/error, `Option<T>` for present/absent. \
         You must *handle* them — the compiler will not let you ignore errors silently.\n\n",
    );

    match parse_level("5") {
        Ok(n) => out.push_str(&format!(
            "Step 1 — Ok branch\n  parse_level(\"5\") → Ok({n})\n  \
             `match` or `if let Ok(n) = ...` extracts the value safely.\n\n",
        )),
        Err(e) => out.push_str(&format!("Step 1 — unexpected Err: {e}\n\n")),
    }
    match parse_level("nope") {
        Ok(n) => out.push_str(&format!("Step 2 — Ok {n}\n\n")),
        Err(e) => out.push_str(&format!(
            "Step 2 — Err branch (expected for bad input)\n  parse_level(\"nope\") → Err(\"{e}\")\n  \
             Your code decides: retry, show a message, or return the error upward.\n\n",
        )),
    }
    let maybe: Option<u32> = Some(10);
    out.push_str(&format!(
        "Step 3 — Option transforms\n  Some(10).map(|v| v * 2) → {:?}\n  \
         `map` applies a function only when the value is Some.\n\n",
        maybe.map(|v| v * 2)
    ));
    out.push_str(
        "Step 4 — the ? operator (you will use this constantly)\n  \
         Inside a fn returning Result, `?` means:\n  \
           • if Ok(v)  → unwrap v and continue\n  \
           • if Err(e) → return Err(e) immediately\n\n",
    );
    out.push_str(
        "Step 5 — avoid unwrap() in real code\n  \
         `unwrap()` and `expect()` panic (crash) on Err/None.\n  \
         Fine for prototypes and tests — use match or ? in production paths.\n",
    );
    out
}

pub const MEMORY: &str =
    "Errors are values (Result/Option) — you must handle or propagate them; no hidden exceptions.";

static Q1: QuizQuestion = QuizQuestion::new(
    "Result<T, E> has variants…",
    &["Some/None", "Ok/Err", "Left/Right", "True/False"],
    1,
    "Think success vs failure.",
    "Ok holds success value, Err holds error.",
);

static Q2: QuizQuestion = QuizQuestion::new(
    "The ? operator…",
    &[
        "Panics on error",
        "Propagates Err to caller",
        "Clones the value always",
        "Only works on Option",
    ],
    1,
    "Syntactic sugar for early return.",
    "? returns Err early from functions returning Result.",
);

static Q3: QuizQuestion = QuizQuestion::new(
    "unwrap() on Err…",
    &[
        "Returns default",
        "Panics at runtime",
        "Compiles but warns",
        "Converts to Ok",
    ],
    1,
    "Demo labeled this as bad practice.",
    "unwrap panics — use match or ? in production code.",
);

static BOSS: QuizQuestion = {
    let mut q = QuizQuestion::new(
        "Best way to handle recoverable file-not-found?",
        &[
            "unwrap()",
            "match or ? on Result",
            "ignore it",
            "panic! always",
        ],
        1,
        "Recoverable errors should not crash the game.",
        "Explicit Result handling keeps control flow visible.",
    );
    q.is_bad_unwrap_choice = true;
    q
};

static LINKS: ResourceLinks = ResourceLinks {
    book: "https://doc.rust-lang.org/book/ch09-00-error-handling.html",
    rust_by_example: "https://doc.rust-lang.org/rust-by-example/error.html",
    std_docs: Some("https://doc.rust-lang.org/std/result/enum.Result.html"),
    reference: None,
    youtube: &["https://www.youtube.com/watch?v=7aoNZ9M6xCE"],
};

pub const QUEST: Quest = Quest {
    id: "errors",
    order: 5,
    emoji: "⚠️",
    title: "Errors & Result",
    demo,
    memory_note: MEMORY,
    questions: &[Q1, Q2, Q3],
    boss: BOSS,
    links: LINKS,
};
