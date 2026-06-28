//! Achievement badges and unlock rules.

pub struct Achievement {
    pub id: &'static str,
    pub emoji: &'static str,
    pub name: &'static str,
}

pub const ALL: &[Achievement] = &[
    Achievement {
        id: "first_steps",
        emoji: "👣",
        name: "First Steps",
    },
    Achievement {
        id: "borrow_slayer",
        emoji: "⚔️",
        name: "Borrow Slayer",
    },
    Achievement {
        id: "no_panic",
        emoji: "🧘",
        name: "No Panic",
    },
    Achievement {
        id: "iterator_hero",
        emoji: "🔄",
        name: "Iterator Hero",
    },
    Achievement {
        id: "thread_safe",
        emoji: "🧵",
        name: "Thread Safe",
    },
    Achievement {
        id: "full_stack_rustacean",
        emoji: "👑",
        name: "Full Stack Rustacean",
    },
    Achievement {
        id: "cellar_boss",
        emoji: "👹",
        name: "Warden Slayer",
    },
    Achievement {
        id: "archives_boss",
        emoji: "🗿",
        name: "Golem Breaker",
    },
    Achievement {
        id: "forge_boss",
        emoji: "👻",
        name: "Phantom Banished",
    },
    Achievement {
        id: "summit_boss",
        emoji: "🐉",
        name: "Dragon Slayer",
    },
    Achievement {
        id: "champion_victory",
        emoji: "🏆",
        name: "Legendary Champion",
    },
];

pub fn display_name(id: &str) -> Option<(&'static str, &'static str)> {
    ALL.iter().find(|a| a.id == id).map(|a| (a.emoji, a.name))
}
