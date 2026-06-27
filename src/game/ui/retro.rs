//! Retro box-drawing banners and borders.
//!
//! Shared by the hub, quest map, and title screen.
//!
//! LEARN: emoji and CJK are "wide" — count display columns with `unicode_width`, not `.len()`.

use colored::Colorize;
use unicode_width::UnicodeWidthStr;

use super::terminal;

/// Fixed inner width (display columns) for every box row — keeps right edges aligned.
pub const BOX_INNER_WIDTH: usize = 42;

pub fn title_banner() -> String {
    if terminal::use_emoji() {
        format!(
            "{}\n{}\n{}",
            box_top("🦀  R U S T   Q U E S T  ⚔️"),
            box_line("~ learn rust. earn ranks. ~"),
            box_bottom()
        )
    } else {
        format!(
            "{}\n{}\n{}",
            box_top("RUST QUEST"),
            box_line("learn rust. earn ranks."),
            box_bottom()
        )
    }
}

/// Pad plain text to exactly [`BOX_INNER_WIDTH`] display columns.
pub fn pad_inner(text: &str) -> String {
    let w = text.width();
    if w >= BOX_INNER_WIDTH {
        text.to_string()
    } else {
        format!("{}{}", text, " ".repeat(BOX_INNER_WIDTH - w))
    }
}

fn horizontal_rule_len() -> usize {
    // Side row is: `║` + space + inner + space + `║` → inner + 4 columns total.
    BOX_INNER_WIDTH + 2
}

pub fn box_top(title: &str) -> String {
    format!(
        "{}\n{}",
        format!("╔{}╗", "═".repeat(horizontal_rule_len())).bright_cyan(),
        box_line(title)
    )
}

/// Plain-text row inside the box (pad first, then draw borders).
pub fn box_line(text: &str) -> String {
    box_border(&pad_inner(text))
}

/// Styled row: pad plain text for width, then apply color/style to the padded string.
pub fn box_line_styled(plain: &str, style: fn(&str) -> colored::ColoredString) -> String {
    let padded = pad_inner(plain);
    box_border(&style(&padded).to_string())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inner_padding_is_fixed_width() {
        assert_eq!(pad_inner("hi").width(), BOX_INNER_WIDTH);
        assert_eq!(pad_inner("🦀 quest").width(), BOX_INNER_WIDTH);
    }

    #[test]
    fn top_and_bottom_match_side_rows() {
        let side = box_line("aligned");
        let top = format!("╔{}╗", "═".repeat(horizontal_rule_len()));
        let bottom = format!("╚{}╝", "═".repeat(horizontal_rule_len()));
        assert_eq!(side.chars().count(), top.chars().count());
        assert_eq!(side.chars().count(), bottom.chars().count());
    }
}
