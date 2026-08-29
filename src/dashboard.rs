//! Start screen shown when Nevi launches with nothing to edit.
//!
//! A pure render condition, not a mode: the screen exists only while
//! `dashboard_active` holds (single untouched scratch buffer, single pane,
//! Normal mode) and disappears forever the moment a real buffer, edit, or
//! mode change arrives. It renders only on frames that already repaint, so
//! it never touches the hot path.
//!
//! Shortcuts while the screen shows: `1`-`9` open a RECENT entry, `h` then
//! `1`-`9` jumps to that harpoon slot (mirroring `<leader>1`-`<leader>4`).

use crate::editor::{Editor, Mode};
use std::path::{Path, PathBuf};

const MAX_RECENT: usize = 5;

/// Per-span color role; the renderer maps these onto theme colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpanStyle {
    /// Brand title — accent color, bold.
    Title,
    /// Accent-colored detail (section headers, icons, project labels).
    Accent,
    /// Shortcut keys and entry numbers — highlight color, bold.
    Key,
    /// Primary text, bold (file names).
    TextBold,
    /// De-emphasized text (tagline, hints, separators).
    Dim,
}

pub struct DashboardSpan {
    pub text: String,
    pub style: SpanStyle,
}

/// One centered line of the start screen. Empty `spans` = blank spacer row.
pub struct DashboardLine {
    pub spans: Vec<DashboardSpan>,
}

impl DashboardLine {
    pub fn width(&self) -> usize {
        self.spans.iter().map(|s| s.text.chars().count()).sum()
    }
}

fn span(style: SpanStyle, text: impl Into<String>) -> DashboardSpan {
    DashboardSpan {
        text: text.into(),
        style,
    }
}

fn blank() -> DashboardLine {
    DashboardLine { spans: Vec::new() }
}

impl Editor {
    /// True while the start screen owns the text area.
    pub fn dashboard_active(&self) -> bool {
        self.mode == Mode::Normal
            && self.buffer_count() == 1
            && self.pane_count() == 1
            && self.buffer().path.is_none()
            && self.buffer().is_empty()
            && !self.buffer().dirty
    }

    /// The numbered RECENT list: highest-scored recently opened files.
    pub fn dashboard_recents(&self) -> Vec<PathBuf> {
        self.recent_files.top(MAX_RECENT)
    }

    /// Harpoon pins with their 1-based slot numbers. Slots keep their
    /// positions (matching `<leader>N`) even when earlier files are missing.
    pub fn dashboard_pins(&self) -> Vec<(usize, PathBuf)> {
        self.harpoon
            .files()
            .iter()
            .enumerate()
            .map(|(idx, p)| {
                (
                    idx + 1,
                    std::fs::canonicalize(p).unwrap_or_else(|_| p.clone()),
                )
            })
            .filter(|(_, p)| p.is_file())
            .collect()
    }

    /// Open the 1-based numbered RECENT entry. Returns false when the number
    /// has no entry, so the caller can let the key fall through.
    pub fn open_dashboard_entry(&mut self, number: usize) -> bool {
        let recents = self.dashboard_recents();
        let Some(path) = number.checked_sub(1).and_then(|i| recents.get(i)) else {
            return false;
        };
        let path = path.clone();
        self.open_file(path).is_ok()
    }

    /// Open harpoon slot `slot` (1-based), the dashboard's `h1`-`h9` keys.
    pub fn open_dashboard_harpoon(&mut self, slot: usize) -> bool {
        let Some(path) = self.harpoon.get_slot(slot).cloned() else {
            return false;
        };
        self.open_file(path).is_ok()
    }
}

/// Display name for a repo checkout: a linked worktree (whose `.git` is a
/// file pointing at `<main>/.git/worktrees/<name>`) resolves to the main
/// repository's name, a normal checkout to its own directory name.
fn repo_display_name(root: &Path) -> String {
    let own_name = || {
        root.file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default()
    };
    let dot_git = root.join(".git");
    if !dot_git.is_file() {
        return own_name();
    }
    let Ok(content) = std::fs::read_to_string(&dot_git) else {
        return own_name();
    };
    let Some(gitdir) = content.strip_prefix("gitdir:") else {
        return own_name();
    };
    let gitdir = gitdir.trim();
    let gitdir = if Path::new(gitdir).is_absolute() {
        PathBuf::from(gitdir)
    } else {
        root.join(gitdir)
    };
    // Walk up to the `.git` component; the main checkout is its parent.
    let mut cur = gitdir.as_path();
    while let Some(parent) = cur.parent() {
        if cur.file_name().is_some_and(|n| n == ".git") {
            return parent
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(own_name);
        }
        cur = parent;
    }
    own_name()
}

/// (display path, project label) for a RECENT entry: path relative to its
/// repo checkout, or file name + parent dir name outside any repo.
fn entry_display(path: &Path) -> (String, String) {
    if let Some(root) = path.parent().and_then(|p| crate::git::find_repo_root(p)) {
        let rel = path
            .strip_prefix(&root)
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| path.display().to_string());
        return (rel, repo_display_name(&root));
    }
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string());
    let label = path
        .parent()
        .and_then(|p| p.file_name())
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    (name, label)
}

fn file_devicon(path: &Path, minimal: bool) -> String {
    if minimal {
        String::new()
    } else {
        let chip = crate::finder::FuzzyFinder::get_file_icon(path);
        format!("{} ", crate::ui_glyphs::devicon_for_chip(chip))
    }
}

/// Build the start screen's lines. The renderer centers each line and colors
/// spans by style; minimal mode drops the Nerd Font glyphs, nothing else.
pub fn dashboard_lines(editor: &Editor) -> Vec<DashboardLine> {
    let minimal = editor.settings.resolved_ui_style().is_minimal();
    let glyphs = editor.ui_glyphs();
    let version = env!("CARGO_PKG_VERSION");

    let mut lines = vec![
        DashboardLine {
            spans: vec![span(SpanStyle::Title, "N E V I")],
        },
        blank(),
        DashboardLine {
            spans: vec![span(
                SpanStyle::Dim,
                format!("v{version} — your vim muscle memory, without the configuration overhead"),
            )],
        },
        blank(),
    ];

    let recents = editor.dashboard_recents();
    if !recents.is_empty() {
        let displays: Vec<(String, String)> = recents.iter().map(|p| entry_display(p)).collect();
        let name_width = displays
            .iter()
            .map(|(n, _)| n.chars().count())
            .max()
            .unwrap_or(0);
        lines.push(DashboardLine {
            spans: vec![span(SpanStyle::Accent, "RECENT")],
        });
        for (i, (name, project)) in displays.iter().enumerate() {
            lines.push(DashboardLine {
                spans: vec![
                    span(SpanStyle::Key, format!("{}  ", i + 1)),
                    span(SpanStyle::TextBold, format!("{name:name_width$}")),
                    span(SpanStyle::Accent, format!("    {project}")),
                ],
            });
        }
        lines.push(blank());
    }

    let pins = editor.dashboard_pins();
    if !pins.is_empty() {
        lines.push(DashboardLine {
            spans: vec![span(SpanStyle::Accent, "HARPOON")],
        });
        let mut spans = Vec::new();
        for (i, (slot, path)) in pins.iter().enumerate() {
            if i > 0 {
                spans.push(span(SpanStyle::Dim, "   "));
            }
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| path.display().to_string());
            spans.push(span(SpanStyle::Key, format!("h{slot} ")));
            let icon = file_devicon(path, minimal);
            if !icon.is_empty() {
                spans.push(span(SpanStyle::Accent, icon));
            }
            spans.push(span(SpanStyle::TextBold, name));
        }
        lines.push(DashboardLine { spans });
        lines.push(blank());
    }

    let mut ready = Vec::new();
    if !glyphs.dashboard_bolt.is_empty() {
        ready.push(span(SpanStyle::Key, format!("{} ", glyphs.dashboard_bolt)));
    }
    let ready_text = match editor.startup_ready_ms {
        Some(ms) => format!("ready in {ms}ms · 0 plugins · v{version}"),
        None => format!("0 plugins · v{version}"),
    };
    ready.push(span(SpanStyle::Dim, ready_text));
    lines.push(DashboardLine { spans: ready });
    lines.push(blank());

    let mut hints = Vec::new();
    for (i, (key, label)) in [
        ("e", "explorer"),
        ("ff", "find files"),
        ("fg", "live grep"),
        (":", "command"),
    ]
    .iter()
    .enumerate()
    {
        if i > 0 {
            hints.push(span(SpanStyle::Dim, " · "));
        }
        hints.push(span(SpanStyle::Key, *key));
        hints.push(span(SpanStyle::Dim, format!(" {label}")));
    }
    lines.push(DashboardLine { spans: hints });

    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recent_files::RecentFiles;

    fn dashboard_editor(name: &str) -> (Editor, PathBuf) {
        let base = std::env::temp_dir().join(format!("nevi-dashboard-test-{name}"));
        let _ = std::fs::remove_dir_all(&base);
        std::fs::create_dir_all(&base).unwrap();
        let mut editor = Editor::default();
        editor.recent_files = RecentFiles::load_from(base.join("recents.json"));
        editor.project_root = Some(base.clone());
        (editor, base)
    }

    fn touch(dir: &Path, name: &str) -> PathBuf {
        let p = dir.join(name);
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(&p, "content").unwrap();
        p
    }

    fn all_text(lines: &[DashboardLine]) -> String {
        lines
            .iter()
            .map(|l| l.spans.iter().map(|s| s.text.as_str()).collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }

    #[test]
    fn dashboard_active_only_for_untouched_single_scratch() {
        let (mut editor, base) = dashboard_editor("active");
        assert!(editor.dashboard_active());

        // Insert mode hides it even before any edit.
        editor.mode = Mode::Insert;
        assert!(!editor.dashboard_active());
        editor.mode = Mode::Normal;

        // Opening a real file kills it.
        let file = touch(&base, "a.rs");
        editor.open_file(file).unwrap();
        assert!(!editor.dashboard_active());
    }

    #[test]
    fn dashboard_active_false_once_scratch_is_dirty() {
        let (mut editor, _base) = dashboard_editor("dirty");
        editor.buffer_mut().dirty = true;
        assert!(!editor.dashboard_active());
    }

    #[test]
    fn recents_and_pins_are_separate_lists() {
        let (mut editor, base) = dashboard_editor("entries");
        let a = touch(&base, "a.rs");
        let b = touch(&base, "b.rs");
        editor.recent_files.record(&a);
        editor.recent_files.record(&b);
        editor.recent_files.record(&b); // b outscores a
        let c = touch(&base, "c.rs");
        editor.harpoon.add_file(&b);
        editor.harpoon.add_file(&c);

        let recents: Vec<_> = editor
            .dashboard_recents()
            .iter()
            .map(|p| p.file_name().unwrap().to_str().unwrap().to_string())
            .collect();
        assert_eq!(recents, vec!["b.rs", "a.rs"]);

        // Pins keep their slot numbers; a pinned file may also be recent.
        let pins: Vec<_> = editor
            .dashboard_pins()
            .iter()
            .map(|(slot, p)| (*slot, p.file_name().unwrap().to_str().unwrap().to_string()))
            .collect();
        assert_eq!(pins, vec![(1, "b.rs".to_string()), (2, "c.rs".to_string())]);
    }

    #[test]
    fn pins_keep_slot_numbers_when_earlier_files_are_missing() {
        let (mut editor, base) = dashboard_editor("slots");
        let gone = base.join("gone.rs");
        let kept = touch(&base, "kept.rs");
        editor.harpoon.add_file(&gone);
        editor.harpoon.add_file(&kept);

        let pins = editor.dashboard_pins();
        assert_eq!(pins.len(), 1);
        assert_eq!(
            pins[0].0, 2,
            "slot 2 stays h2 even when slot 1's file is gone"
        );
    }

    #[test]
    fn open_dashboard_entry_opens_numbered_recent() {
        let (mut editor, base) = dashboard_editor("open");
        let a = touch(&base, "a.rs");
        let b = touch(&base, "b.rs");
        editor.recent_files.record(&a);
        editor.recent_files.record(&a);
        editor.recent_files.record(&b);

        assert!(editor.open_dashboard_entry(2));
        assert_eq!(
            editor.buffer().path.as_deref(),
            Some(std::fs::canonicalize(&b).unwrap().as_path())
        );
        assert!(!editor.dashboard_active());

        let (mut empty, _base) = dashboard_editor("open-empty");
        assert!(
            !empty.open_dashboard_entry(1),
            "no entries → key falls through"
        );
    }

    #[test]
    fn open_dashboard_harpoon_opens_slot() {
        let (mut editor, base) = dashboard_editor("harpoon-open");
        let a = touch(&base, "a.rs");
        let b = touch(&base, "b.rs");
        editor.harpoon.add_file(&a);
        editor.harpoon.add_file(&b);

        assert!(editor.open_dashboard_harpoon(2));
        assert_eq!(
            editor.buffer().path.as_ref().unwrap().file_name().unwrap(),
            "b.rs"
        );
        assert!(
            !editor.open_dashboard_harpoon(9),
            "empty slot falls through"
        );
    }

    #[test]
    fn lines_match_the_brand_layout() {
        let (mut editor, base) = dashboard_editor("lines");
        let a = touch(&base, "src/a.rs");
        editor.recent_files.record(&a);
        let pin = touch(&base, "pin.rs");
        editor.harpoon.add_file(&pin);

        let lines = dashboard_lines(&editor);
        let text = all_text(&lines);

        assert_eq!(lines[0].spans[0].text, "N E V I");
        assert_eq!(lines[0].spans[0].style, SpanStyle::Title);
        assert!(text.contains("your vim muscle memory"));
        assert!(text.contains("RECENT"));
        assert!(text.contains("HARPOON"));
        assert!(text.contains("h1 "), "pins carry h-number shortcuts");
        assert!(text.contains("0 plugins"));
        assert!(text.contains("ff find files"));
    }

    #[test]
    fn recent_entries_show_project_relative_path_and_label() {
        let (mut editor, base) = dashboard_editor("project");
        // Mark the temp base as a project root.
        std::fs::create_dir_all(base.join(".git")).unwrap();
        let file = touch(&base, "src/deep/mod.rs");
        editor.recent_files.record(&file);

        let lines = dashboard_lines(&editor);
        let text = all_text(&lines);
        assert!(
            text.contains("src/deep/mod.rs"),
            "path is project-relative; text={text}"
        );
        assert!(
            text.contains("nevi-dashboard-test-project"),
            "project label is the repo dir name; text={text}"
        );
    }

    #[test]
    fn worktree_recents_are_labeled_with_the_main_repo_name() {
        let (mut editor, base) = dashboard_editor("worktree");
        // Simulate <main>/.git/worktrees/wt plus a linked worktree checkout.
        let main_gitdir = base.join("mainrepo/.git/worktrees/wt");
        std::fs::create_dir_all(&main_gitdir).unwrap();
        let wt_root = base.join("checkouts/wt");
        std::fs::create_dir_all(&wt_root).unwrap();
        std::fs::write(
            wt_root.join(".git"),
            format!("gitdir: {}\n", main_gitdir.display()),
        )
        .unwrap();
        let file = touch(&wt_root, "src/lib.rs");
        editor.recent_files.record(&file);

        let text = all_text(&dashboard_lines(&editor));
        assert!(
            text.contains("mainrepo"),
            "worktree entries carry the main repo's name; text={text}"
        );
        assert!(
            !text.contains("checkouts/wt"),
            "label is a name, not a path"
        );
    }

    #[test]
    fn ready_line_includes_startup_time_when_measured() {
        let (mut editor, _base) = dashboard_editor("ready");
        editor.startup_ready_ms = Some(18);
        let text = all_text(&dashboard_lines(&editor));
        assert!(text.contains("ready in 18ms"));
    }
}
