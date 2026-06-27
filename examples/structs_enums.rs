//! Run with: cargo run --example structs_enums

use rust_quest::topics::registry;

fn main() {
    let quest = registry::find("structs_enums").expect("quest");
    println!("{} {}\n", quest.emoji, quest.title);
    println!("{}", (quest.demo)());
    println!("\nMemory: {}", quest.memory_note);
}
