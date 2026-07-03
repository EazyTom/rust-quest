//! Retro box-drawing banners and borders.
//!
//! Shared by the hub, quest map, and title screen.
//!
//! LEARN: emoji and CJK are "wide" — count display columns with `unicode_width`, not `.len()`.

use colored::Colorize;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use super::terminal;

/// Fixed inner width (display columns) for every box row — keeps right edges aligned.
pub const BOX_INNER_WIDTH: usize = 54;

fn pad_to(text: &str, width: usize) -> String {
    let w = text.width();
    if w >= width {
        text.to_string()
    } else {
        format!("{}{}", text, " ".repeat(width - w))
    }
}

/// Pad plain text to exactly [`BOX_INNER_WIDTH`] display columns.
pub fn pad_inner(text: &str) -> String {
    pad_to(text, BOX_INNER_WIDTH)
}

/// Center plain text within [`BOX_INNER_WIDTH`] display columns.
pub fn pad_center(text: &str) -> String {
    let w = text.width();
    if w >= BOX_INNER_WIDTH {
        text.to_string()
    } else {
        let pad_total = BOX_INNER_WIDTH - w;
        let left = pad_total / 2;
        format!("{}{}{}", " ".repeat(left), text, " ".repeat(pad_total - left))
    }
}

/// Center `plain`, then swap `needle` for its styled ANSI version (same visible width).
/// LEARN: pad with plain text first — ANSI escape codes must not affect column math.
fn center_and_style(plain: &str, needle: &str, styled: &str) -> String {
    pad_center(plain).replacen(needle, styled, 1)
}

/// Word-wrap plain text to fit the box; each row is padded to [`BOX_INNER_WIDTH`].
pub fn wrap_inner(text: &str) -> Vec<String> {
    let max = BOX_INNER_WIDTH;
    if text.is_empty() {
        return vec![pad_to("", max)];
    }

    let mut rows: Vec<String> = Vec::new();
    let mut line = String::new();
    let mut line_w = 0usize;

    let flush_line = |line: &mut String, line_w: &mut usize, rows: &mut Vec<String>| {
        if !line.is_empty() || rows.is_empty() {
            rows.push(pad_to(line, max));
            line.clear();
            *line_w = 0;
        }
    };

    for word in text.split_whitespace() {
        let ww = word.width();
        if ww > max {
            flush_line(&mut line, &mut line_w, &mut rows);
            let mut chunk = String::new();
            let mut chunk_w = 0usize;
            for ch in word.chars() {
                let cw = UnicodeWidthChar::width(ch).unwrap_or(0);
                if chunk_w + cw > max && !chunk.is_empty() {
                    rows.push(pad_to(&chunk, max));
                    chunk.clear();
                    chunk_w = 0;
                }
                chunk.push(ch);
                chunk_w += cw;
            }
            if !chunk.is_empty() {
                line = chunk;
                line_w = chunk_w;
            }
            continue;
        }

        let needed = if line.is_empty() { ww } else { line_w + 1 + ww };
        if needed > max {
            flush_line(&mut line, &mut line_w, &mut rows);
            line.push_str(word);
            line_w = ww;
        } else {
            if !line.is_empty() {
                line.push(' ');
                line_w += 1;
            }
            line.push_str(word);
            line_w += ww;
        }
    }
    flush_line(&mut line, &mut line_w, &mut rows);
    rows
}

pub fn title_banner() -> String {
    let subtitle = if terminal::use_emoji() {
        "~ learn rust. earn ranks. ~"
    } else {
        "learn rust. earn ranks."
    };
    format!(
        "{}\n{}\n{}",
        format!(
            "{}\n{}",
            format!("╔{}╗", "═".repeat(horizontal_rule_len())).bright_cyan(),
            box_border(&title_line_styled())
        ),
        box_border(&pad_center(subtitle)),
        box_bottom()
    )
}

/// Centered title row — bold red on cyan for "RUST QUEST".
fn title_line_styled() -> String {
    let title = "RUST QUEST";
    let styled = title.red().bold().on_cyan().to_string();
    let plain = if terminal::use_emoji() {
        format!("🧙 {title} ⚔️")
    } else {
        title.to_string()
    };
    center_and_style(&plain, title, &styled)
}

fn horizontal_rule_len() -> usize {
    // LEARN: top/bottom use `═`.repeat(this); side rows add 4 columns of border + padding.
    BOX_INNER_WIDTH + 2
}

pub fn box_top(title: &str) -> String {
    format!(
        "{}\n{}",
        format!("╔{}╗", "═".repeat(horizontal_rule_len())).bright_cyan(),
        box_line(title)
    )
}

pub fn box_top_centered(title: &str) -> String {
    format!(
        "{}\n{}",
        format!("╔{}╗", "═".repeat(horizontal_rule_len())).bright_cyan(),
        box_border(&pad_center(title))
    )
}

/// Magenta diamond rule matching the width of box rows.
fn menu_ornament() -> String {
    let w = horizontal_rule_len() + 2;
    format!("◆{}◆", "─".repeat(w.saturating_sub(2)))
}

/// Bordered main menu header — decorative frame; dialoguer renders the live selection below.
pub fn main_menu_frame() -> String {
    let title = if terminal::use_emoji() {
        "⚔️  Main Menu  🕯️"
    } else {
        "Main Menu"
    };
    let hint = if terminal::use_emoji() {
        "↑/↓ choose  ·  Enter  ·  Esc quit"
    } else {
        "Up/Down  ·  Enter  ·  Esc quit"
    };
    let rule = menu_ornament().bright_magenta();
    format!(
        "\n{}\n{}\n{}\n{}\n{}",
        rule,
        box_top(title),
        box_line_styled(hint, |s| s.bright_white().dimmed()),
        box_bottom(),
        rule
    )
}

/// Plain-text row inside the box (wraps long lines; pad before borders).
pub fn box_line(text: &str) -> String {
    wrap_inner(text)
        .into_iter()
        .map(|inner| box_border(&inner))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Styled row: wrap and pad plain text for width, then apply color/style to each row.
pub fn box_line_styled(plain: &str, style: fn(&str) -> colored::ColoredString) -> String {
    wrap_inner(plain)
        .into_iter()
        .map(|padded| box_border(&style(&padded).to_string()))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Draw vertical borders around already-padded inner content (may include ANSI color codes).
pub fn box_border(inner: &str) -> String {
    format!("{}{}{}", "║ ".bright_cyan(), inner, " ║".bright_cyan())
}

pub fn box_bottom() -> String {
    format!(
        "{}",
        format!("╚{}╝", "═".repeat(horizontal_rule_len())).bright_cyan()
    )
}

pub fn section_header(title: &str) -> String {
    format!("\n{}", title.bright_magenta().bold())
}

pub fn success(msg: &str) -> String {
    if terminal::use_emoji() {
        format!("✅ {}", msg.green())
    } else {
        format!("[OK] {}", msg.green())
    }
}

pub fn failure(msg: &str) -> String {
    if terminal::use_emoji() {
        format!("❌ {}", msg.red())
    } else {
        format!("[!!] {}", msg.red())
    }
}

/// Dungeon Master narration row inside a quest room box.
pub fn dungeon_master_box_line(text: &str) -> String {
    box_line_styled(text, |s| s.bright_white().italic())
}

/// Foe presence row inside a quest room box.
pub fn enemy_box_line(emoji: &str, name: &str, action: &str) -> String {
    let plain = format!("{emoji} {name} {action}");
    box_line_styled(&plain, |s| s.red().bold())
}

pub fn dungeon_master_says(text: &str) -> String {
    format!(
        "{} {}",
        "🎲 Dungeon Master:".bright_white().bold(),
        text.bright_white().italic()
    )
}

pub fn teaching_note(text: &str) -> String {
    format!("💡 {}", text.bright_yellow())
}

pub fn lore_scroll(text: &str) -> String {
    format!("📜 {}", text.bright_cyan())
}

pub fn enemy_says(emoji: &str, name: &str, text: &str) -> String {
    format!(
        "{} {} {}: \"{}\"",
        emoji,
        name.red().bold(),
        "snarls".dimmed(),
        text.bright_red()
    )
}

pub fn combat_round(round: usize, total: usize) -> String {
    format!(
        "⚔️🛡️  Strike {}/{} — swords and shields clash!",
        round, total
    )
    .bright_yellow()
    .bold()
    .to_string()
}

pub fn boss_combat_round(round: usize, total: usize, boss_emoji: &str) -> String {
    format!(
        "⚔️🛡️  {boss_emoji} Boss strike {}/{} — blades ring!",
        round, total
    )
    .bright_red()
    .bold()
    .to_string()
}

pub fn combat_miss() -> String {
    "💣 The blow goes wide!".bright_red().to_string()
}

pub fn combat_hit() -> String {
    "⚔️🛡️ Clean parry — true lore lands!".bright_green().bold().to_string()
}

pub fn final_gambit(emoji: &str, name: &str) -> String {
    format!(
        "💣 Final gambit — {} {} hurls a logic bomb!",
        emoji, name
    )
    .bright_red()
    .bold()
    .to_string()
}

pub fn victory_flash(text: &str) -> String {
    format!("✨ {}", text.bright_green().bold())
}

pub fn tavern_cheer() -> String {
    "🍺🍺 Victory is yours — raise a mug!".bright_green().bold().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strip_ansi(s: &str) -> String {
        let mut out = String::with_capacity(s.len());
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\x1b' {
                while let Some(&next) = chars.peek() {
                    chars.next();
                    if next == 'm' {
                        break;
                    }
                }
            } else {
                out.push(c);
            }
        }
        out
    }

    fn visible_width(s: &str) -> usize {
        strip_ansi(s).width()
    }

    #[test]
    fn inner_padding_is_fixed_width() {
        assert_eq!(pad_inner("hi").width(), BOX_INNER_WIDTH);
        assert_eq!(pad_inner("🦀 quest").width(), BOX_INNER_WIDTH);
    }

    #[test]
    fn center_padding_is_fixed_width() {
        let centered = pad_center("RUST QUEST");
        assert_eq!(centered.width(), BOX_INNER_WIDTH);
        assert!(centered.starts_with(' '));
        assert!(centered.ends_with(' '));
    }

    #[test]
    fn wrap_splits_long_lines() {
        let long = "💡 Study the runes, then face 👹 Borrow Checker Warden in the hall.";
        let rows = wrap_inner(long);
        assert!(rows.len() >= 2);
        for row in &rows {
            assert_eq!(row.width(), BOX_INNER_WIDTH);
        }
    }

    #[test]
    fn title_banner_rows_align() {
        let inner = title_line_styled();
        assert_eq!(visible_width(&inner), BOX_INNER_WIDTH);
        let subtitle = pad_center("~ learn rust. earn ranks. ~");
        assert_eq!(visible_width(&subtitle), BOX_INNER_WIDTH);
    }

    #[test]
    fn main_menu_frame_ornaments_match_box_width() {
        let frame = main_menu_frame();
        let box_row = box_line("aligned");
        let ornament = menu_ornament();
        assert_eq!(
            visible_width(&ornament),
            visible_width(box_row.lines().next().unwrap())
        );
        assert!(frame.contains("Main Menu"));
        assert!(frame.contains("🕯️"));
    }

    #[test]
    fn top_and_bottom_match_side_rows() {
        let side = box_line("aligned");
        let top = format!("╔{}╗", "═".repeat(horizontal_rule_len()));
        let bottom = format!("╚{}╝", "═".repeat(horizontal_rule_len()));
        for line in side.lines() {
            let w = visible_width(line);
            assert_eq!(w, visible_width(&top));
            assert_eq!(w, visible_width(&bottom));
        }
    }
}
