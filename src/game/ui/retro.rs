//! Retro box-drawing banners and borders.
//!
//! Shared by the hub, quest map, and title screen.
//!
//! LEARN: emoji and CJK are "wide" — count display columns with `unicode_width`, not `.len()`.
//! Layout adapts to terminal width: full box → horizontal rules → plain indented text.

use colored::Colorize;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use super::terminal;

/// Maximum inner width (display columns) when the terminal is wide enough.
pub const BOX_INNER_WIDTH: usize = 54;

const FULL_BOX_MIN_COLS: usize = 64;
const RULE_ONLY_MIN_COLS: usize = 48;
const MIN_INNER_WIDTH: usize = 20;

/// How borders are drawn for the current terminal width.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutStyle {
    /// Unicode box (`╔═╗ ║ ╚═╝`).
    FullBox,
    /// Horizontal rules only — no side borders (narrow terminals).
    RuleOnly,
    /// Indented plain text (very narrow or broken terminals).
    Plain,
}

/// Terminal-aware layout used by all box helpers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetroLayout {
    pub inner_width: usize,
    pub style: LayoutStyle,
}

impl RetroLayout {
    pub const DEFAULT: Self = Self {
        inner_width: BOX_INNER_WIDTH,
        style: LayoutStyle::FullBox,
    };

    /// Visible span of a content row (matches full-box side rows when possible).
    pub fn row_span(&self) -> usize {
        match self.style {
            LayoutStyle::FullBox => self.inner_width + 4,
            LayoutStyle::RuleOnly => self.inner_width + 4,
            LayoutStyle::Plain => self.inner_width + 2,
        }
    }
}

static mut CURRENT_LAYOUT: RetroLayout = RetroLayout::DEFAULT;

fn inner_width() -> usize {
    unsafe { CURRENT_LAYOUT.inner_width }
}

fn layout_style() -> LayoutStyle {
    unsafe { CURRENT_LAYOUT.style }
}

/// Re-read terminal size and cache layout (call before drawing screens; on resize).
pub fn refresh_layout() -> RetroLayout {
    let cols = crossterm::terminal::size().map(|(c, _)| c).unwrap_or(80);
    refresh_layout_from_cols(cols)
}

/// Set layout from a column count (used by [`refresh_layout`] and tests).
pub fn refresh_layout_from_cols(cols: u16) -> RetroLayout {
    let layout = layout_from_cols(cols);
    unsafe {
        CURRENT_LAYOUT = layout;
    }
    layout
}

pub fn current_layout() -> RetroLayout {
    unsafe { CURRENT_LAYOUT }
}

pub fn layout_from_cols(cols: u16) -> RetroLayout {
    let cols = cols as usize;
    let style = if cols >= FULL_BOX_MIN_COLS {
        LayoutStyle::FullBox
    } else if cols >= RULE_ONLY_MIN_COLS {
        LayoutStyle::RuleOnly
    } else {
        LayoutStyle::Plain
    };
    let inner_width = match style {
        LayoutStyle::Plain => cols.saturating_sub(2).max(MIN_INNER_WIDTH),
        _ => cols
            .saturating_sub(4)
            .clamp(MIN_INNER_WIDTH, BOX_INNER_WIDTH),
    };
    RetroLayout { inner_width, style }
}

fn pad_to(text: &str, width: usize) -> String {
    let w = text.width();
    if w >= width {
        text.to_string()
    } else {
        format!("{}{}", text, " ".repeat(width - w))
    }
}

/// Pad plain text to the current inner width (display columns).
pub fn pad_inner(text: &str) -> String {
    pad_to(text, inner_width())
}

/// Center plain text within the current inner width (display columns).
pub fn pad_center(text: &str) -> String {
    let max = inner_width();
    let w = text.width();
    if w >= max {
        text.to_string()
    } else {
        let pad_total = max - w;
        let left = pad_total / 2;
        format!(
            "{}{}{}",
            " ".repeat(left),
            text,
            " ".repeat(pad_total - left)
        )
    }
}

/// Center `plain`, then swap `needle` for its styled ANSI version (same visible width).
/// LEARN: pad with plain text first — ANSI escape codes must not affect column math.
fn center_and_style(plain: &str, needle: &str, styled: &str) -> String {
    pad_center(plain).replacen(needle, styled, 1)
}

/// Word-wrap plain text to fit the box; each row is padded to the current inner width.
pub fn wrap_inner(text: &str) -> Vec<String> {
    let max = inner_width();
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

fn horizontal_rule_len() -> usize {
    // LEARN: top/bottom use `═`.repeat(this); side rows add 4 columns of border + padding.
    inner_width() + 2
}

fn horizontal_rule_char() -> char {
    match layout_style() {
        LayoutStyle::FullBox => '═',
        LayoutStyle::RuleOnly => '─',
        LayoutStyle::Plain => '─',
    }
}

fn draw_horizontal_rule() -> String {
    horizontal_rule_char()
        .to_string()
        .repeat(horizontal_rule_len())
}

fn side_indent() -> &'static str {
    match layout_style() {
        LayoutStyle::FullBox => "",
        LayoutStyle::RuleOnly => "  ",
        LayoutStyle::Plain => "  ",
    }
}

pub fn title_banner() -> String {
    let subtitle = if terminal::use_emoji() {
        "~ learn rust. earn ranks. ~"
    } else {
        "learn rust. earn ranks."
    };
    match layout_style() {
        LayoutStyle::Plain => {
            let title_plain = if terminal::use_emoji() {
                "🧙 RUST QUEST ⚔️"
            } else {
                "RUST QUEST"
            };
            format!(
                "\n{}\n{}\n",
                title_plain.red().bold(),
                subtitle.bright_white()
            )
        }
        _ => format!(
            "{}\n{}\n{}\n{}",
            box_top_open(),
            box_border(&title_line_styled()),
            box_border(&pad_center(subtitle)),
            box_bottom()
        ),
    }
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

fn box_top_open() -> String {
    match layout_style() {
        LayoutStyle::FullBox => format!("╔{}╗", draw_horizontal_rule())
            .bright_cyan()
            .to_string(),
        LayoutStyle::RuleOnly => draw_horizontal_rule().bright_cyan().to_string(),
        LayoutStyle::Plain => String::new(),
    }
}

pub fn box_top(title: &str) -> String {
    match layout_style() {
        LayoutStyle::Plain => format!("\n{}\n", title.bright_cyan().bold()),
        _ => format!("{}\n{}", box_top_open(), box_line(title)),
    }
}

pub fn box_top_centered(title: &str) -> String {
    match layout_style() {
        LayoutStyle::Plain => format!("\n{}\n", title.bright_cyan().bold()),
        _ => format!("{}\n{}", box_top_open(), box_border(&pad_center(title))),
    }
}

fn menu_rule_line() -> String {
    let span = current_layout().row_span();
    match layout_style() {
        LayoutStyle::Plain => String::new(),
        LayoutStyle::RuleOnly => "─".repeat(span),
        LayoutStyle::FullBox => {
            let w = horizontal_rule_len() + 2;
            format!("◆{}◆", "─".repeat(w.saturating_sub(2)))
        }
    }
}

/// Main menu header — horizontal rules + title; dialoguer renders the live selection below.
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

    match layout_style() {
        LayoutStyle::Plain => format!(
            "\n{}\n{}\n",
            title.bright_cyan().bold(),
            hint.bright_white().dimmed()
        ),
        _ => {
            let rule = menu_rule_line().bright_magenta();
            format!(
                "\n{}\n{}\n{}\n{}\n{}",
                rule,
                pad_center(title).bright_cyan().bold(),
                pad_center(hint).bright_white().dimmed(),
                rule,
                rule
            )
        }
    }
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

/// Draw borders around already-padded inner content (may include ANSI color codes).
pub fn box_border(inner: &str) -> String {
    match layout_style() {
        LayoutStyle::FullBox => format!("{}{}{}", "║ ".bright_cyan(), inner, " ║".bright_cyan()),
        LayoutStyle::RuleOnly => format!("{}{}", side_indent(), inner),
        LayoutStyle::Plain => format!("{}{}", side_indent(), inner.trim_end()),
    }
}

pub fn box_bottom() -> String {
    match layout_style() {
        LayoutStyle::FullBox => format!("╚{}╝", draw_horizontal_rule())
            .bright_cyan()
            .to_string(),
        LayoutStyle::RuleOnly => draw_horizontal_rule().bright_cyan().to_string(),
        LayoutStyle::Plain => String::new(),
    }
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
    "⚔️🛡️ Clean parry — true lore lands!"
        .bright_green()
        .bold()
        .to_string()
}

pub fn final_gambit(emoji: &str, name: &str) -> String {
    format!("💣 Final gambit — {} {} hurls a logic bomb!", emoji, name)
        .bright_red()
        .bold()
        .to_string()
}

pub fn victory_flash(text: &str) -> String {
    format!("✨ {}", text.bright_green().bold())
}

pub fn tavern_cheer() -> String {
    "🍺🍺 Victory is yours — raise a mug!"
        .bright_green()
        .bold()
        .to_string()
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

    fn use_layout(cols: u16) {
        refresh_layout_from_cols(cols);
    }

    #[test]
    fn inner_padding_is_fixed_width() {
        use_layout(80);
        assert_eq!(pad_inner("hi").width(), BOX_INNER_WIDTH);
        assert_eq!(pad_inner("🦀 quest").width(), BOX_INNER_WIDTH);
    }

    #[test]
    fn center_padding_is_fixed_width() {
        use_layout(80);
        let centered = pad_center("RUST QUEST");
        assert_eq!(centered.width(), BOX_INNER_WIDTH);
        assert!(centered.starts_with(' '));
        assert!(centered.ends_with(' '));
    }

    #[test]
    fn wrap_splits_long_lines() {
        use_layout(80);
        let long = "💡 Study the runes, then face 👹 Borrow Checker Warden in the hall.";
        let rows = wrap_inner(long);
        assert!(rows.len() >= 2);
        for row in &rows {
            assert_eq!(row.width(), BOX_INNER_WIDTH);
        }
    }

    #[test]
    fn title_banner_rows_align() {
        use_layout(80);
        let inner = title_line_styled();
        assert_eq!(visible_width(&inner), BOX_INNER_WIDTH);
        let subtitle = pad_center("~ learn rust. earn ranks. ~");
        assert_eq!(visible_width(&subtitle), BOX_INNER_WIDTH);
    }

    #[test]
    fn main_menu_frame_is_banner_not_box() {
        use_layout(80);
        let frame = main_menu_frame();
        assert!(!frame.contains('║'));
        assert!(frame.contains("Main Menu"));
        assert!(frame.contains("🕯️"));
    }

    #[test]
    fn top_and_bottom_match_side_rows() {
        use_layout(80);
        let side = box_line("aligned");
        let top = format!("╔{}╗", draw_horizontal_rule());
        let bottom = format!("╚{}╝", draw_horizontal_rule());
        for line in side.lines() {
            let w = visible_width(line);
            assert_eq!(w, visible_width(&top));
            assert_eq!(w, visible_width(&bottom));
        }
    }

    #[test]
    fn narrow_terminal_uses_rule_only() {
        let layout = layout_from_cols(56);
        assert_eq!(layout.style, LayoutStyle::RuleOnly);
        refresh_layout_from_cols(56);
        let row = box_line("quest node");
        assert!(!row.contains('║'));
        assert!(row.starts_with("  "));
    }

    #[test]
    fn very_narrow_terminal_uses_plain() {
        let layout = layout_from_cols(40);
        assert_eq!(layout.style, LayoutStyle::Plain);
        refresh_layout_from_cols(40);
        let top = box_top("Map");
        assert!(!top.contains('╔'));
        assert!(top.contains("Map"));
    }

    #[test]
    fn layout_shrinks_inner_width_on_medium_terminal() {
        let layout = layout_from_cols(60);
        assert_eq!(layout.inner_width, 54);
        assert_eq!(layout.style, LayoutStyle::RuleOnly);
    }
}
