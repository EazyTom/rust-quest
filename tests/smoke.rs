//! LEARN: integration tests live in tests/ and call the library like an external user.

use rust_quest::topics::registry;

#[test]
fn every_quest_demo_non_empty() {
    for quest in registry::all() {
        let output = (quest.demo)();
        assert!(!output.is_empty(), "demo empty for {}", quest.id);
    }
}
