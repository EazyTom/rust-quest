//! Run with: cargo run --example smart_pointers

use rust_quest::topics::registry;

fn main() {
    let quest = registry::find("smart_pointers").expect("quest");
    println!("{} {}\n", quest.emoji, quest.title);
    println!("{}", (quest.demo)());
    println!("\nMemory: {}", quest.memory_note);
}
