//! Flush stray key events before cooked-mode menus (dialoguer).
//!
//! LEARN: raw-mode UIs (quest map) can leave Enter/Esc in the queue; dialoguer
//! then treats them as an immediate menu pick — often opening Quest Map on hub startup.
//!
//! GAME: call after raw mode ends and after text prompts, before the next Select.

use std::io::{self, Write};
use std::time::Duration;

use crossterm::event::{self};
use crossterm::{cursor, execute, terminal as crossterm_terminal};

/// Clear the terminal and home the cursor — switch from raw map mode back to cooked hub output.
/// LEARN: dialoguer and println! expect a clean screen; leftover map pixels stack above new hub text.
pub fn clear_screen() -> io::Result<()> {
    let _ = crossterm_terminal::disable_raw_mode();
    let mut stdout = io::stdout();
    execute!(
        stdout,
        crossterm_terminal::Clear(crossterm_terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;
    stdout.flush()?;
    Ok(())
}

pub fn clear_screen_quiet() {
    let _ = clear_screen();
}

/// Discard pending keyboard events (non-blocking).
pub fn drain_pending_keys() -> io::Result<()> {
    while event::poll(Duration::from_millis(0))? {
        let _ = event::read()?;
    }
    Ok(())
}

/// Best-effort drain — never fails the caller.
pub fn drain_pending_keys_quiet() {
    let _ = drain_pending_keys();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drain_is_noop_when_empty() {
        drain_pending_keys().unwrap();
    }
}
