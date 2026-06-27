//! Run with: cargo run --example iterators_closures

use rust_quest::topics::registry;

fn main() {
    let quest = registry::find("iterators_closures").expect("quest");
    println!("{} {}\n", quest.emoji, quest.title);
    println!("{}", (quest.demo)());
    println!("\nMemory: {}", quest.memory_note);
}
