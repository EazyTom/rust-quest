//! Per-quest dungeon rooms, foes, and Dungeon Master narration.
//!
//! GAME: each quest is a "room" with a foe defeated by the challenge quiz.

use crate::game::ui::retro;
use crate::topics::registry::Quest;

/// Story framing for one quest encounter.
#[derive(Debug, Clone, Copy)]
pub struct QuestEncounter {
    pub room_name: &'static str,
    /// Two sentences — one per box line when entering the room.
    pub intro: [&'static str; 2],
    pub enemy_emoji: &'static str,
    pub enemy_name: &'static str,
    pub enemy_taunt: &'static str,
    pub enemy_defeat: &'static str,
    pub learn_prompt: &'static str,
    pub challenge_open: &'static str,
}

const ENCOUNTERS: &[(&str, QuestEncounter)] = &[
    (
        "cargo",
        QuestEncounter {
            room_name: "The Crate Cellar",
            intro: [
                "🕯️ Candlelight reveals a door of nailed planks.",
                "📦 A mimic clutches a glowing Cargo.toml.",
            ],
            enemy_emoji: "📦",
            enemy_name: "Manifest Mimic",
            enemy_taunt: "Build failed! Know your manifest, fool!",
            enemy_defeat: "The mimic drops its Cargo.toml and flees.",
            learn_prompt: "🕯️ You study crate runes by candlelight…",
            challenge_open: "The Manifest Mimic blocks the stair!",
        },
    ),
    (
        "types",
        QuestEncounter {
            room_name: "Hall of Sigils",
            intro: [
                "🕯️ Sigils pulse with i32, bool, and char.",
                "👻 A wraith shifts form until typed true.",
            ],
            enemy_emoji: "👻",
            enemy_name: "Type Wraith",
            enemy_taunt: "Wrong type! The wraith flickers and laughs.",
            enemy_defeat: "Named true, the wraith dissolves to mist.",
            learn_prompt: "🪄 You trace each sigil with careful ink…",
            challenge_open: "The Type Wraith bars the archway!",
        },
    ),
    (
        "ownership",
        QuestEncounter {
            room_name: "The Borrow Pit",
            intro: [
                "🕯️ Chains of rules hang from rusted hooks.",
                "👺 A goblin hoards every value — lend well.",
            ],
            enemy_emoji: "👺",
            enemy_name: "Move Goblin",
            enemy_taunt: "That value moved! You cannot borrow it!",
            enemy_defeat: "The goblin yields — ownership is yours.",
            learn_prompt: "📜 You read the laws of move and borrow…",
            challenge_open: "The Move Goblin snarls in the pit!",
        },
    ),
    (
        "structs_enums",
        QuestEncounter {
            room_name: "Mason's Atrium",
            intro: [
                "🕯️ Pillars of struct stone frame the hall.",
                "🐍 A viper forks paths with match-lit eyes.",
            ],
            enemy_emoji: "🐍",
            enemy_name: "Variant Viper",
            enemy_taunt: "No matching arm! The viper strikes!",
            enemy_defeat: "Your match arm pins the viper still.",
            learn_prompt: "🪄 You carve struct blueprints in the dust…",
            challenge_open: "The Variant Viper coils on the dais!",
        },
    ),
    (
        "errors",
        QuestEncounter {
            room_name: "The Panic Vault",
            intro: [
                "🕯️ Floor tiles read Ok on one side, Err on other.",
                "👹 An ogre roars — never unwrap in blind haste!",
            ],
            enemy_emoji: "👹",
            enemy_name: "Unwrap Ogre",
            enemy_taunt: "Panic! Your unwrap shatters the vault!",
            enemy_defeat: "Result in hand, the ogre slumps in chains.",
            learn_prompt: "📜 You learn to carry errors, not crash…",
            challenge_open: "The Unwrap Ogre guards the vault door!",
        },
    ),
    (
        "collections",
        QuestEncounter {
            room_name: "Stack & Heap Stacks",
            intro: [
                "🕯️ Shelves of Vec, HashMap, and String rise.",
                "🐀 A beast gnaws at indices past the end.",
            ],
            enemy_emoji: "🐀",
            enemy_name: "Bounds Beast",
            enemy_taunt: "Index out of bounds! The beast howls!",
            enemy_defeat: "Safe indexing — the beast retreats.",
            learn_prompt: "📜 You inventory growable hoards of data…",
            challenge_open: "The Bounds Beast prowls the stacks!",
        },
    ),
    (
        "traits_generics",
        QuestEncounter {
            room_name: "Polymorph Gallery",
            intro: [
                "🕯️ Statues demand one trait to pass the gate.",
                "🫠 Slime copies your shape — stay generic, hero.",
            ],
            enemy_emoji: "🫠",
            enemy_name: "Monomorph Slime",
            enemy_taunt: "Too concrete! The slime absorbs you!",
            enemy_defeat: "Generics hold — the slime melts away.",
            learn_prompt: "🪄 You sketch trait bounds on gallery walls…",
            challenge_open: "Monomorph Slime oozes from the plinth!",
        },
    ),
    (
        "lifetimes",
        QuestEncounter {
            room_name: "The Epoch Crypt",
            intro: [
                "🕯️ Hourglasses tick with `'a` carved in sand.",
                "💀 A lich binds every scroll to a lifetime.",
            ],
            enemy_emoji: "💀",
            enemy_name: "Lifetime Lich",
            enemy_taunt: "Does not live long enough! the lich wails.",
            enemy_defeat: "Annotations hold — the lich turns to dust.",
            learn_prompt: "📜 You read tomes that outlive their scribes…",
            challenge_open: "The Lifetime Lich rises from the crypt!",
        },
    ),
    (
        "modules_prelude",
        QuestEncounter {
            room_name: "Prelude Chapel",
            intro: [
                "🕯️ `use` candles light modular side halls.",
                "🌑 A shade hides in private crate-shadow.",
            ],
            enemy_emoji: "🌑",
            enemy_name: "Visibility Shade",
            enemy_taunt: "Private! You may not see that path!",
            enemy_defeat: "Modules aligned — the shade fades.",
            learn_prompt: "📜 You map `pub` doors and prelude paths…",
            challenge_open: "The Visibility Shade blocks the chapel!",
        },
    ),
    (
        "iterators_closures",
        QuestEncounter {
            room_name: "The Lazy Stream",
            intro: [
                "🕯️ A river of `.iter()` coils without end.",
                "🐉 A hydra regrows unless `.collect()` lands.",
            ],
            enemy_emoji: "🐉",
            enemy_name: "Lazy Hydra",
            enemy_taunt: "Still lazy! Another head sprouts!",
            enemy_defeat: "Collected and closed — the hydra sleeps.",
            learn_prompt: "🪄 You follow lazy streams and closure spells…",
            challenge_open: "The Lazy Hydra surges from the stream!",
        },
    ),
    (
        "smart_pointers",
        QuestEncounter {
            room_name: "Reference Refinery",
            intro: [
                "🕯️ Rc, Arc, and Box clang on the anvil.",
                "👁️ A specter points at memory long freed.",
            ],
            enemy_emoji: "👁️",
            enemy_name: "Dangling Specter",
            enemy_taunt: "Use after free! The specter wails!",
            enemy_defeat: "Smart pointers shine — specter banished.",
            learn_prompt: "📜 You forge pointers that own their fate…",
            challenge_open: "The Dangling Specter haunts the forge!",
        },
    ),
    (
        "concurrency",
        QuestEncounter {
            room_name: "The Mutex Maze",
            intro: [
                "🕯️ Threaded corridors echo with lock and key.",
                "🐲 A drake races threads — Sync or perish!",
            ],
            enemy_emoji: "🐲",
            enemy_name: "Data-Race Drake",
            enemy_taunt: "Race detected! The drake breathes chaos!",
            enemy_defeat: "Synchronized — the drake falls silent.",
            learn_prompt: "🪄 You learn to share state without panic…",
            challenge_open: "The Data-Race Drake circles above!",
        },
    ),
    (
        "testing_docs",
        QuestEncounter {
            room_name: "Assert Amphitheater",
            intro: [
                "🕯️ Banners read `#[test]` and `///` doc runes.",
                "🧌 A gremlin skips every test that always pass.",
            ],
            enemy_emoji: "🧌",
            enemy_name: "Flaky Gremlin",
            enemy_taunt: "Assertion failed! The gremlin cackles!",
            enemy_defeat: "Green tests — the gremlin flees the stage.",
            learn_prompt: "📜 You inscribe tests and doc comments…",
            challenge_open: "The Flaky Gremlin jeers from the pit!",
        },
    ),
    (
        "advanced_cargo",
        QuestEncounter {
            room_name: "The Release Tower",
            intro: [
                "🕯️ Wind howls past workspace and feature flags.",
                "🗿 A gargoyle guards `--release` tower secrets.",
            ],
            enemy_emoji: "🗿",
            enemy_name: "Feature Gargoyle",
            enemy_taunt: "Wrong profile! The gargoyle stonewalls you!",
            enemy_defeat: "Release built — the gargoyle crumbles.",
            learn_prompt: "🪄 You climb flags toward optimized builds…",
            challenge_open: "The Feature Gargoyle blocks the summit!",
        },
    ),
];

pub fn encounter(quest_id: &str) -> Option<&'static QuestEncounter> {
    ENCOUNTERS
        .iter()
        .find(|(id, _)| *id == quest_id)
        .map(|(_, e)| e)
}

pub fn encounter_for(quest: Quest) -> Option<&'static QuestEncounter> {
    encounter(quest.id)
}

/// Print the two-sentence room intro when entering a quest from the map.
pub fn print_room_arrival(quest: Quest) {
    let Some(enc) = encounter_for(quest) else {
        return;
    };
    println!();
    println!("{}", retro::box_top(&format!("🕯️  {}", enc.room_name)));
    println!("{}", retro::dungeon_master_box_line(enc.intro[0]));
    println!("{}", retro::dungeon_master_box_line(enc.intro[1]));
    println!(
        "{}",
        retro::enemy_box_line(enc.enemy_emoji, enc.enemy_name, "blocks your path")
    );
    println!("{}\n", retro::box_bottom());
}

/// One-line map footer for the selected quest node.
pub fn map_selection_blurb(quest_id: &str) -> Option<String> {
    let enc = encounter(quest_id)?;
    Some(format!(
        "{} {} · {} — {}",
        enc.enemy_emoji, enc.enemy_name, enc.room_name, enc.challenge_open
    ))
}
