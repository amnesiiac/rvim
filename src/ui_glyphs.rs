//! Shared glyph tables for rich (Nerd Font) vs minimal (ASCII/basic-Unicode)
//! UI chrome. One layout code path per surface; only the table differs.
//! Minimal doubles as the no-Nerd-Font fallback, so it must avoid
//! private-use-area codepoints entirely (enforced by test below).

pub struct UiGlyphs {
    /// Powerline transition, left side (rendered with fg = previous segment's
    /// bg and bg = next segment's bg).
    pub sep_left: &'static str,
    /// Powerline transition, right side.
    pub sep_right: &'static str,
    /// Item separator inside a flat segment (minimal statusline).
    pub item_sep: &'static str,
    /// Git branch prefix.
    pub branch: &'static str,
    /// Modified-buffer marker (replaces "[+]").
    pub modified: &'static str,
    /// Read-only marker.
    pub readonly: &'static str,
    /// Macro-recording marker prefix (register char is appended by caller).
    pub recording: &'static str,
    /// Diagnostic count prefixes.
    pub diag_error: &'static str,
    pub diag_warn: &'static str,
    /// LSP idle indicator.
    pub lsp_ok: &'static str,
    /// LSP busy indicator frames; advanced per LSP notification
    /// (event-driven — never on a timer).
    pub lsp_busy_frames: &'static [&'static str],
}

pub static RICH: UiGlyphs = UiGlyphs {
    sep_left: "\u{e0b0}",
    sep_right: "\u{e0b2}",
    item_sep: " ",
    branch: "\u{e0a0} ",
    modified: "●",
    readonly: "\u{f023} ",
    recording: "\u{f111} @",
    diag_error: "\u{f057} ",
    diag_warn: "\u{f071} ",
    lsp_ok: "✓",
    lsp_busy_frames: &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧"],
};

pub static MINIMAL: UiGlyphs = UiGlyphs {
    sep_left: "",
    sep_right: "",
    item_sep: " · ",
    branch: "",
    modified: "•",
    readonly: "[RO] ",
    recording: "[recording @",
    diag_error: "E:",
    diag_warn: "W:",
    lsp_ok: "✓",
    lsp_busy_frames: &["~"],
};

impl UiGlyphs {
    pub fn for_minimal(minimal: bool) -> &'static UiGlyphs {
        if minimal { &MINIMAL } else { &RICH }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fields(g: &UiGlyphs) -> Vec<&'static str> {
        let mut v = vec![
            g.sep_left,
            g.sep_right,
            g.item_sep,
            g.branch,
            g.modified,
            g.readonly,
            g.recording,
            g.diag_error,
            g.diag_warn,
            g.lsp_ok,
        ];
        v.extend(g.lsp_busy_frames);
        v
    }

    #[test]
    fn minimal_avoids_nerd_font_private_use_codepoints() {
        for s in fields(&MINIMAL) {
            assert!(
                !s.chars()
                    .any(|c| matches!(c as u32, 0xE000..=0xF8FF | 0xF0000..=0xFFFFD)),
                "minimal glyph {s:?} uses a private-use codepoint"
            );
        }
    }

    #[test]
    fn for_minimal_selects_tables() {
        assert!(std::ptr::eq(UiGlyphs::for_minimal(true), &MINIMAL));
        assert!(std::ptr::eq(UiGlyphs::for_minimal(false), &RICH));
    }
}
