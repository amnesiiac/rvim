use crate::editor::register::RegisterContent;
use crate::editor::{Editor, Mode};
use crate::input::Motion;

#[test]
fn change_line_preserves_existing_indentation() {
    let mut editor = Editor::default();
    editor.replace_buffer_content("  alpha beta\nsecond\n");

    editor.change_line(1, None);

    assert_eq!(editor.buffer().content(), "  \nsecond\n");
    assert_eq!((editor.cursor.line, editor.cursor.col), (0, 2));
    assert_eq!(editor.mode, Mode::Insert);
}

#[test]
fn undo_after_indented_change_line_restores_original_text() {
    let mut editor = Editor::default();
    editor.replace_buffer_content("  alpha beta\nsecond\n");
    editor.change_line(1, None);
    for ch in "replacement".chars() {
        editor.insert_char(ch);
    }
    editor.enter_normal_mode();

    editor.undo();

    assert_eq!(editor.buffer().content(), "  alpha beta\nsecond\n");
    assert_eq!((editor.cursor.line, editor.cursor.col), (0, 2));
}

#[test]
fn redo_after_indented_change_line_returns_to_indentation() {
    let mut editor = Editor::default();
    editor.replace_buffer_content("  alpha beta\nsecond\n");
    editor.change_line(1, None);
    for ch in "replacement".chars() {
        editor.insert_char(ch);
    }
    editor.enter_normal_mode();
    editor.undo();

    editor.redo();

    assert_eq!(editor.buffer().content(), "  replacement\nsecond\n");
    assert_eq!((editor.cursor.line, editor.cursor.col), (0, 2));
}

#[test]
fn counted_linewise_paste_leaves_cursor_on_first_pasted_line() {
    let mut editor = Editor::default();
    editor.replace_buffer_content("alpha\nbeta\n");
    editor.delete_line(1, Some('a'));

    editor.paste_after_count(Some('a'), 2);

    assert_eq!(editor.buffer().content(), "beta\nalpha\nalpha\n");
    assert_eq!((editor.cursor.line, editor.cursor.col), (1, 0));
}

#[test]
fn counted_linewise_paste_is_one_undo_and_redo_change() {
    let mut editor = Editor::default();
    editor.replace_buffer_content("alpha\nbeta\n");
    editor.delete_line(1, Some('a'));
    editor.paste_after_count(Some('a'), 2);

    editor.undo();
    assert_eq!(editor.buffer().content(), "beta\n");
    assert_eq!((editor.cursor.line, editor.cursor.col), (0, 0));

    editor.redo();
    assert_eq!(editor.buffer().content(), "beta\nalpha\nalpha\n");
    assert_eq!((editor.cursor.line, editor.cursor.col), (0, 0));
}

#[test]
fn counted_linewise_paste_before_is_one_undo_change() {
    let mut editor = Editor::default();
    editor.replace_buffer_content("alpha\nbeta\n");
    editor.delete_line(1, Some('a'));
    editor.paste_before_count(Some('a'), 2);

    editor.undo();

    assert_eq!(editor.buffer().content(), "beta\n");
    assert_eq!((editor.cursor.line, editor.cursor.col), (0, 0));
}

#[test]
fn counted_multiline_linewise_paste_repeats_complete_blocks() {
    let mut editor = Editor::default();
    editor.replace_buffer_content("alpha\nbeta\ngamma\ndelta\n");
    editor.yank_line(3, Some('a'));

    editor.paste_after_count(Some('a'), 2);

    assert_eq!(
        editor.buffer().content(),
        "alpha\nalpha\nbeta\ngamma\nalpha\nbeta\ngamma\nbeta\ngamma\ndelta\n"
    );
    assert_eq!((editor.cursor.line, editor.cursor.col), (1, 0));
}

#[test]
fn counted_empty_linewise_paste_preserves_each_blank_line() {
    let mut editor = Editor::default();
    editor.replace_buffer_content("\nalpha\n");
    editor.yank_line(1, Some('a'));

    editor.paste_after_count(Some('a'), 2);

    assert_eq!(editor.buffer().content(), "\n\n\nalpha\n");
    assert_eq!((editor.cursor.line, editor.cursor.col), (1, 0));
}

#[test]
fn counted_characterwise_paste_keeps_cursor_on_last_inserted_character() {
    let mut editor = Editor::default();
    editor.replace_buffer_content("abcd\n");
    editor.delete_char_at();
    editor
        .registers
        .set(Some('a'), RegisterContent::Chars("a".to_string()));

    editor.paste_after_count(Some('a'), 2);

    assert_eq!(editor.buffer().content(), "baacd\n");
    assert_eq!((editor.cursor.line, editor.cursor.col), (0, 2));
}

#[test]
fn multiline_characterwise_paste_before_stays_at_insert_start() {
    let mut editor = Editor::default();
    editor.replace_buffer_content("alpha\nbeta\ngamma\n");
    editor.yank_motion(Motion::LineEnd, 2, Some('a'));

    editor.paste_before(Some('a'));

    assert_eq!(editor.buffer().content(), "alpha\nbetaalpha\nbeta\ngamma\n");
    assert_eq!((editor.cursor.line, editor.cursor.col), (0, 0));
}

#[test]
fn linewise_paste_lands_on_first_nonblank_of_inserted_line() {
    let mut editor = Editor::default();
    editor.replace_buffer_content("  alpha\nsecond\n");
    editor.yank_line(1, Some('a'));

    editor.paste_after(Some('a'));

    assert_eq!(editor.buffer().content(), "  alpha\n  alpha\nsecond\n");
    assert_eq!((editor.cursor.line, editor.cursor.col), (1, 2));
}

#[test]
fn linewise_paste_after_unterminated_last_line_restores_final_newline() {
    let mut editor = Editor::default();
    editor.replace_buffer_content("abc");
    editor.yank_line(1, Some('a'));

    editor.paste_after(Some('a'));
    editor.paste_after(Some('a'));

    assert_eq!(editor.buffer().content(), "abc\nabc\nabc\n");
}

#[test]
fn linewise_paste_after_unterminated_last_line_remains_undoable() {
    let mut editor = Editor::default();
    editor.replace_buffer_content("abc");
    editor.yank_line(1, Some('a'));
    editor.paste_after(Some('a'));

    // The no-EOL source text is normalized on load (fixendofline), so undo
    // restores the terminated form.
    editor.undo();
    assert_eq!(editor.buffer().content(), "abc\n");

    editor.redo();
    assert_eq!(editor.buffer().content(), "abc\nabc\n");
}

#[test]
fn counted_delete_to_line_end_from_column_zero_is_linewise() {
    let mut editor = Editor::default();
    editor.replace_buffer_content("alpha\nbeta\ngamma\n");

    editor.delete_motion(Motion::LineEnd, 2, Some('a'));

    assert_eq!(editor.buffer().content(), "gamma\n");
    assert_eq!((editor.cursor.line, editor.cursor.col), (0, 0));

    editor.paste_after(Some('a'));
    assert_eq!(editor.buffer().content(), "gamma\nalpha\nbeta\n");
}

// Issue #272 (:h cw): cw on a non-blank changes only up to the end of the
// word, excluding the trailing whitespace the w motion covers.
#[test]
fn change_word_excludes_trailing_whitespace() {
    let mut editor = Editor::default();
    editor.replace_buffer_content("abc def  # text\n");
    editor.cursor.col = 4;

    editor.change_motion(Motion::WordForward, 1, None);

    assert_eq!(editor.buffer().content(), "abc   # text\n");
    assert_eq!((editor.cursor.line, editor.cursor.col), (0, 4));
    assert_eq!(editor.mode, Mode::Insert);
}

// Unlike ce, cw on the last char of a word changes just that char instead
// of jumping to the next word's end.
#[test]
fn change_word_at_last_char_changes_only_that_char() {
    let mut editor = Editor::default();
    editor.replace_buffer_content("ab cd ef\n");
    editor.cursor.col = 1;

    editor.change_motion(Motion::WordForward, 1, None);

    assert_eq!(editor.buffer().content(), "a cd ef\n");
    assert_eq!((editor.cursor.line, editor.cursor.col), (0, 1));
}

// With a count, the stand-still at word end consumes the first count
// (Vim's end_word "stop" flag), so c2w from a word end changes one word
// less than c2e would.
#[test]
fn counted_change_word_from_word_end_consumes_first_count_in_place() {
    let mut editor = Editor::default();
    editor.replace_buffer_content("ab cd ef\n");
    editor.cursor.col = 1;

    editor.change_motion(Motion::WordForward, 2, None);

    assert_eq!(editor.buffer().content(), "a ef\n");
}

// On whitespace the special case does not apply: cw keeps the exclusive w
// range and changes the blanks up to the next word start.
#[test]
fn change_word_on_whitespace_keeps_exclusive_range() {
    let mut editor = Editor::default();
    editor.replace_buffer_content("ab   cd\n");
    editor.cursor.col = 2;

    editor.change_motion(Motion::WordForward, 1, None);

    assert_eq!(editor.buffer().content(), "abcd\n");
    assert_eq!((editor.cursor.line, editor.cursor.col), (0, 2));
}

#[test]
fn change_big_word_excludes_trailing_whitespace() {
    let mut editor = Editor::default();
    editor.replace_buffer_content("a.b c.d  e\n");

    editor.change_motion(Motion::BigWordForward, 1, None);

    assert_eq!(editor.buffer().content(), " c.d  e\n");
    assert_eq!((editor.cursor.line, editor.cursor.col), (0, 0));
}
