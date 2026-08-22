/// Cursor position in the buffer (0-indexed)
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Cursor {
    /// Line number (0-indexed)
    pub line: usize,
    /// Column number (0-indexed)
    pub col: usize,
}

/// Vim `curswant`: the column vertical motions aim for, so `j`/`k` through
/// short or blank lines come back out at the original column. A `goal` of
/// `usize::MAX` means "stick to end of line" (set by `$`).
///
/// The record stores the cursor/buffer state that produced it and is only
/// honored while all of it still matches. Any other motion, edit, undo, or
/// buffer switch changes one of these fields, so the sticky column expires
/// implicitly instead of requiring explicit clears in every cursor-mutation
/// path of the editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DesiredCol {
    /// Column vertical motions aim for (`usize::MAX` = end of line)
    pub goal: usize,
    /// Cursor position when this goal was recorded
    pub at: Cursor,
    /// Buffer the goal was recorded in
    pub buffer_idx: usize,
    /// Buffer version when recorded; edits bump it and expire the goal
    pub buffer_version: u64,
}

impl Cursor {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }

    /// Move cursor up by n lines
    pub fn move_up(&mut self, n: usize) {
        self.line = self.line.saturating_sub(n);
    }

    /// Move cursor down by n lines (caller should clamp to buffer length)
    pub fn move_down(&mut self, n: usize) {
        self.line = self.line.saturating_add(n);
    }

    /// Move cursor left by n columns
    pub fn move_left(&mut self, n: usize) {
        self.col = self.col.saturating_sub(n);
    }

    /// Move cursor right by n columns (caller should clamp to line length)
    pub fn move_right(&mut self, n: usize) {
        self.col = self.col.saturating_add(n);
    }

    /// Set cursor to start of line
    pub fn move_to_line_start(&mut self) {
        self.col = 0;
    }

    /// Set cursor to a specific position
    pub fn set(&mut self, line: usize, col: usize) {
        self.line = line;
        self.col = col;
    }
}
