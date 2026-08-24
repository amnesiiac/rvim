//! Pure statusline segment builder.
//!
//! Turns cached editor state (`StatusContext`) into colored segments for the
//! rich (powerline) or minimal (flat) layout. No I/O, no `Editor`, no
//! terminal — `render_status_line` in `terminal/mod.rs` is the only consumer
//! and does nothing but gather state, call this, and emit ANSI. That keeps
//! both visual modes testable without a terminal.

use crossterm::style::Color;

use crate::editor::Mode;
use crate::theme::Theme;
use crate::ui_glyphs::UiGlyphs;

pub struct StatusSegment {
    pub text: String,
    pub fg: Color,
    pub bg: Color,
    pub bold: bool,
}

/// Segments in draw order. The renderer paints `left`, pads with the
/// statusline background to the terminal width, then paints `right`.
pub struct StatusLineContent {
    pub left: Vec<StatusSegment>,
    pub right: Vec<StatusSegment>,
}

/// Everything the statusline shows, pre-gathered from cached editor state.
/// Render-reads-cache-only rule: callers must not compute git/LSP data here.
pub struct StatusContext<'a> {
    pub mode: Mode,
    /// Preformatted pending count/operator, e.g. "3d" ("" when none).
    pub pending: &'a str,
    /// Macro register currently recording.
    pub recording: Option<char>,
    pub project: Option<&'a str>,
    pub filename: &'a str,
    pub modified: bool,
    pub readonly: bool,
    pub large_file: bool,
    pub branch: Option<&'a str>,
    /// (added, modified) line counts from the cached git diff.
    pub diff: Option<(usize, usize)>,
    /// (errors, warnings) for the current buffer.
    pub diag: (usize, usize),
    pub lang: &'a str,
    pub lsp_attached: bool,
    pub lsp_busy: bool,
    /// Current spinner frame (advanced per LSP notification, never a timer).
    pub lsp_spinner: &'a str,
    /// 1-based for display.
    pub line: usize,
    pub col: usize,
    /// 0..=100 scroll position.
    pub percent: usize,
}

fn mode_color(mode: Mode, theme: &Theme) -> Color {
    match mode {
        Mode::Normal | Mode::Command => theme.ui.statusline_mode_normal,
        Mode::Insert => theme.ui.statusline_mode_insert,
        Mode::Visual | Mode::VisualLine | Mode::VisualBlock => theme.ui.statusline_mode_visual,
        Mode::Replace => theme.ui.statusline_mode_replace,
        _ => theme.ui.statusline_mode_normal,
    }
}

/// Vim parity: command mode keeps showing NORMAL in the statusline.
fn mode_label(mode: Mode) -> &'static str {
    if mode == Mode::Command {
        "NORMAL"
    } else {
        mode.as_str()
    }
}

fn seg(text: String, fg: Color, bg: Color) -> StatusSegment {
    StatusSegment {
        text,
        fg,
        bg,
        bold: false,
    }
}

fn seg_bold(text: String, fg: Color, bg: Color) -> StatusSegment {
    StatusSegment {
        text,
        fg,
        bg,
        bold: true,
    }
}

/// Trailing file-state markers shared by both layouts: modified dot,
/// read-only, large-file, pending operator, macro recording.
fn push_file_markers(
    out: &mut Vec<StatusSegment>,
    ctx: &StatusContext,
    glyphs: &UiGlyphs,
    theme: &Theme,
    bg: Color,
    minimal: bool,
) {
    let fg = theme.ui.statusline_fg;
    if ctx.modified {
        out.push(seg(format!(" {}", glyphs.modified), theme.git.modified, bg));
    }
    if ctx.readonly {
        out.push(seg(format!(" {}", glyphs.readonly.trim_end()), fg, bg));
    }
    if ctx.large_file {
        out.push(seg(" [large]".to_string(), fg, bg));
    }
    if !ctx.pending.is_empty() {
        out.push(seg(format!(" [{}]", ctx.pending), fg, bg));
    }
    if let Some(register) = ctx.recording {
        // Minimal's prefix opens a bracket that needs closing; rich is a glyph.
        let close = if minimal { "]" } else { "" };
        out.push(seg(
            format!(" {}{}{}", glyphs.recording, register, close),
            theme.diagnostic.error,
            bg,
        ));
    }
}

pub fn build_status_segments(
    ctx: &StatusContext,
    glyphs: &UiGlyphs,
    theme: &Theme,
    minimal: bool,
) -> StatusLineContent {
    if minimal {
        build_minimal(ctx, glyphs, theme)
    } else {
        build_rich(ctx, glyphs, theme)
    }
}

fn build_rich(ctx: &StatusContext, glyphs: &UiGlyphs, theme: &Theme) -> StatusLineContent {
    let mode_bg = mode_color(ctx.mode, theme);
    let base_bg = theme.ui.statusline_bg;
    let base_fg = theme.ui.statusline_fg;
    let section_bg = theme.ui.statusline_section_bg;

    let mut left = Vec::new();
    left.push(seg_bold(
        format!(" {} ", mode_label(ctx.mode)),
        base_bg,
        mode_bg,
    ));

    // Mode → git (or file) powerline transition.
    let after_mode_bg = if ctx.branch.is_some() {
        section_bg
    } else {
        base_bg
    };
    left.push(seg(glyphs.sep_left.to_string(), mode_bg, after_mode_bg));

    if let Some(branch) = ctx.branch {
        left.push(seg(
            format!(" {}{}", glyphs.branch, branch),
            base_fg,
            section_bg,
        ));
        if let Some((added, modified)) = ctx.diff {
            if added > 0 {
                left.push(seg(format!(" +{}", added), theme.git.added, section_bg));
            }
            if modified > 0 {
                left.push(seg(
                    format!(" ~{}", modified),
                    theme.git.modified,
                    section_bg,
                ));
            }
        }
        left.push(seg(" ".to_string(), base_fg, section_bg));
        left.push(seg(glyphs.sep_left.to_string(), section_bg, base_bg));
    }

    let project = ctx.project.map(|p| format!("[{}] ", p)).unwrap_or_default();
    left.push(seg(
        format!(" {}{}", project, ctx.filename),
        base_fg,
        base_bg,
    ));
    push_file_markers(&mut left, ctx, glyphs, theme, base_bg, false);

    let mut right = Vec::new();
    let (errors, warnings) = ctx.diag;
    if errors > 0 {
        right.push(seg(
            format!("{}{} ", glyphs.diag_error, errors),
            theme.diagnostic.error,
            base_bg,
        ));
    }
    if warnings > 0 {
        right.push(seg(
            format!("{}{} ", glyphs.diag_warn, warnings),
            theme.diagnostic.warning,
            base_bg,
        ));
    }
    right.push(seg(glyphs.sep_right.to_string(), section_bg, base_bg));
    let lsp = if !ctx.lsp_attached {
        String::new()
    } else if ctx.lsp_busy {
        format!(" {}", ctx.lsp_spinner)
    } else {
        format!(" {}", glyphs.lsp_ok)
    };
    right.push(seg(format!(" {}{} ", ctx.lang, lsp), base_fg, section_bg));
    right.push(seg(glyphs.sep_right.to_string(), mode_bg, section_bg));
    right.push(seg_bold(
        format!(" {}:{}  {}% ", ctx.line, ctx.col, ctx.percent),
        base_bg,
        mode_bg,
    ));

    StatusLineContent { left, right }
}

fn build_minimal(ctx: &StatusContext, glyphs: &UiGlyphs, theme: &Theme) -> StatusLineContent {
    let bg = theme.ui.statusline_bg;
    let fg = theme.ui.statusline_fg;

    let mut left = Vec::new();
    left.push(seg_bold(
        format!(" {}", mode_label(ctx.mode)),
        mode_color(ctx.mode, theme),
        bg,
    ));
    if let Some(branch) = ctx.branch {
        left.push(seg(format!("{}{}", glyphs.item_sep, branch), fg, bg));
        if let Some((added, modified)) = ctx.diff {
            if added > 0 {
                left.push(seg(format!(" +{}", added), theme.git.added, bg));
            }
            if modified > 0 {
                left.push(seg(format!(" ~{}", modified), theme.git.modified, bg));
            }
        }
    }
    let project = ctx.project.map(|p| format!("[{}] ", p)).unwrap_or_default();
    left.push(seg(
        format!("{}{}{}", glyphs.item_sep, project, ctx.filename),
        fg,
        bg,
    ));
    push_file_markers(&mut left, ctx, glyphs, theme, bg, true);

    let mut right = Vec::new();
    let (errors, warnings) = ctx.diag;
    if errors > 0 {
        right.push(seg(
            format!("{}{} ", glyphs.diag_error, errors),
            theme.diagnostic.error,
            bg,
        ));
    }
    if warnings > 0 {
        right.push(seg(
            format!("{}{} ", glyphs.diag_warn, warnings),
            theme.diagnostic.warning,
            bg,
        ));
    }
    right.push(seg(format!(" {}", ctx.lang), fg, bg));
    if ctx.lsp_attached {
        let indicator = if ctx.lsp_busy {
            ctx.lsp_spinner
        } else {
            glyphs.lsp_ok
        };
        right.push(seg(format!(" {}", indicator), fg, bg));
    }
    right.push(seg(format!(" {}:{} ", ctx.line, ctx.col), fg, bg));

    StatusLineContent { left, right }
}

/// Terminal columns occupied by the segments — unicode-width, never `.len()`.
/// Icons are multi-byte and CJK filenames are double-width; byte math
/// misaligns the right side.
pub fn display_width(segments: &[StatusSegment]) -> usize {
    use unicode_width::UnicodeWidthStr;
    segments
        .iter()
        .map(|s| UnicodeWidthStr::width(s.text.as_str()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui_glyphs::{MINIMAL, RICH};

    fn theme() -> Theme {
        Theme::default()
    }

    fn ctx_base<'a>() -> StatusContext<'a> {
        StatusContext {
            mode: Mode::Normal,
            pending: "",
            recording: None,
            project: Some("rvim"),
            filename: "mod.rs",
            modified: true,
            readonly: false,
            large_file: false,
            branch: Some("master"),
            diff: Some((12, 3)),
            diag: (2, 1),
            lang: "rust",
            lsp_attached: true,
            lsp_busy: false,
            lsp_spinner: "",
            line: 128,
            col: 14,
            percent: 93,
        }
    }

    fn all_text(content: &StatusLineContent) -> String {
        content
            .left
            .iter()
            .chain(content.right.iter())
            .map(|s| s.text.as_str())
            .collect()
    }

    #[test]
    fn rich_layout_has_mode_block_and_powerline_transitions() {
        let content = build_status_segments(&ctx_base(), &RICH, &theme(), false);
        assert!(content.left[0].text.contains("NORMAL"));
        assert!(content.left[0].bold);
        let all = all_text(&content);
        assert!(all.contains("\u{e0b0}"), "left powerline separator present");
        assert!(all.contains("master"), "branch renders");
        assert!(all.contains("●"), "modified dot renders");
        assert!(
            all.contains("+12") && all.contains("~3"),
            "diff stats render"
        );
    }

    #[test]
    fn minimal_layout_is_flat_and_ascii_prefixed() {
        let theme = theme();
        let content = build_status_segments(&ctx_base(), &MINIMAL, &theme, true);
        for s in content.left.iter().chain(content.right.iter()) {
            assert_eq!(s.bg, theme.ui.statusline_bg, "minimal = one background");
        }
        let all = all_text(&content);
        assert!(all.contains("E:2"));
        assert!(all.contains("W:1"));
        assert!(!all.contains("\u{e0b0}"));
    }

    #[test]
    fn zero_diagnostics_are_hidden() {
        let mut ctx = ctx_base();
        ctx.diag = (0, 0);
        for minimal in [false, true] {
            let content =
                build_status_segments(&ctx, UiGlyphs::for_minimal(minimal), &theme(), minimal);
            let all = all_text(&content);
            assert!(!all.contains("E:0"));
            assert!(!all.contains('\u{f057}'));
            assert!(!all.contains('\u{f071}'));
        }
    }

    #[test]
    fn recording_and_pending_are_preserved() {
        let mut ctx = ctx_base();
        ctx.pending = "3d";
        ctx.recording = Some('q');
        let content = build_status_segments(&ctx, &MINIMAL, &theme(), true);
        let all = all_text(&content);
        assert!(all.contains("[3d]"));
        assert!(all.contains("[recording @q]"));

        let content = build_status_segments(&ctx, &RICH, &theme(), false);
        let all = all_text(&content);
        assert!(all.contains("[3d]"));
        assert!(all.contains("@q"));
    }

    #[test]
    fn command_mode_displays_normal_badge() {
        let mut ctx = ctx_base();
        ctx.mode = Mode::Command;
        let content = build_status_segments(&ctx, &RICH, &theme(), false);
        assert!(content.left[0].text.contains("NORMAL"));
    }

    #[test]
    fn display_width_counts_columns_not_bytes() {
        let segs = vec![StatusSegment {
            text: "日本".into(),
            fg: Color::Reset,
            bg: Color::Reset,
            bold: false,
        }];
        // 2 double-width chars = 4 columns (6 bytes).
        assert_eq!(display_width(&segs), 4);
    }
}
