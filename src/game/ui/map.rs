//! Crossterm quest path map — arrow keys select quests.
//!
//! Read this to learn how the interactive map works (no ratatui).

use std::io::{self, Write};

use colored::Colorize;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute, terminal as crossterm_terminal,
};

use crate::game::epic;
use crate::game::narrative;
use crate::game::state::GameState;
use crate::topics::registry;

use super::retro;

/// Visual state of one node on the map.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeStatus {
    Completed,
    Available,
    Locked,
}

pub struct MapNode {
    pub quest_id: &'static str,
    pub emoji: &'static str,
    pub label: &'static str,
}

pub fn map_nodes() -> Vec<MapNode> {
    registry::all()
        .iter()
        .map(|q| MapNode {
            quest_id: q.id,
            emoji: q.emoji,
            label: q.title,
        })
        .collect()
}

/// GAME: pure status for tests — no terminal IO.
pub fn node_status(state: &GameState, quest_id: &str) -> NodeStatus {
    if state.quest_completed(quest_id) {
        NodeStatus::Completed
    } else if state.is_unlocked(quest_id) {
        NodeStatus::Available
    } else {
        NodeStatus::Locked
    }
}

/// GAME: first cursor position when opening the map — next quest to play.
pub fn initial_map_selection(state: &GameState) -> usize {
    let nodes = map_nodes();
    // GAME: cursor lands on the next quest to play, not the last one you finished.
    for (i, node) in nodes.iter().enumerate() {
        if node_status(state, node.quest_id) == NodeStatus::Available {
            return i;
        }
    }
    for (i, node) in nodes.iter().enumerate() {
        if node_status(state, node.quest_id) != NodeStatus::Locked {
            return i;
        }
    }
    0
}

fn prepare_terminal_for_map() -> io::Result<()> {
    // LEARN: dialoguer uses cooked mode; reset before crossterm raw mode so keys work.
    // GAME: also clears hub text when entering (or re-entering) the quest map.
    super::input::clear_screen()
}

fn next_selectable(state: &GameState, from: usize, delta: i32) -> usize {
    let nodes = map_nodes();
    if nodes.is_empty() {
        return 0;
    }
    let mut idx = from as i32;
    for _ in 0..nodes.len() {
        idx = (idx + delta).rem_euclid(nodes.len() as i32);
        let u = idx as usize;
        if node_status(state, nodes[u].quest_id) != NodeStatus::Locked {
            return u;
        }
    }
    from
}

fn render_map(state: &GameState, selected: usize) -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(
        stdout,
        crossterm_terminal::Clear(crossterm_terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    writeln!(stdout, "{}", retro::box_top("🧭  QUEST MAP  🗺️"))?;
    writeln!(stdout, "{}", retro::box_line(""))?;

    let nodes = map_nodes();
    for (i, node) in nodes.iter().enumerate() {
        let status = node_status(state, node.quest_id);
        let dungeon_door = if epic::dungeon_ready_at(state, node.quest_id) {
            "🚪"
        } else {
            ""
        };
        let prefix = match status {
            NodeStatus::Completed => "✅",
            NodeStatus::Available if i == selected => "▶ ",
            NodeStatus::Available => "○ ",
            NodeStatus::Locked => "🔒",
        };
        let line = if dungeon_door.is_empty() {
            format!("{} {}  {}", prefix, node.emoji, node.label)
        } else {
            format!("{} {} {}  {}", prefix, dungeon_door, node.emoji, node.label)
        };
        let row = match status {
            NodeStatus::Completed => retro::box_line_styled(&line, |s| s.green()),
            NodeStatus::Available if i == selected => {
                retro::box_line_styled(&line, |s| s.bright_magenta().bold())
            }
            NodeStatus::Available => retro::box_line(&line),
            NodeStatus::Locked => retro::box_line_styled(&line, |s| s.dimmed()),
        };
        writeln!(stdout, "{}", row)?;
        if i + 1 < nodes.len() {
            writeln!(stdout, "{}", retro::box_line("     │"))?;
        }
    }

    writeln!(stdout, "{}", retro::box_bottom())?;
    if let Some(node) = nodes.get(selected) {
        if node_status(state, node.quest_id) != NodeStatus::Locked {
            if let Some(blurb) = narrative::map_selection_blurb(node.quest_id) {
                writeln!(stdout)?;
                writeln!(stdout, "{}", retro::dungeon_master_says(&blurb))?;
            }
        }
    }
    writeln!(
        stdout,
        "{}",
        "  ↑/↓ move   Enter enter room   Esc camp".bright_white()
    )?;
    stdout.flush()?;
    Ok(())
}

/// Returns selected quest id, or None when user exits to hub.
pub fn run_quest_map(state: &GameState) -> io::Result<Option<&'static str>> {
    prepare_terminal_for_map()?;
    crossterm_terminal::enable_raw_mode()?;
    let result = quest_map_loop(state);
    crossterm_terminal::disable_raw_mode()?;
    // GAME: drain Enter/Esc so the hub dialoguer menu does not auto-select Quest Map.
    super::input::drain_pending_keys_quiet();
    result
}

/// GAME: arrow-key loop — Esc returns None (back to hub), Enter enters the highlighted quest.
fn quest_map_loop(state: &GameState) -> io::Result<Option<&'static str>> {
    let nodes = map_nodes();
    let mut selected = initial_map_selection(state);

    render_map(state, selected)?;

    loop {
        // GAME: poll without redrawing — full-screen clear+repaint every 100ms caused flashing
        // on some Linux terminals (ghost borders, unreadable map). Redraw only on input/resize.
        if !event::poll(std::time::Duration::from_millis(100))? {
            continue;
        }
        match event::read()? {
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Up => {
                    let next = next_selectable(state, selected, -1);
                    if next != selected {
                        selected = next;
                        render_map(state, selected)?;
                    }
                }
                KeyCode::Down => {
                    let next = next_selectable(state, selected, 1);
                    if next != selected {
                        selected = next;
                        render_map(state, selected)?;
                    }
                }
                KeyCode::Enter => {
                    let id = nodes[selected].quest_id;
                    if node_status(state, id) == NodeStatus::Locked {
                        continue;
                    }
                    return Ok(Some(id));
                }
                KeyCode::Esc | KeyCode::Char('q') => return Ok(None),
                _ => {}
            },
            Event::Resize(_, _) => render_map(state, selected)?,
            _ => {}
        }
    }
}
