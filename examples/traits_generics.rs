//! Run with: cargo run --example traits_generics

use rust_quest::topics::registry;

fn main() {
    let quest = registry::find("traits_generics").expect("quest");
    println!("{} {}\n", quest.emoji, quest.title);
    println!("{}", (quest.demo)());
    println!("\nMemory: {}", quest.memory_note);
}
