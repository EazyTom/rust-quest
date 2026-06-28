# AGENTS.md — Rust Quest

Guide for Cursor, Claude, and other coding agents working in this repository.

## What this project is

**Rust Quest** (`rust-quest/`) is a self-contained, retro MUD-style CLI game that teaches Rust through **14 quests**, quizzes, ranks, dungeon bosses, and heavily commented source code. Crate name: `rust-quest`. Import path: `rust_quest::`. Display title: **Rust Quest**.

**Project root:** `rust-quest/` — run all `cargo` and git commands here. Parent directories (`Rust-Test/`) are outside the game repo boundary unless the user explicitly widens scope.

**Dual learning path:**

1. Play the game (`cargo run`).
2. Read the source (`// LEARN:` / `// GAME:` comments throughout).

---

## Quick commands

```bash
cd rust-quest
cargo run                    # play
cargo test                   # unit + integration + doc tests
.\scripts\run_tests.ps1      # Windows full harness (fmt, clippy, test, release)
./scripts/run_tests.sh       # Unix equivalent
cargo run --example ownership
```

Close a running `rust-quest.exe` before `cargo test` on Windows — the binary may be locked ("Access is denied").

---

## Directory map (current)

```text
rust-quest/
├── AGENTS.md              ← you are here
├── CONTRIBUTING.md        ← human contributor guide
├── README.md
├── Cargo.toml
├── assets/music/*.mp3     ← background tracks (auto-discovered)
├── .rust-test/            ← gitignored saves + test-work/
├── src/
│   ├── main.rs            ← terminal setup → load → music → hub → save on quit
│   ├── lib.rs
│   ├── game/
│   │   ├── hub.rs         ← menus, quest flow, quizzes, music menu
│   │   ├── state.rs       ← pure rules (NO println!)
│   │   ├── progress.rs    ← JSON save v5, migrations
│   │   ├── xp.rs, quiz.rs, achievements.rs
│   │   ├── epic.rs        ← 4 phase dungeon bosses + champion screen
│   │   ├── narrative.rs   ← per-quest room/enemy MUD copy (14 entries)
│   │   ├── audio.rs       ← rodio MP3 loop, MusicMode, mute
│   │   └── ui/            ← terminal, retro, copy, map (crossterm)
│   ├── topics/
│   │   ├── registry.rs    ← canonical quest order (single source of truth)
│   │   └── *.rs           ← one file per quest (demo, quiz, links)
│   └── resources/links.rs
├── examples/              ← 14 quest demos
├── tests/                 ← integration tests + fixtures/
└── scripts/run_tests.*
```

---

## Architecture rules

| Layer | Responsibility | IO? |
|-------|----------------|-----|
| `game/state.rs` | Unlocks, XP-once, ranks, streaks, achievements triggers | **No** |
| `game/quiz.rs` | Scoring, shuffled choices, pass ≥75% | **No** |
| `game/progress.rs` | Save/load, schema version, migrations | Files only |
| `game/hub.rs` | dialoguer menus, quest flow, prints | Yes |
| `game/ui/map.rs` | crossterm quest map | Yes (not CI-tested) |
| `topics/*.rs` | Static quest content | No |
| `game/narrative.rs` | Static encounter table | No |

**Hub loop:** `main` → `load_progress` → `MusicHandle::launch_music` → `run_hub` → `save_progress` on quit.

**Quest flow:** Hub → Quest Map (`run_quest_map`) → `cycle_on_quest` (if music cycle mode) → `run_quest` → Learn / Challenge / Resources.

**Quiz flow:** 3 questions + boss (Q4). `PresentedQuestion` shuffles choices so index 0 is not always correct. Pass threshold: **≥75%** via `score_presented`.

---

## Save file (`.rust-test/progress.json`)

- Path: `env!("CARGO_MANIFEST_DIR").join(".rust-test/progress.json")` — **never** `%APPDATA%`, `dirs`, or `temp_dir`.
- Current `SAVE_VERSION`: **5** (see `game/progress.rs`).
- Accept loads from versions 1–5 with `#[serde(default)]` on new fields.
- **Always bump `SAVE_VERSION`** and add migration logic when adding persisted fields.

**Music settings (v5):**

| Field | Meaning |
|-------|---------|
| `music_muted` | Player chose mute — stays silent until unmute |
| `music_mode` | `fixed` = always play `music_track`; `cycle_on_quest` = rotate on quest map entry |
| `music_track` | Stem for fixed mode (e.g. `mossy_gate`) |
| `music_last_stem` | Cycle cursor for quest rotation |

Runtime-only: `music_playing_stem` on `GameState` (not saved).

---

## 14 quests (registry order)

Register in `src/topics/registry.rs` — do not reorder lightly (unlocks, ranks, epic phases depend on order).

| # | ID | Title |
|---|-----|-------|
| 1 | `cargo` | Cargo Workflow |
| 2 | `types` | Types & Variables |
| 3 | `ownership` | Ownership & Borrowing |
| 4 | `structs_enums` | Structs & Enums |
| 5 | `errors` | Errors & Result |
| 6 | `collections` | Collections |
| 7 | `traits_generics` | Traits & Generics |
| 8 | `lifetimes` | Lifetimes |
| 9 | `modules_prelude` | Modules & Prelude |
| 10 | `iterators_closures` | Iterators & Closures |
| 11 | `smart_pointers` | Smart Pointers |
| 12 | `concurrency` | Concurrency |
| 13 | `testing_docs` | Testing & Docs |
| 14 | `advanced_cargo` | Advanced Cargo |

**Epic phases** (`game/epic.rs`): Cellar (1–5), Archives (6–8), Forge (9–11), Summit (12–14). Phase-end quests show 🚪 on the map when a dungeon boss is ready.

**Per-quest narrative** (`game/narrative.rs`): room name, 2-line intro, enemy name/emoji/taunt/defeat — separate from phase bosses.

---

## How to add features

### New quest (15+)

1. Create `src/topics/my_quest.rs` with `QUEST` const matching `Quest` struct in `registry.rs`.
2. Add module to `topics/mod.rs` and entry in `registry::all()` at correct order.
3. Add `[[example]]` in `Cargo.toml`.
4. Add row to `ENCOUNTERS` in `game/narrative.rs`.
5. Update `tests/quest_registry.rs` expected count if intentionally adding beyond 14.
6. Update `epic.rs` phase grouping if the quest belongs to a story arc.
7. Add links in `resources/links.rs` pattern inside the topic file.

### New background track

1. Drop `my_track.mp3` in `assets/music/` (use `snake_case` filenames).
2. No code changes required — `audio::discover_tracks()` picks it up.
3. Label auto-generated: `my_track` → "My Track".

### New persisted setting

1. Add field to `GameState` and `SaveData`.
2. Bump `SAVE_VERSION`, extend `load_progress_from` version check.
3. Implement migration in `From<SaveData>` or a dedicated `migrate_*` fn.
4. Wire hub UI + save on change.
5. Preserve on `GameState::reset()` if it is a player preference (like music/name).

### New UI copy / MUD flavor

- Reusable strings: `game/ui/copy.rs`
- Per-quest rooms/enemies: `game/narrative.rs`
- Box drawing / colors: `game/ui/retro.rs` — use `BOX_INNER_WIDTH` (42 cols), **pad plain text before ANSI styling**

### New achievement

- Rule in `game/achievements.rs`, trigger from `state.complete_step` or hub/quiz paths.

---

## UI conventions

- **Hub menus:** `dialoguer::Select` with `.interact_opt()` — Esc returns `None` (Back). Default index 0. Arrow keys, not letter-key prefixes like `[M]`.
- **Quest map:** raw crossterm in `map.rs`; switch from dialoguer cooked mode before enabling raw mode.
- **Colors:** `colored` crate; success/failure via `retro::success` / `retro::failure` (emoji + text for accessibility).
- **Comments in demos:** step-numbered, beginner-friendly; game engine stays "Quest 1–6 level Rust."

---

## Testing requirements

Before finishing a change:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Prefer extracting **pure functions** over mocking. Test:

- `game/state.rs`, `game/quiz.rs`, `game/progress.rs` — unit tests co-located
- `tests/*.rs` — integration (registry, save roundtrip, map node status, smoke)

**Do not** add CI tests that require a TTY for dialoguer or crossterm.

Writable test output: `.rust-test/test-work/` only. Fixtures: `tests/fixtures/` (committed, read-only).

---

## Things agents should AVOID (lessons learned)

1. **Do not edit the original plan file** (`rust_learning_project_ac2795e0.plan*.md`) — it is reference only.
2. **Do not add player avatars or custom menu input layers** — rejected; use dialoguer Select.
3. **Do not use `[M]`-style menu labels** that imply letter keys when dialoguer uses arrows.
4. **Do not default quiz cursor to the correct answer** — always use `QuizQuestion::present()` shuffle.
5. **Do not pad ANSI-colored strings for box width** — pad plain text first (`retro::pad_inner`), then style.
6. **Do not store saves outside the repo** — no `dirs`, `%APPDATA%`, or system temp for progress.
7. **Do not put `println!` in `game/state.rs`** — keep rules testable and IO-free.
8. **Do not use ratatui** — quest map is raw crossterm only.
9. **Do not scatter narrative copy across 14 topic files** — centralize encounters in `narrative.rs`; keep topic files for teaching content.
10. **Do not hardcode MP3 track enums** — use `assets/music/` discovery so new files work without code changes.
11. **Do not conflate music mute with track selection** — `music_muted` is independent; mute must persist across sessions.
12. **Do not cycle music on every app launch** unless the player chose cycle mode — fixed track means fixed until changed.
13. **Do not over-engineer** — minimal diff, match existing patterns, no speculative abstractions.
14. **Do not commit `.rust-test/progress.json`** — gitignored player data.
15. **Do not add Python comparison modules** — out of scope per plan.
16. **Do not skip save version bumps** when adding fields — silent data loss otherwise.
17. **Avoid `unwrap()` in game engine** — ok in labeled quest demos only.
18. **Do not create markdown files the user did not ask for** — except this AGENTS.md / CONTRIBUTING.md request.

---

## Dependencies (when to add more)

| Crate | Use |
|-------|-----|
| `dialoguer` | Hub/quiz menus |
| `crossterm` | Quest map only |
| `colored` | ANSI colors |
| `unicode-width` | Emoji-safe box padding |
| `serde` / `serde_json` | Saves |
| `opener` | Browser links |
| `rodio` | Background MP3 (features: `mp3`) |

Avoid new deps unless necessary. No `dirs`, no `ratatui`, no async runtime.

---

## Comment prefixes

```rust
//! Module doc — what file does, when to read it
// LEARN: — teaches Rust
// GAME: — explains game logic
```

Every `src/` file should have a `//!` module doc. Match the readability level of surrounding code.

---

## Public API surface

`lib.rs` re-exports: `GameState`, `load_progress`, `save_progress`, `run_hub`, `MusicHandle`, `MusicMode`.

`run_hub(state, music)` requires a `MusicHandle` — create in `main` with `MusicHandle::start()`.

---

## Out of scope

Async deep dives, WASM, unsafe, proc macros, multiplayer, in-terminal code editing, Python contrasts.

---

## Verification checklist (before marking work done)

- [ ] `cargo test` passes (game not running on Windows)
- [ ] `clippy` clean with `-D warnings`
- [ ] `cargo fmt --check` clean
- [ ] Save migration handled if schema changed
- [ ] New quests registered in order with narrative + tests updated
- [ ] No secrets in commits; `.rust-test/` not staged
- [ ] Focused diff — no unrelated refactors

---

## Reference

Human-oriented workflow: [CONTRIBUTING.md](CONTRIBUTING.md). Player-facing docs: [README.md](README.md). Original build plan: kept outside repo as design reference — do not modify it in place.
