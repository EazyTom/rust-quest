//! Run with: cargo run --example advanced_cargo

use rust_quest::topics::registry;

fn main() {
    let quest = registry::find("advanced_cargo").expect("quest");
    println!("{} {}\n", quest.emoji, quest.title);
    println!("{}", (quest.demo)());
    println!("\nMemory: {}", quest.memory_note);
}
