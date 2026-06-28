//! Integration tests for the 14-quest registry ordering and link metadata.
//!
//! LEARN: `registry::all()` returns a static slice — no heap allocation at runtime.

use rust_quest::game::narrative;
use rust_quest::topics::registry;

#[test]
fn fourteen_quests_ordered() {
    let quests = registry::all();
    assert_eq!(quests.len(), 14);
    let mut ids = std::collections::HashSet::new();
    for (i, q) in quests.iter().enumerate() {
        assert_eq!(q.order, (i + 1) as u8);
        assert!(ids.insert(q.id));
        assert!(!q.links.book.is_empty());
        assert!(!q.links.rust_by_example.is_empty());
        assert!(
            narrative::encounter(q.id).is_some(),
            "missing narrative for {}",
            q.id
        );
    }
}
