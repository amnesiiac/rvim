//! Vim-style key notation ↔ `KeyEvent` codec.
//!
//! One grammar shared by the vim-oracle test harness (notation → events for
//! replay) and the macro lens (recorded events → notation for viewing and
//! editing). Keeping both directions in one module is what guarantees an
//! edited macro parses back to exactly the events the encoder was shown.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Parse a vim-notation key string (`ciw<Esc><C-r>`) into key events.
pub fn parse_key_sequence(input: &str) -> Result<Vec<KeyEvent>, String> {
    let mut keys = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '<' {
            let mut token = String::new();
            let mut closed = false;
            for token_ch in chars.by_ref() {
                if token_ch == '>' {
                    closed = true;
                    break;
                }
                token.push(token_ch);
            }

            if !closed {
                return Err(format!("unterminated key token in `{input}`"));
            }

            keys.push(parse_key_token(&token)?);
        } else {
            keys.push(char_key(ch));
        }
    }

    Ok(keys)
}

fn parse_key_token(token: &str) -> Result<KeyEvent, String> {
    let lower = token.to_ascii_lowercase();
    match lower.as_str() {
        "esc" => Ok(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)),
        "cr" | "enter" | "return" => Ok(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE)),
        "tab" => Ok(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE)),
        "bs" | "backspace" => Ok(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE)),
        "left" => Ok(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE)),
        "right" => Ok(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE)),
        "up" => Ok(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE)),
        "down" => Ok(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE)),
        "del" | "delete" => Ok(KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE)),
        "home" => Ok(KeyEvent::new(KeyCode::Home, KeyModifiers::NONE)),
        "end" => Ok(KeyEvent::new(KeyCode::End, KeyModifiers::NONE)),
        "pageup" => Ok(KeyEvent::new(KeyCode::PageUp, KeyModifiers::NONE)),
        "pagedown" => Ok(KeyEvent::new(KeyCode::PageDown, KeyModifiers::NONE)),
        "space" => Ok(char_key(' ')),
        "lt" => Ok(char_key('<')),
        _ => {
            if let Some(control) = lower
                .strip_prefix("c-")
                .or_else(|| lower.strip_prefix("ctrl-"))
            {
                let mut chars = control.chars();
                if let (Some(ch), None) = (chars.next(), chars.next()) {
                    return Ok(KeyEvent::new(KeyCode::Char(ch), KeyModifiers::CONTROL));
                }
            }
            Err(format!("unsupported key token `<{token}>`"))
        }
    }
}

fn char_key(ch: char) -> KeyEvent {
    if ch.is_ascii_uppercase() {
        KeyEvent::new(KeyCode::Char(ch), KeyModifiers::SHIFT)
    } else {
        KeyEvent::new(KeyCode::Char(ch), KeyModifiers::NONE)
    }
}

/// Encode key events as vim notation, or explain which key can't be written.
pub fn encode_key_sequence(keys: &[KeyEvent]) -> Result<String, String> {
    let mut out = String::new();
    for key in keys {
        out.push_str(&encode_key_event(key)?);
    }
    Ok(out)
}

fn encode_key_event(key: &KeyEvent) -> Result<String, String> {
    match key.code {
        KeyCode::Char(ch) => {
            if key.modifiers.contains(KeyModifiers::CONTROL) {
                // Parsing lowercases the token, so normalize here for roundtrips.
                return Ok(format!("<C-{}>", ch.to_ascii_lowercase()));
            }
            // SHIFT is implied by the character itself ('A', '$'); other
            // modifiers (Alt, Super) have no notation.
            if key.modifiers.difference(KeyModifiers::SHIFT) != KeyModifiers::NONE {
                return Err(format!(
                    "key `{ch}` with modifiers {:?} has no notation",
                    key.modifiers
                ));
            }
            if ch == '<' {
                return Ok("<lt>".to_string());
            }
            Ok(ch.to_string())
        }
        code => {
            if key.modifiers != KeyModifiers::NONE {
                return Err(format!(
                    "key {code:?} with modifiers {:?} has no notation",
                    key.modifiers
                ));
            }
            let token = match code {
                KeyCode::Esc => "Esc",
                KeyCode::Enter => "CR",
                KeyCode::Tab => "Tab",
                KeyCode::Backspace => "BS",
                KeyCode::Left => "Left",
                KeyCode::Right => "Right",
                KeyCode::Up => "Up",
                KeyCode::Down => "Down",
                KeyCode::Delete => "Del",
                KeyCode::Home => "Home",
                KeyCode::End => "End",
                KeyCode::PageUp => "PageUp",
                KeyCode::PageDown => "PageDown",
                other => return Err(format!("key {other:?} has no notation")),
            };
            Ok(format!("<{token}>"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{encode_key_sequence, parse_key_sequence};
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    #[test]
    fn notation_roundtrips_through_events_and_back() {
        let samples = [
            "ciw<Esc>",
            "0f,ci\"hello<Esc>j",
            "2dd@aP",
            "<C-r><C-d>",
            "<CR><Tab><BS><Esc>",
            "<Left><Right><Up><Down>",
            "<Del><Home><End><PageUp><PageDown>",
            "i<lt>div><CR><Esc>",
            "f x",
            "A;<Esc>",
        ];

        for sample in samples {
            let events = parse_key_sequence(sample).expect(sample);
            let encoded = encode_key_sequence(&events).expect(sample);
            assert_eq!(encoded, sample, "notation should roundtrip unchanged");
        }
    }

    #[test]
    fn events_roundtrip_through_notation_and_back() {
        let events = vec![
            KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Char('G'), KeyModifiers::SHIFT),
            KeyEvent::new(KeyCode::Char('r'), KeyModifiers::CONTROL),
            KeyEvent::new(KeyCode::Char('<'), KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE),
            KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE),
        ];

        let notation = encode_key_sequence(&events).expect("encode");
        assert_eq!(notation, "dG<C-r><lt><Esc><Del>");
        assert_eq!(parse_key_sequence(&notation).expect("parse"), events);
    }

    #[test]
    fn uppercase_and_symbol_chars_encode_without_modifier_tokens() {
        // Terminals disagree on whether shifted characters carry SHIFT; the
        // character itself is the source of truth either way.
        let with_shift = KeyEvent::new(KeyCode::Char('$'), KeyModifiers::SHIFT);
        let without = KeyEvent::new(KeyCode::Char('$'), KeyModifiers::NONE);

        assert_eq!(encode_key_sequence(&[with_shift]).unwrap(), "$");
        assert_eq!(encode_key_sequence(&[without]).unwrap(), "$");
    }

    #[test]
    fn control_uppercase_normalizes_to_parseable_lowercase() {
        let event = KeyEvent::new(
            KeyCode::Char('R'),
            KeyModifiers::CONTROL | KeyModifiers::SHIFT,
        );

        let notation = encode_key_sequence(&[event]).expect("encode");
        assert_eq!(notation, "<C-r>");
        assert!(parse_key_sequence(&notation).is_ok());
    }

    #[test]
    fn unrepresentable_keys_report_an_error_instead_of_lossy_output() {
        let alt_char = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::ALT);
        assert!(encode_key_sequence(&[alt_char]).is_err());

        let function_key = KeyEvent::new(KeyCode::F(5), KeyModifiers::NONE);
        assert!(encode_key_sequence(&[function_key]).is_err());
    }

    #[test]
    fn parse_rejects_unterminated_and_unknown_tokens() {
        assert!(parse_key_sequence("d<Esc").is_err());
        assert!(parse_key_sequence("<F5>").is_err());
    }
}
