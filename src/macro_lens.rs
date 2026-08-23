//! Macro Lens — view and edit recorded macros as readable vim notation.
//!
//! `:Macros` renders every recorded register through the key-notation encoder;
//! `:MacroEdit {register}` opens one in an editable scratch buffer whose `:w`
//! parses the notation back into the register. The pure text logic lives here;
//! buffer plumbing stays on `Editor`.

/// Name of the read-only `:Macros` overview buffer.
pub const MACROS_VIEW_BUFFER_NAME: &str = "[macros]";

const EDIT_BUFFER_PREFIX: &str = "[macro-";
const EDIT_BUFFER_SUFFIX: &str = "]";

/// Buffer name for editing one register, e.g. `[macro-a]`.
pub fn edit_buffer_name(register: char) -> String {
    format!("{EDIT_BUFFER_PREFIX}{register}{EDIT_BUFFER_SUFFIX}")
}

/// Recover the register from an edit-buffer name. Deriving it from the name
/// keeps `Editor` free of extra state that buffer closes could orphan.
pub fn register_from_edit_buffer_name(name: &str) -> Option<char> {
    let register = name
        .strip_prefix(EDIT_BUFFER_PREFIX)?
        .strip_suffix(EDIT_BUFFER_SUFFIX)?;
    let mut chars = register.chars();
    match (chars.next(), chars.next()) {
        (Some(ch), None) if ch.is_ascii_lowercase() => Some(ch),
        _ => None,
    }
}

/// Notation from an edit buffer's content. Newlines are dropped (never part of
/// notation — `<CR>` is a token) so long macros may be wrapped for readability.
pub fn notation_from_edit_content(content: &str) -> String {
    content.chars().filter(|ch| *ch != '\n').collect()
}

/// Render the `:Macros` overview from (register, encoded-or-error) pairs.
pub fn render_macros_view(entries: &[(char, Result<String, String>)]) -> String {
    let mut out = String::from("# Macros\n\n");

    if entries.is_empty() {
        out.push_str("No macros recorded yet. Record one with `q{a-z}...q`.\n");
        return out;
    }

    out.push_str("Replay with `@{register}`. Edit with `:MacroEdit {register}`.\n\n");
    for (register, notation) in entries {
        match notation {
            Ok(notation) => {
                out.push_str(&format!("@{register}  {notation}\n"));
            }
            Err(reason) => {
                out.push_str(&format!("@{register}  (not editable: {reason})\n"));
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{
        edit_buffer_name, notation_from_edit_content, register_from_edit_buffer_name,
        render_macros_view,
    };

    #[test]
    fn edit_buffer_names_roundtrip_to_registers() {
        for register in 'a'..='z' {
            let name = edit_buffer_name(register);
            assert_eq!(register_from_edit_buffer_name(&name), Some(register));
        }
    }

    #[test]
    fn non_macro_buffer_names_are_rejected() {
        for name in ["[macros]", "[macro-A]", "[macro-ab]", "[macro-]", "main.rs"] {
            assert_eq!(register_from_edit_buffer_name(name), None, "{name}");
        }
    }

    #[test]
    fn edit_content_drops_newlines_but_keeps_meaningful_spaces() {
        assert_eq!(notation_from_edit_content("ciw<Esc>\n"), "ciw<Esc>");
        assert_eq!(notation_from_edit_content("f x\ndd\n"), "f xdd");
        // A trailing space is a real key (e.g. `f<space>`), not stray whitespace.
        assert_eq!(notation_from_edit_content("f \n"), "f ");
    }

    #[test]
    fn view_lists_macros_and_marks_unencodable_ones() {
        let entries = vec![
            ('a', Ok("0f,ci\"hi<Esc>".to_string())),
            ('q', Err("key F(5) has no notation".to_string())),
        ];

        let view = render_macros_view(&entries);

        assert!(view.contains("@a  0f,ci\"hi<Esc>"));
        assert!(view.contains("@q  (not editable: key F(5) has no notation)"));
    }

    #[test]
    fn empty_view_explains_how_to_record() {
        let view = render_macros_view(&[]);
        assert!(view.contains("No macros recorded yet"));
    }
}
