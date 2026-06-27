//! Run with: cargo run --example modules_prelude

use rust_quest::topics::registry;

fn main() {
    let quest = registry::find("modules_prelude").expect("quest");
    println!("{} {}\n", quest.emoji, quest.title);
    println!("{}", (quest.demo)());
    println!("\nMemory: {}", quest.memory_note);
}
