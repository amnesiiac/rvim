use crate::editor::Editor;
use crate::input::key_notation::parse_key_sequence;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

fn editor_with_macro(register: char, notation: &str) -> Editor {
    let mut editor = Editor::default();
    editor
        .macros
        .set_macro(register, parse_key_sequence(notation).expect(notation));
    editor
}

#[test]
fn macros_view_lists_recorded_macros_as_notation() {
    let mut editor = editor_with_macro('a', "0f,ci\"hi<Esc>j");
    editor
        .macros
        .set_macro('b', parse_key_sequence("dw").unwrap());

    editor.open_macros_view();

    assert_eq!(editor.buffer().display_name(), "[macros]");
    assert!(editor.buffer().is_read_only());
    let content = editor.buffer().content();
    assert!(content.contains("@a  0f,ci\"hi<Esc>j"));
    assert!(content.contains("@b  dw"));
}

#[test]
fn macros_view_without_recordings_explains_how_to_record() {
    let mut editor = Editor::default();

    editor.open_macros_view();

    assert!(editor.buffer().content().contains("No macros recorded yet"));
}

#[test]
fn macros_view_marks_unencodable_macros_instead_of_failing() {
    let mut editor = Editor::default();
    editor
        .macros
        .set_macro('q', vec![KeyEvent::new(KeyCode::F(5), KeyModifiers::NONE)]);

    editor.open_macros_view();

    assert!(editor.buffer().content().contains("@q  (not editable:"));
}

#[test]
fn macro_edit_buffer_shows_notation_and_saves_back_to_register() {
    let mut editor = editor_with_macro('a', "ciw<Esc>");

    editor
        .open_macro_edit_buffer('a')
        .expect("open edit buffer");

    assert_eq!(editor.buffer().display_name(), "[macro-a]");
    assert!(!editor.buffer().is_read_only());
    assert_eq!(editor.buffer().content(), "ciw<Esc>");

    editor.replace_buffer_content("2ciw<Esc>");
    editor.buffer_mut().dirty = true;
    let message = editor.save_macro_edit_buffer().expect("save");

    assert!(message.contains("Macro @a updated"));
    assert_eq!(
        editor.macros.get_macro('a'),
        Some(&parse_key_sequence("2ciw<Esc>").unwrap())
    );
    assert!(!editor.buffer().dirty, "successful save clears dirty");
}

#[test]
fn macro_edit_of_empty_register_creates_a_new_macro() {
    let mut editor = Editor::default();

    editor
        .open_macro_edit_buffer('z')
        .expect("open edit buffer");
    assert_eq!(editor.buffer().content(), "");

    editor.replace_buffer_content("ggVG");
    editor.save_macro_edit_buffer().expect("save");

    assert_eq!(
        editor.macros.get_macro('z'),
        Some(&parse_key_sequence("ggVG").unwrap())
    );
}

#[test]
fn macro_edit_save_rejects_bad_notation_and_preserves_register() {
    let mut editor = editor_with_macro('a', "x");
    editor
        .open_macro_edit_buffer('a')
        .expect("open edit buffer");

    editor.replace_buffer_content("d<Esc");
    editor.buffer_mut().dirty = true;
    let error = editor.save_macro_edit_buffer().expect_err("bad notation");

    assert!(error.contains("unterminated"));
    assert_eq!(
        editor.macros.get_macro('a'),
        Some(&parse_key_sequence("x").unwrap()),
        "register must be untouched on parse errors"
    );
    assert!(editor.buffer().dirty, "failed save must not clear dirty");
}

#[test]
fn macro_edit_saving_empty_content_clears_the_register() {
    let mut editor = editor_with_macro('a', "x");
    editor
        .open_macro_edit_buffer('a')
        .expect("open edit buffer");

    editor.replace_buffer_content("");
    let message = editor.save_macro_edit_buffer().expect("save");

    assert!(message.contains("cleared"));
    assert_eq!(editor.macros.get_macro('a'), None);
}

#[test]
fn macro_edit_is_blocked_while_recording() {
    let mut editor = Editor::default();
    editor.macros.start_recording('q');

    let error = editor.open_macro_edit_buffer('a').expect_err("recording");

    assert!(error.contains("recording"));
}

#[test]
fn macro_edit_rejects_invalid_registers_and_unencodable_macros() {
    let mut editor = Editor::default();
    assert!(editor.open_macro_edit_buffer('A').is_err());
    assert!(editor.open_macro_edit_buffer('1').is_err());

    editor
        .macros
        .set_macro('q', vec![KeyEvent::new(KeyCode::F(5), KeyModifiers::NONE)]);
    let error = editor.open_macro_edit_buffer('q').expect_err("unencodable");
    assert!(error.contains("cannot be edited"));
}

#[test]
fn wrapped_edit_content_joins_lines_before_parsing() {
    let mut editor = Editor::default();
    editor
        .open_macro_edit_buffer('a')
        .expect("open edit buffer");

    editor.replace_buffer_content("gg\nVG\n");
    editor.save_macro_edit_buffer().expect("save");

    assert_eq!(
        editor.macros.get_macro('a'),
        Some(&parse_key_sequence("ggVG").unwrap())
    );
}
