//! Run with: cargo run --example ownership

use rust_quest::topics::registry;

fn main() {
    let quest = registry::find("ownership").expect("quest");
    println!("{} {}\n", quest.emoji, quest.title);
    println!("{}", (quest.demo)());
    println!("\nMemory: {}", quest.memory_note);
}
