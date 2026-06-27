//! Quest 2 — Types and variables.

use crate::game::quiz::QuizQuestion;
use crate::resources::links::ResourceLinks;
use crate::topics::registry::Quest;

pub fn demo() -> String {
    let mut out = String::new();
    out.push_str("=== Types & Variables ===\n\n");
    out.push_str(
        "Rust is *statically typed*: every value has a type known at compile time. \
         That helps the compiler catch mistakes early. Variables are \
         *immutable by default* — you opt in to mutation with `mut`.\n\n",
    );

    let x: i32 = 42;
    out.push_str(&format!(
        "Step 1 — explicit type annotation\n  let x: i32 = {x};\n  \
         The `: i32` tells the compiler this is a 32-bit signed integer.\n\n",
    ));

    let mut y = 10;
    y += 1;
    out.push_str(&format!(
        "Step 2 — mutable binding\n  let mut y = 10; y += 1;  → y is now {y}\n  \
         Without `mut`, `y += 1` would not compile.\n\n",
    ));

    let z = 5;
    let z = z + 1;
    out.push_str(&format!(
        "Step 3 — shadowing (not the same as mutation)\n  let z = 5; let z = z + 1;  → {z}\n  \
         Shadowing creates a *new* binding with the same name. \
         You can even change the type: `let spaces = \"   \"; let spaces = spaces.len();`\n\n",
    ));

    let tup: (i32, &str) = (1, "hi");
    out.push_str(&format!(
        "Step 4 — tuple groups different types\n  let tup = ({}, \"{}\");\n  \
         Access fields with tup.0 and tup.1 (zero-based).\n\n",
        tup.0, tup.1
    ));

    let arr: [u8; 3] = [1, 2, 3];
    out.push_str(&format!(
        "Step 5 — fixed-size array on the stack\n  let arr: [u8; 3] = {arr:?};\n  \
         `[T; N]` means exactly N elements — length is part of the type. \
         Unlike Vec, arrays cannot grow at runtime.\n",
    ));
    out
}

pub const MEMORY: &str =
    "Fixed-size arrays and integer types prevent silent overflow/coercion surprises.";

static Q1: QuizQuestion = QuizQuestion::new(
    "Which keyword makes a binding mutable?",
    &["const", "mut", "static", "ref"],
    1,
    "Variables are immutable by default in Rust.",
    "mut allows reassignment and in-place mutation.",
);

static Q2: QuizQuestion = QuizQuestion::new(
    "What is shadowing?",
    &[
        "Mutating a variable in place",
        "Creating a new binding with the same name",
        "Copying a reference",
        "A compile error always",
    ],
    1,
    "let x = 1; let x = x + 1; uses two bindings.",
    "Shadowing creates a new variable that hides the old one.",
);

static Q3: QuizQuestion = QuizQuestion::new(
    "[u8; 3] means…",
    &[
        "Slice of 3 bytes",
        "Array of 3 u8 on stack",
        "Vector with 3 items",
        "Tuple of u8",
    ],
    1,
    "Semicolon inside brackets is array length.",
    "[T; N] is a fixed-size array type.",
);

static BOSS: QuizQuestion = QuizQuestion::new(
    "Default immutability helps memory safety because…",
    &[
        "It disables the borrow checker",
        "It limits accidental concurrent mutation",
        "It removes the need for types",
        "It enables garbage collection",
    ],
    1,
    "Think about who can change data and when.",
    "Immutable by default reduces accidental shared mutation bugs.",
);

static LINKS: ResourceLinks = ResourceLinks {
    book: "https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html",
    rust_by_example: "https://doc.rust-lang.org/rust-by-example/variable_bindings.html",
    std_docs: None,
    reference: None,
    youtube: &["https://www.youtube.com/watch?v=jbR7Y7CFTjI"],
};

pub const QUEST: Quest = Quest {
    id: "types",
    order: 2,
    emoji: "🔢",
    title: "Types & Variables",
    demo,
    memory_note: MEMORY,
    questions: &[Q1, Q2, Q3],
    boss: BOSS,
    links: LINKS,
};
