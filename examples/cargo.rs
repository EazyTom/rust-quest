//! Run with: cargo run --example cargo

use rust_quest::topics::registry;

fn main() {
    let quest = registry::find("cargo").expect("quest");
    println!("{} {}\n", quest.emoji, quest.title);
    println!("{}", (quest.demo)());
    println!("\nMemory: {}", quest.memory_note);
}
