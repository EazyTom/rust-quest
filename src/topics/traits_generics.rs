//! Quest 7 — Traits and generics.

use crate::game::quiz::QuizQuestion;
use crate::resources::links::ResourceLinks;
use crate::topics::registry::Quest;

trait Power {
    fn power(&self) -> u32;
}

struct Hero {
    level: u32,
}

impl Power for Hero {
    fn power(&self) -> u32 {
        self.level * 10
    }
}

fn describe<T: Power>(item: &T) -> u32 {
    item.power()
}

pub fn demo() -> String {
    let mut out = String::new();
    out.push_str("=== Traits & Generics ===\n\n");
    out.push_str(
        "A *trait* defines shared behavior (like an interface). \
         *Generics* let you write one function that works for many types, \
         as long as they implement the required trait. The compiler \
         generates specialized code for each type used — no runtime overhead.\n\n",
    );

    let h = Hero { level: 5 };
    out.push_str(&format!(
        "Step 1 — trait + impl\n  trait Power {{ fn power(&self) -> u32; }}\n  \
         impl Power for Hero {{ fn power(&self) -> self.level * 10 }}\n  \
         Hero level 5 → power = {}\n\n",
        h.power()
    ));
    out.push_str(&format!(
        "Step 2 — generic function with trait bound\n  fn describe<T: Power>(item: &T) -> u32\n  \
         describe(&h) = {}\n  \
         T can be any type that implements Power — compiler checks at build time.\n\n",
        describe(&h)
    ));
    out.push_str(
        "Step 3 — static dispatch (monomorphization)\n  \
         The compiler creates a separate `describe::<Hero>` at compile time.\n  \
         Calls are direct — no virtual table lookup like some OOP languages.\n\n",
    );
    out.push_str(
        "Step 4 — derive macros save boilerplate\n  \
         #[derive(Debug, Clone, PartialEq)] auto-writes common impl blocks.\n  \
         You will see derive on nearly every struct in this game.\n",
    );
    out
}

pub const MEMORY: &str =
    "Traits define shared behavior; monomorphization generates specialized code at compile time.";

static Q1: QuizQuestion = QuizQuestion::new(
    "Traits are most like…",
    &[
        "Java classes only",
        "Shared behavior interfaces",
        "Macros",
        "Modules",
    ],
    1,
    "impl Trait for Type provides methods.",
    "Traits define behavior types can implement.",
);

static Q2: QuizQuestion = QuizQuestion::new(
    "fn foo<T: Display>(x: T) uses…",
    &[
        "Dynamic dispatch only",
        "Generic type parameter with bound",
        "A macro",
        "unsafe",
    ],
    1,
    "T is replaced for each concrete type used.",
    "Generics with trait bounds constrain T.",
);

static Q3: QuizQuestion = QuizQuestion::new(
    "derive(Clone)…",
    &[
        "Makes type Copy always",
        "Auto-implements clone()",
        "Imports modules",
        "Removes borrow checker",
    ],
    1,
    "Proc macro generates impl at compile time.",
    "derive expands to impl Clone for your type.",
);

static BOSS: QuizQuestion = QuizQuestion::new(
    "Static dispatch means…",
    &[
        "Calls resolved at runtime via vtable always",
        "Compiler generates specialized functions per type",
        "No monomorphization",
        "Only works on heap",
    ],
    1,
    "Generics often compile to separate copies per T.",
    "Monomorphization creates type-specific code at compile time.",
);

static LINKS: ResourceLinks = ResourceLinks {
    book: "https://doc.rust-lang.org/book/ch10-00-generics.html",
    rust_by_example: "https://doc.rust-lang.org/rust-by-example/trait.html",
    std_docs: Some("https://doc.rust-lang.org/std/marker/trait.Copy.html"),
    reference: None,
    youtube: &["https://www.youtube.com/watch?v=T0XflTuIY6U"],
};

pub const QUEST: Quest = Quest {
    id: "traits_generics",
    order: 7,
    emoji: "⚡",
    title: "Traits & Generics",
    demo,
    memory_note: MEMORY,
    questions: &[Q1, Q2, Q3],
    boss: BOSS,
    links: LINKS,
};
