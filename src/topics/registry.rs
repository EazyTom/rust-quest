//! Quest registry — single ordered list of all 14 quests.
//!
//! When adding a quest, register it here in learning order.

use crate::game::quiz::QuizQuestion;
use crate::resources::links::ResourceLinks;

use super::{
    advanced_cargo, cargo, collections, concurrency, errors, iterators_closures, lifetimes,
    modules_prelude, ownership, smart_pointers, structs_enums, testing_docs, traits_generics,
    types,
};

/// One playable quest in Rust Quest.
#[derive(Clone, Copy)]
pub struct Quest {
    pub id: &'static str,
    pub order: u8,
    pub emoji: &'static str,
    pub title: &'static str,
    pub demo: fn() -> String,
    pub memory_note: &'static str,
    pub questions: &'static [QuizQuestion],
    pub boss: QuizQuestion,
    pub links: ResourceLinks,
}

/// All quests in recommended learning order.
pub fn all() -> &'static [Quest] {
    &[
        cargo::QUEST,
        types::QUEST,
        ownership::QUEST,
        structs_enums::QUEST,
        errors::QUEST,
        collections::QUEST,
        traits_generics::QUEST,
        lifetimes::QUEST,
        modules_prelude::QUEST,
        iterators_closures::QUEST,
        smart_pointers::QUEST,
        concurrency::QUEST,
        testing_docs::QUEST,
        advanced_cargo::QUEST,
    ]
}

pub fn find(id: &str) -> Option<Quest> {
    all().iter().find(|q| q.id == id).copied()
}
