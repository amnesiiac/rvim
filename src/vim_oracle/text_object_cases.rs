use super::OracleCase;

/// Text-object cases: operators over iw/aw and friends, verified against
/// real Neovim. Yank results are observed by pasting the register back.
pub(super) const TEXT_OBJECT_CASES: &[OracleCase] = &[
    // Words.
    OracleCase {
        name: "delete inner word",
        initial_text: "one two three\n",
        keys: "wdiw",
    },
    OracleCase {
        name: "delete around word",
        initial_text: "one two three\n",
        keys: "wdaw",
    },
    OracleCase {
        name: "inner word on whitespace deletes the gap",
        initial_text: "one   two\n",
        keys: "3ldiw",
    },
    OracleCase {
        name: "counted delete around words",
        initial_text: "one two three four\n",
        keys: "d2aw",
    },
    OracleCase {
        name: "delete inner big word",
        initial_text: "foo-bar baz\n",
        keys: "diW",
    },
    OracleCase {
        name: "delete around big word",
        initial_text: "foo-bar baz\n",
        keys: "daW",
    },
    OracleCase {
        name: "yank around word and paste",
        initial_text: "one two\n",
        keys: "yawP",
    },
    // Quotes: Vim's quote objects seek FORWARD on the line when the cursor
    // is before the opening quote.
    OracleCase {
        name: "change inner double quotes from before the string",
        initial_text: "say \"hello world\" end\n",
        keys: "ci\"X<Esc>",
    },
    OracleCase {
        name: "delete around double quotes eats trailing space",
        initial_text: "a \"b c\" d\n",
        keys: "da\"",
    },
    OracleCase {
        name: "delete inner single quotes",
        initial_text: "x 'y z' w\n",
        keys: "di'",
    },
    OracleCase {
        name: "delete inner backticks",
        initial_text: "a `b` c\n",
        keys: "di`",
    },
    // Brackets.
    OracleCase {
        name: "delete inner parens",
        initial_text: "f(a, b) g\n",
        keys: "fadi(",
    },
    OracleCase {
        name: "delete around parens",
        initial_text: "f(a, b) g\n",
        keys: "fada(",
    },
    OracleCase {
        name: "inner parens with cursor on opening paren",
        initial_text: "f(a)b\n",
        keys: "f(di(",
    },
    OracleCase {
        name: "inner parens via b alias",
        initial_text: "f(a, b) g\n",
        keys: "fadib",
    },
    OracleCase {
        name: "nested braces inner targets innermost",
        initial_text: "{a {b} c}\n",
        keys: "fbdi{",
    },
    OracleCase {
        name: "nested braces around outer from outer content",
        initial_text: "{a {b} c}\n",
        keys: "fada{",
    },
    OracleCase {
        name: "inner braces via B alias",
        initial_text: "x {y} z\n",
        keys: "fydiB",
    },
    OracleCase {
        name: "delete inner brackets",
        initial_text: "x[1, 2]y\n",
        keys: "f1di[",
    },
    // `<lt>` is the notation for a literal `<` on both replay sides.
    OracleCase {
        name: "delete inner angle brackets",
        initial_text: "a<b>c\n",
        keys: "fbdi<lt>",
    },
    // Paragraphs.
    OracleCase {
        name: "delete inner paragraph",
        initial_text: "one\ntwo\n\nthree\n",
        keys: "dip",
    },
    OracleCase {
        name: "delete around paragraph",
        initial_text: "one\ntwo\n\nthree\n",
        keys: "dap",
    },
    OracleCase {
        name: "inner paragraph on blank lines deletes the gap",
        initial_text: "a\n\n\nb\n",
        keys: "jdip",
    },
    OracleCase {
        name: "around paragraph takes all trailing blanks",
        initial_text: "one\ntwo\n\n\n\nthree\n",
        keys: "dap",
    },
    // Sentences.
    OracleCase {
        name: "delete inner sentence",
        initial_text: "One two. Three four. Five.\n",
        keys: "dis",
    },
    OracleCase {
        name: "delete around sentence",
        initial_text: "One two. Three four. Five.\n",
        keys: "das",
    },
    // Tags.
    OracleCase {
        name: "delete inner tag",
        initial_text: "<div><b>x</b></div>\n",
        keys: "fxdit",
    },
    OracleCase {
        name: "delete around tag",
        initial_text: "<div><b>x</b></div>\n",
        keys: "fxdat",
    },
    // Visual mode composes with text objects.
    OracleCase {
        name: "visual inner parens delete",
        initial_text: "f(a, b) g\n",
        keys: "favi(d",
    },
    OracleCase {
        name: "visual around quotes delete",
        initial_text: "a \"b c\" d\n",
        keys: "fbva\"d",
    },
];
