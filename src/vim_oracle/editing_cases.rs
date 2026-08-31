use super::OracleCase;

/// Editing cases verify both the resulting text and Vim's final cursor/mode.
/// Yank behavior is made observable by pasting the captured register contents.
pub(super) const EDITING_CASES: &[OracleCase] = &[
    OracleCase {
        name: "delete first char on second line",
        initial_text: "alpha\nbeta\n",
        keys: "j0x",
    },
    OracleCase {
        name: "append punctuation at line end",
        initial_text: "alpha\n",
        keys: "A!<Esc>",
    },
    OracleCase {
        name: "delete current line",
        initial_text: "alpha\nbeta\n",
        keys: "dd",
    },
    OracleCase {
        name: "counted char delete",
        initial_text: "abcdef\n",
        keys: "4x",
    },
    OracleCase {
        name: "counted line delete",
        initial_text: "alpha\nbeta\ngamma\n",
        keys: "2dd",
    },
    OracleCase {
        name: "delete to line end",
        initial_text: "alpha beta\n",
        keys: "wD",
    },
    OracleCase {
        name: "insert before cursor",
        initial_text: "alpha\n",
        keys: "iX<Esc>",
    },
    OracleCase {
        name: "append after cursor",
        initial_text: "alpha\n",
        keys: "aX<Esc>",
    },
    OracleCase {
        name: "open line below",
        initial_text: "alpha\n",
        keys: "ochild<Esc>",
    },
    OracleCase {
        name: "open line above",
        initial_text: "alpha\n",
        keys: "Oparent<Esc>",
    },
    OracleCase {
        name: "delete word",
        initial_text: "alpha beta\n",
        keys: "dw",
    },
    OracleCase {
        name: "delete enter motion",
        initial_text: "zero\n    one\n  two\nthree\n",
        keys: "d<CR>",
    },
    OracleCase {
        name: "change inner word",
        initial_text: "alpha beta\n",
        keys: "ciwdone<Esc>",
    },
    OracleCase {
        name: "change current line",
        initial_text: "  alpha beta\nsecond\n",
        keys: "ccreplacement<Esc>",
    },
    OracleCase {
        name: "change current line register shape",
        initial_text: "  alpha beta\nsecond\n",
        keys: "ccreplacement<Esc>p",
    },
    OracleCase {
        name: "counted line change",
        initial_text: "  alpha beta\nsecond\nthird\n",
        keys: "2ccreplacement<Esc>",
    },
    OracleCase {
        name: "undo indented line change",
        initial_text: "  alpha beta\nsecond\n",
        keys: "ccreplacement<Esc>u",
    },
    OracleCase {
        name: "redo indented line change",
        initial_text: "  alpha beta\nsecond\n",
        keys: "ccreplacement<Esc>u<C-r>",
    },
    OracleCase {
        name: "change to line end",
        initial_text: "alpha beta gamma\n",
        keys: "wCreplaced<Esc>",
    },
    OracleCase {
        name: "change to line end register shape",
        initial_text: "alpha beta gamma\n",
        keys: "wCreplaced<Esc>p",
    },
    OracleCase {
        name: "counted change to line end",
        initial_text: "alpha\nbeta\ngamma\n",
        keys: "2Creplaced<Esc>",
    },
    OracleCase {
        name: "counted change to line end past eof",
        initial_text: "alpha\n",
        keys: "2C",
    },
    OracleCase {
        name: "yank current line",
        initial_text: "alpha\nbeta\n",
        keys: "yyp",
    },
    OracleCase {
        name: "linewise yank without final newline",
        initial_text: "abc",
        keys: "yypp",
    },
    OracleCase {
        name: "linewise paste does not expose trailing newline as a line",
        initial_text: "",
        keys: "iabc<Esc>yypj",
    },
    OracleCase {
        name: "yank to line end",
        initial_text: "alpha\nbeta\n",
        keys: "YP",
    },
    OracleCase {
        name: "counted yank to line end",
        initial_text: "alpha\nbeta\ngamma\n",
        keys: "2YP",
    },
    OracleCase {
        name: "counted yank to line end past eof",
        initial_text: "alpha\n",
        keys: "x2Yp",
    },
    OracleCase {
        name: "paste after linewise yank",
        initial_text: "alpha\nbeta\n",
        keys: "ddp",
    },
    OracleCase {
        name: "paste before linewise yank",
        initial_text: "alpha\nbeta\n",
        keys: "ddP",
    },
    OracleCase {
        name: "delete to word end",
        initial_text: "alpha beta gamma\n",
        keys: "wde",
    },
    OracleCase {
        name: "delete to word end register shape",
        initial_text: "alpha beta gamma\n",
        keys: "wdep",
    },
    OracleCase {
        name: "delete to previous word start",
        initial_text: "alpha beta gamma\n",
        keys: "wdb",
    },
    OracleCase {
        name: "delete to previous word start register shape",
        initial_text: "alpha beta gamma\n",
        keys: "wdbp",
    },
    OracleCase {
        name: "delete with line-end motion",
        initial_text: "alpha beta gamma\n",
        keys: "wd$",
    },
    OracleCase {
        name: "counted delete with line-end motion",
        initial_text: "alpha\nbeta\ngamma\n",
        keys: "2d$",
    },
    OracleCase {
        name: "counted delete with line-end motion past eof",
        initial_text: "alpha\n",
        keys: "2d$",
    },
    OracleCase {
        name: "counted delete line-end register",
        initial_text: "alpha\nbeta\ngamma\n",
        keys: "2d$p",
    },
    OracleCase {
        name: "change around word",
        initial_text: "alpha beta gamma\n",
        keys: "wcawdone<Esc>",
    },
    OracleCase {
        name: "change around word register shape",
        initial_text: "alpha beta gamma\n",
        keys: "wcawdone<Esc>p",
    },
    OracleCase {
        name: "counted change around word before operator",
        initial_text: "alpha beta gamma delta\n",
        keys: "w2cawdone<Esc>",
    },
    OracleCase {
        name: "counted change around word after operator",
        initial_text: "alpha beta gamma delta\n",
        keys: "wc2awdone<Esc>",
    },
    OracleCase {
        name: "counted line yank",
        initial_text: "alpha\nbeta\ngamma\ndelta\n",
        keys: "3yyp",
    },
    OracleCase {
        name: "counted linewise paste",
        initial_text: "alpha\nbeta\n",
        keys: "dd2p",
    },
    OracleCase {
        name: "undo counted linewise paste",
        initial_text: "alpha\nbeta\n",
        keys: "dd2pu",
    },
    OracleCase {
        name: "redo counted linewise paste",
        initial_text: "alpha\nbeta\n",
        keys: "dd2pu<C-r>",
    },
    OracleCase {
        name: "undo counted linewise paste before",
        initial_text: "alpha\nbeta\n",
        keys: "dd2Pu",
    },
    OracleCase {
        name: "counted multiline linewise paste",
        initial_text: "alpha\nbeta\ngamma\ndelta\n",
        keys: "3yy2p",
    },
    OracleCase {
        name: "counted empty linewise paste",
        initial_text: "\nalpha\n",
        keys: "yy2p",
    },
    OracleCase {
        name: "counted characterwise paste",
        initial_text: "abcd\n",
        keys: "x2p",
    },
    OracleCase {
        name: "undo counted characterwise paste",
        initial_text: "abcd\n",
        keys: "x2pu",
    },
    OracleCase {
        name: "redo counted characterwise paste",
        initial_text: "abcd\n",
        keys: "x2pu<C-r>",
    },
    OracleCase {
        name: "counted characterwise paste before",
        initial_text: "abcd\n",
        keys: "x2P",
    },
    OracleCase {
        name: "counted characterwise paste after and move",
        initial_text: "abcd\n",
        keys: "x2gp",
    },
    OracleCase {
        name: "multiline characterwise paste after and move",
        initial_text: "alpha\nbeta\ngamma\n",
        keys: "2Ygp",
    },
    OracleCase {
        name: "counted characterwise paste before and move",
        initial_text: "abcd\n",
        keys: "x2gP",
    },
    // Fuzz-found: J/gJ on the last line must be a no-op. The old guard
    // counted the trailing rope sentinel as a joinable line and silently
    // deleted the file's final newline.
    OracleCase {
        name: "join on last line is a no-op",
        initial_text: "alpha\nbeta\n",
        keys: "GJ",
    },
    OracleCase {
        name: "join without space on last line is a no-op",
        initial_text: "alpha\nbeta\n",
        keys: "GgJ",
    },
    // No-EOL buffers must behave like Neovim ('fixendofline' default):
    // the buffer gains the missing final newline on load.
    OracleCase {
        name: "enter in insert at eof without trailing newline",
        initial_text: "abc",
        keys: "A<CR><Esc>",
    },
    OracleCase {
        name: "join on last line without trailing newline",
        initial_text: "alpha\nbeta",
        keys: "GJ",
    },
    // Case-changing operators: gu/gU/g~ with a motion, the doubled line
    // forms, and the ~ quick toggle.
    OracleCase {
        name: "uppercase to word end",
        initial_text: "hello world\n",
        keys: "gUw",
    },
    OracleCase {
        name: "lowercase to word end",
        initial_text: "HELLO World\n",
        keys: "guw",
    },
    OracleCase {
        name: "toggle case to word end",
        initial_text: "Hello world\n",
        keys: "g~w",
    },
    OracleCase {
        name: "uppercase entire line",
        initial_text: "hello World\nnext\n",
        keys: "gUU",
    },
    OracleCase {
        name: "lowercase entire line",
        initial_text: "HELLO World\nnext\n",
        keys: "guu",
    },
    OracleCase {
        name: "toggle case entire line",
        initial_text: "Hello World\nnext\n",
        keys: "g~~",
    },
    OracleCase {
        name: "toggle case single char",
        initial_text: "hello\n",
        keys: "~",
    },
    OracleCase {
        name: "counted toggle case",
        initial_text: "hello\n",
        keys: "3~",
    },
    // Substitute family: X deletes before the cursor, s/S replace and
    // enter insert.
    OracleCase {
        name: "delete char before cursor",
        initial_text: "abcdef\n",
        keys: "3lX",
    },
    OracleCase {
        name: "counted delete before cursor",
        initial_text: "abcdef\n",
        keys: "4l2X",
    },
    OracleCase {
        name: "substitute char",
        initial_text: "abc\n",
        keys: "sX<Esc>",
    },
    OracleCase {
        name: "counted substitute chars",
        initial_text: "abcdef\n",
        keys: "3sXY<Esc>",
    },
    OracleCase {
        name: "substitute line",
        initial_text: "  indented\nnext\n",
        keys: "Sfoo<Esc>",
    },
    OracleCase {
        name: "counted substitute lines",
        initial_text: "one\ntwo\nthree\nfour\n",
        keys: "2Sx<Esc>",
    },
    // :h w special case: dw on the buffer's last word deletes through the
    // end of that word; landing on a real final word stays exclusive.
    OracleCase {
        name: "delete word on last word of buffer",
        initial_text: "one two three\n",
        keys: "wwdw",
    },
    OracleCase {
        name: "delete word landing on short final word",
        initial_text: "one x\n",
        keys: "dw",
    },
    // Dot repeat: `.` replays the last change at the current cursor
    // position, with a typed count replacing the change's own count.
    OracleCase {
        name: "repeat char delete",
        initial_text: "abcdef\n",
        keys: "xl.",
    },
    OracleCase {
        name: "repeat delete word",
        initial_text: "one two three\n",
        keys: "dww.",
    },
    OracleCase {
        name: "repeat insert",
        initial_text: "alpha\nbeta\n",
        keys: "ihi <Esc>j0.",
    },
    OracleCase {
        name: "repeat change word",
        initial_text: "one two\nthree four\n",
        keys: "cwX<Esc>j0.",
    },
    OracleCase {
        name: "repeat with count override",
        initial_text: "abcdef\n",
        keys: "x3.",
    },
    OracleCase {
        name: "repeat keeps original count",
        initial_text: "abcdefgh\n",
        keys: "2x.",
    },
    OracleCase {
        name: "repeat remembers overridden count",
        initial_text: "abcdefghij\n",
        keys: "x3..",
    },
    OracleCase {
        name: "repeat open line below",
        initial_text: "alpha\n",
        keys: "oX<Esc>.",
    },
    OracleCase {
        name: "repeat after undo",
        initial_text: "ab\n",
        keys: "xu.",
    },
    OracleCase {
        name: "repeat paste",
        initial_text: "abc\n",
        keys: "ylp.",
    },
    OracleCase {
        name: "repeat toggle case",
        initial_text: "hello\n",
        keys: "~.",
    },
    OracleCase {
        name: "repeat replace char",
        initial_text: "abcd\n",
        keys: "rXl.",
    },
    OracleCase {
        name: "repeat join",
        initial_text: "a\nb\nc\nd\n",
        keys: "J.",
    },
    OracleCase {
        name: "repeat substitute",
        initial_text: "abc def\n",
        keys: "sX<Esc>w.",
    },
    OracleCase {
        name: "undo reverts whole repeated insert",
        initial_text: "alpha\n",
        keys: "ohi<Esc>.u",
    },
    OracleCase {
        name: "motion between change and repeat is not recorded",
        initial_text: "abc def\n",
        keys: "xww.",
    },
    // gi returns to where insert mode last stopped and resumes inserting.
    OracleCase {
        name: "go to last insert position and insert",
        initial_text: "alpha\nbeta\n",
        keys: "jA!<Esc>gggix<Esc>",
    },
    // Plain joins (the last-line no-op pins above cover the boundary).
    OracleCase {
        name: "join lines with space",
        initial_text: "foo\nbar\nbaz\n",
        keys: "J",
    },
    OracleCase {
        name: "counted join",
        initial_text: "a\nb\nc\nd\n",
        keys: "3J",
    },
    OracleCase {
        name: "join without added space",
        initial_text: "foo \nbar\n",
        keys: "gJ",
    },
    // Linewise gp/gP land the cursor after the pasted text (the charwise
    // forms are pinned by the "characterwise paste ... and move" cases).
    OracleCase {
        name: "linewise paste after and move",
        initial_text: "one\ntwo\n",
        keys: "yygp",
    },
    OracleCase {
        name: "linewise paste before and move",
        initial_text: "one\ntwo\n",
        keys: "yygP",
    },
    // Issue #272: cw on a non-blank must not include the trailing
    // whitespace the w motion covers (:h cw special case).
    OracleCase {
        name: "change word excludes trailing spaces",
        initial_text: "abc def  # text\n",
        keys: "llllcwxyz<Esc>",
    },
    // On the last char of a word, cw changes just that char (unlike ce,
    // which would jump to the next word's end).
    OracleCase {
        name: "change word at last char of word",
        initial_text: "ab cd ef\n",
        keys: "lcwX<Esc>",
    },
    // With a count, the stand-still at word end consumes the first count.
    OracleCase {
        name: "counted change word from word end",
        initial_text: "ab cd ef\n",
        keys: "lc2wX<Esc>",
    },
    OracleCase {
        name: "counted change word excludes trailing spaces",
        initial_text: "abc def ghi jkl\n",
        keys: "c2wX<Esc>",
    },
    // On whitespace the special case does not apply; cw keeps the
    // exclusive w range and changes the blanks up to the next word.
    OracleCase {
        name: "change word on whitespace",
        initial_text: "ab   cd\n",
        keys: "llcwX<Esc>",
    },
    OracleCase {
        name: "change big word excludes trailing spaces",
        initial_text: "a.b c.d  e\n",
        keys: "cWX<Esc>",
    },
    OracleCase {
        name: "change word stops at punctuation boundary",
        initial_text: "abc# def\n",
        keys: "cwX<Esc>",
    },
    // Insert-mode <C-v> takes the next key literally (issue #281's repro
    // inserted "vy" instead of the 0x19 control byte).
    OracleCase {
        name: "ctrl-v inserts literal control char",
        initial_text: "\n",
        keys: "i<C-v><C-y><Esc>",
    },
    OracleCase {
        name: "ctrl-v inserts literal escape and insert continues",
        initial_text: "\n",
        keys: "iA<C-v><Esc>B<Esc>",
    },
    OracleCase {
        name: "ctrl-v inserts literal tab",
        initial_text: "\n",
        keys: "iA<C-v><Tab>B<Esc>",
    },
    // The literal path must bypass auto-pairs: Vim inserts a lone paren.
    OracleCase {
        name: "ctrl-v open paren inserts without auto pair",
        initial_text: "\n",
        keys: "i<C-v>(<Esc>",
    },
    OracleCase {
        name: "ctrl-q aliases ctrl-v literal insert",
        initial_text: "\n",
        keys: "i<C-v><C-y>x<C-q><C-y><Esc>",
    },
];
