# Contributing to Rust Quest

Thank you for helping improve **Rust Quest** — the retro CLI adventure for learning Rust.

This guide is for **human contributors**. AI coding agents should read [AGENTS.md](AGENTS.md) instead (or in addition).

---

## What you are contributing to

Rust Quest is a single crate (`rust-quest`) with:

- A **game engine** (`src/game/`) — state, saves, UI, audio, narrative
- **14 learning quests** (`src/topics/`) — demos, quizzes, resource links
- **Examples and tests** that keep the learning path honest

Design goals:

- Beginner-readable Rust in the game engine
- Heavily commented source (`// LEARN:` / `// GAME:`)
- Testable pure logic separated from terminal IO
- Everything (saves, test output) stays **inside the repo**

---

## Getting started

### Prerequisites

- **Rust 1.75+** ([rustup](https://rustup.rs/) stable, or distro packages — Debian 13 ships 1.85; Debian 12 ships 1.63 and is below MSRV)
- A UTF-8 terminal: **Windows Terminal**, **WezTerm**, or **iTerm2** recommended

### Clone and run

```bash
cd rust-quest
cargo run
cargo test
```

### Full quality gate (recommended before a PR)

**Windows:**

```powershell
.\scripts\run_tests.ps1
```

**macOS / Linux:**

```bash
./scripts/run_tests.sh
```

This runs: `fmt --check` → `check` → `clippy` → `test` → `release build`.

> **Windows tip:** Close the game before running tests. A running `rust-quest.exe` can lock the binary and cause "Access is denied".

---

## Project layout

| Path | Purpose |
|------|---------|
| `src/main.rs` | Entry: terminal setup, load save, music, hub |
| `src/game/state.rs` | Game rules (no printing) |
| `src/game/hub.rs` | Menus and quest flow |
| `src/game/progress.rs` | Save file + version migrations |
| `src/game/narrative.rs` | MUD-style room/enemy text per quest |
| `src/game/audio.rs` | Background music (MP3 in `assets/music/`) |
| `src/game/epic.rs` | Phase bosses and champion victory |
| `src/topics/registry.rs` | Quest order and registration |
| `tests/` | Integration tests |
| `examples/` | One runnable demo per quest |
| `.rust-test/` | Local saves (gitignored) |

See [README.md](README.md) for the player-facing quest list and ranks.

---

## How to contribute

### Bug fixes

1. Reproduce with `cargo run` or a failing test.
2. Fix with the **smallest correct change**.
3. Add or adjust a test if the bug was in pure logic (`state`, `quiz`, `progress`).
4. Run `.\scripts\run_tests.ps1` (or `./scripts/run_tests.sh`).

### New quest content

Follow the pattern in an existing file (e.g. `src/topics/ownership.rs`):

1. `demo()` — step-by-step teaching text
2. `MEMORY` — one-line memory-safety note
3. `Q1`–`Q3` + `BOSS` — `QuizQuestion` constants
4. `LINKS` — book, Rust by Example, optional YouTube
5. `pub const QUEST: Quest` — wire everything together

Then:

- Register in `src/topics/registry.rs` (mind unlock order)
- Add `[[example]]` in `Cargo.toml`
- Add encounter row in `src/game/narrative.rs`
- Update tests if quest count changes

### UI / copy / narrative

- **Dungeon Master flavor:** `src/game/narrative.rs` (per quest) and `src/game/ui/copy.rs` (shared)
- **Boxes and colors:** `src/game/ui/retro.rs` — respect `BOX_INNER_WIDTH`; pad text before coloring
- **Hub menus:** `dialoguer` Select with arrow keys; Esc = Back

### Background music

Drop a new `.mp3` into `assets/music/` using `snake_case` names (e.g. `dark_hall.mp3`). Tracks are discovered automatically.

Player settings (saved in progress):

- **Fixed track** — always play the chosen song until changed
- **Cycle per quest** — rotate when entering a quest from the map
- **Mute** — independent of track choice

### Save file changes

If you add persisted fields:

1. Update `GameState` and `SaveData` in `progress.rs`
2. Bump `SAVE_VERSION`
3. Accept older versions in `load_progress_from`
4. Migrate in `From<SaveData>` (see existing v4→v5 music migration)

Never store saves outside `.rust-test/` under the project root.

### Release version (GitHub Action)

App semver lives in `Cargo.toml` only — there are **no published binaries**; players clone and `cargo run`.

To cut a release:

1. Merge gameplay changes to the default branch.
2. Run **Actions → Bump version → Run workflow**.
3. Choose **patch** / **minor** / **major** (use **dry run** first to preview).
4. The workflow runs [`scripts/bump_release.py`](scripts/bump_release.py), which:
   - Increments `[package].version` in `Cargo.toml` and syncs `Cargo.lock`
   - Prepends a **What's New** entry in `README.md` from commits since the previous `v*.*.*` tag (gameplay paths only; skips cosmetic-only UI edits)
   - **Skips file edits** if `Cargo.toml`, `Cargo.lock`, and `README.md` already match the target release (including a `### vX.Y.Z` What's New entry) — useful when version files were updated in an earlier commit; the workflow still creates the git tag if missing
   - Commits, tags `vX.Y.Z`, and pushes

Tag the current `1.0.0` baseline once manually if this is the first release: `git tag -a v1.0.0 -m "rust-quest v1.0.0"`.

---

## Code style

- Run `cargo fmt --all` before submitting.
- `cargo clippy -- -D warnings` must pass.
- Match existing naming and module structure.
- Prefer explicit `match` over clever one-liners in the game engine.
- Avoid `unwrap()` in engine code; quest demos may use it when labeled for teaching.
- Module docs (`//!`) at the top of every `src/` file.
- Use `// LEARN:` and `// GAME:` prefixes where they help beginners.

**Keep game engine complexity at early-quest Rust level.** Advanced patterns belong in quest demos, not in `state.rs` / `progress.rs`.

---

## Testing guidelines

| Test type | Location | Good for |
|-----------|----------|----------|
| Unit | `#[cfg(test)]` in `src/game/*.rs` | Scoring, unlocks, save helpers |
| Integration | `tests/*.rs` | Registry, roundtrip saves, smoke demos |
| Doc | `///` examples | Public API teaching |

We **do not** automate dialoguer or crossterm in CI (no TTY). Extract logic to pure functions instead.

Integration tests write to `.rust-test/test-work/` — not system temp directories.

---

## Pull request checklist

- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo clippy --all-targets -- -D warnings` passes
- [ ] `cargo test` passes
- [ ] New quests include narrative, registry entry, example, and links
- [ ] Save schema bumped + migrated if you changed persistence
- [ ] No player saves or secrets committed
- [ ] README updated if player-visible behavior changed
- [ ] Focused PR — one feature or fix per PR when possible

---

## What we are not looking for (right now)

- Python comparison modules
- Ratatui or heavy TUI frameworks
- Async networking / multiplayer
- Player avatar systems
- Saves in `%APPDATA%` or other external paths
- Unrequested large refactors or new dependencies

---

## Questions and conduct

- Open an issue for large features before a big PR.
- Be kind — this project is built for learners, especially beginners reading the source.
- When in doubt, read [AGENTS.md](AGENTS.md) for architectural constraints agents follow — the same rules apply to humans.

---

## License

By contributing, you agree that your contributions are licensed under the same terms as the project ([MIT](LICENSE-MIT)).
