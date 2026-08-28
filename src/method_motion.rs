//! Method motions `]m` `[m` `]M` `[M` backed by tree-sitter.
//!
//! Deliberate deviation from Vim: Vim's `]m` family runs a Java-shaped brace
//! heuristic (`nv_bracket_block`) that stops at `if` braces and bare `}` in
//! Rust-like code. We jump between tree-sitter function boundaries instead,
//! which is what nvim-treesitter-textobjects users get when they map `]m` to
//! `@function.outer`. Because of this, these motions have native regression
//! tests rather than vim-oracle cases — the divergence is by design.
//!
//! Performance contract: callers pass in the background parse tree as-is and
//! never parse on the keypress. A missing tree (unsupported language,
//! large-file degradation mode) makes the motion fail in place; a stale tree
//! lags the buffer by at most one highlight debounce, and callers clamp the
//! result to the live buffer.

use tree_sitter::Tree;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodBoundary {
    NextStart, // ]m
    PrevStart, // [m
    NextEnd,   // ]M
    PrevEnd,   // [M
}

/// Node kinds that count as a function or method per language.
///
/// Languages not listed (json, toml, css, ...) yield no targets, so the
/// motion fails in place. Rust closures and Python lambdas are excluded so
/// `]m` doesn't stop at every inline callback; JS/TS arrow functions and Go
/// func literals stay in to match nvim-treesitter-textobjects'
/// `@function.outer` captures.
fn function_node_kinds(language: &str) -> &'static [&'static str] {
    match language {
        "rust" => &["function_item"],
        "javascript" | "typescript" | "tsx" => &[
            "function_declaration",
            "generator_function_declaration",
            "function_expression",
            "generator_function",
            "method_definition",
            "arrow_function",
        ],
        "python" => &["function_definition"],
        "go" => &["function_declaration", "method_declaration", "func_literal"],
        "ruby" => &["method", "singleton_method"],
        "php" => &["function_definition", "method_declaration"],
        // SyntaxManager registers tree-sitter-bash as "shell"
        "shell" => &["function_definition"],
        _ => &[],
    }
}

/// Find the [count]'th method boundary relative to `cursor_byte`.
///
/// Returns `(line, char_col)` in the tree's source snapshot, or `None` when
/// the language has no function kinds or there are fewer than `count`
/// boundaries in that direction (Vim fails the whole motion rather than
/// stopping at the last match).
pub fn find_method_boundary(
    tree: &Tree,
    source: &str,
    language: &str,
    cursor_byte: usize,
    boundary: MethodBoundary,
    count: usize,
) -> Option<(usize, usize)> {
    let kinds = function_node_kinds(language);
    if kinds.is_empty() {
        return None;
    }
    let count = count.max(1);
    let want_start = matches!(
        boundary,
        MethodBoundary::NextStart | MethodBoundary::PrevStart
    );

    // Full pre-order walk collecting every function boundary. The tree is
    // already in memory, so this is pointer-chasing only — microseconds even
    // on multi-thousand-line files (:FlightRecorder shows the real cost).
    // Nested functions come out of order here, hence the sort below.
    let mut targets: Vec<(usize, (usize, usize))> = Vec::new();
    let mut walker = tree.walk();
    'walk: loop {
        let node = walker.node();
        if kinds.contains(&node.kind()) {
            let target = if want_start {
                let point = node.start_position();
                let line_start = node.start_byte() - point.column;
                let col = source
                    .get(line_start..node.start_byte())
                    .map(|prefix| prefix.chars().count())?;
                Some((node.start_byte(), (point.row, col)))
            } else {
                last_char_target(source, node.end_byte(), node.end_position())
            };
            if let Some(target) = target {
                targets.push(target);
            }
        }
        if walker.goto_first_child() {
            continue;
        }
        loop {
            if walker.goto_next_sibling() {
                break;
            }
            if !walker.goto_parent() {
                break 'walk;
            }
        }
    }

    targets.sort_unstable_by_key(|t| t.0);
    targets.dedup_by_key(|t| t.0);

    match boundary {
        MethodBoundary::NextStart | MethodBoundary::NextEnd => targets
            .iter()
            .filter(|t| t.0 > cursor_byte)
            .nth(count - 1)
            .map(|t| t.1),
        MethodBoundary::PrevStart | MethodBoundary::PrevEnd => targets
            .iter()
            .rev()
            .filter(|t| t.0 < cursor_byte)
            .nth(count - 1)
            .map(|t| t.1),
    }
}

/// `]M`/`[M` land ON the last character of the function (the `}` in braced
/// languages, the `d` of Ruby's `end`). `end_byte` is exclusive, so step
/// back one char; `end_point.column` counts bytes on the final row, which
/// gives us the row's start without scanning the file.
fn last_char_target(
    source: &str,
    end_byte: usize,
    end_point: tree_sitter::Point,
) -> Option<(usize, (usize, usize))> {
    if end_point.column == 0 {
        // Node ends exactly at a line boundary — its last char is a newline,
        // which is not a cursor position. No real grammar produces this for
        // function nodes; skip rather than guess.
        return None;
    }
    let end_byte = end_byte.min(source.len());
    let (last_byte, _) = source.get(..end_byte)?.char_indices().next_back()?;
    let line_start = end_byte - end_point.column;
    let col = source.get(line_start..last_byte)?.chars().count();
    Some((last_byte, (end_point.row, col)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::SyntaxManager;
    use std::path::Path;

    fn boundary(
        file_name: &str,
        source: &str,
        cursor_byte: usize,
        boundary: MethodBoundary,
        count: usize,
    ) -> Option<(usize, usize)> {
        let mut syntax = SyntaxManager::new();
        syntax.set_language_from_path(Path::new(file_name));
        syntax.parse_string(source);
        let language = syntax.language_name().expect("language").to_string();
        let (tree, cached) = syntax.get_tree_and_source().expect("tree");
        find_method_boundary(tree, cached, &language, cursor_byte, boundary, count)
    }

    const RUST_SRC: &str = "\
struct S;

impl S {
    fn method(&self) {
        let f = || 1;
    }
}

fn free() {
    if true {
    }
}
";

    #[test]
    fn rust_next_start_finds_impl_method_and_free_fn() {
        // From the top: first `fn method`, then `fn free` — the closure and
        // the if-block braces are not stops.
        let first = boundary("t.rs", RUST_SRC, 0, MethodBoundary::NextStart, 1).unwrap();
        assert_eq!(first, (3, 4));
        let second = boundary("t.rs", RUST_SRC, 0, MethodBoundary::NextStart, 2).unwrap();
        assert_eq!(second, (8, 0));
    }

    #[test]
    fn rust_prev_start_from_inside_free_fn() {
        // Cursor inside `if true {` (line 9) — [m goes to `fn free`, 2[m to
        // `fn method`.
        let cursor = RUST_SRC.find("if true").unwrap();
        assert_eq!(
            boundary("t.rs", RUST_SRC, cursor, MethodBoundary::PrevStart, 1),
            Some((8, 0))
        );
        assert_eq!(
            boundary("t.rs", RUST_SRC, cursor, MethodBoundary::PrevStart, 2),
            Some((3, 4))
        );
    }

    #[test]
    fn rust_next_end_lands_on_closing_brace() {
        // From the top, ]M lands on `fn method`'s closing `}` (line 5 col 4).
        assert_eq!(
            boundary("t.rs", RUST_SRC, 0, MethodBoundary::NextEnd, 1),
            Some((5, 4))
        );
        // From inside `free`, ]M lands on its own `}` (line 11 col 0).
        let cursor = RUST_SRC.find("if true").unwrap();
        assert_eq!(
            boundary("t.rs", RUST_SRC, cursor, MethodBoundary::NextEnd, 1),
            Some((11, 0))
        );
    }

    #[test]
    fn rust_prev_end_finds_method_close() {
        let cursor = RUST_SRC.find("if true").unwrap();
        assert_eq!(
            boundary("t.rs", RUST_SRC, cursor, MethodBoundary::PrevEnd, 1),
            Some((5, 4))
        );
    }

    #[test]
    fn rust_fails_when_no_more_boundaries() {
        // 9]m: fewer than nine functions — the whole motion fails, as in Vim.
        assert_eq!(
            boundary("t.rs", RUST_SRC, 0, MethodBoundary::NextStart, 9),
            None
        );
        // [m from the top of the file has nothing behind it.
        assert_eq!(
            boundary("t.rs", RUST_SRC, 0, MethodBoundary::PrevStart, 1),
            None
        );
    }

    #[test]
    fn typescript_counts_methods_and_arrows() {
        let src = "\
class C {
    method() {}
}

const f = (x: number) => x + 1;

function g() {}
";
        assert_eq!(
            boundary("t.ts", src, 0, MethodBoundary::NextStart, 1),
            Some((1, 4))
        );
        // Arrow functions are targets, matching @function.outer.
        let arrow_col = "const f = ".chars().count();
        assert_eq!(
            boundary("t.ts", src, 0, MethodBoundary::NextStart, 2),
            Some((4, arrow_col))
        );
        assert_eq!(
            boundary("t.ts", src, 0, MethodBoundary::NextStart, 3),
            Some((6, 0))
        );
    }

    #[test]
    fn go_counts_funcs_methods_and_literals() {
        let src = "\
func a() {}

func (s S) b() {
    go func() {
    }()
}
";
        // Cursor sits ON `func a`'s start, so ]m goes to the NEXT start,
        // matching Vim's strictly-forward rule.
        assert_eq!(
            boundary("t.go", src, 0, MethodBoundary::NextStart, 1),
            Some((2, 0))
        );
        // func literal inside b is a target too.
        assert_eq!(
            boundary("t.go", src, 0, MethodBoundary::NextStart, 2),
            Some((3, 7))
        );
        // `func a` is itself a target, reachable backward.
        let cursor = src.find("func (s").unwrap();
        assert_eq!(
            boundary("t.go", src, cursor, MethodBoundary::PrevStart, 1),
            Some((0, 0))
        );
    }

    #[test]
    fn python_defs_including_multibyte_lines() {
        // The multibyte char before `def` exercises byte→char col conversion.
        let src = "\
x = 1

def a():
    pass

class C:
    def b(self):  # π comment
        pass
";
        assert_eq!(
            boundary("t.py", src, 0, MethodBoundary::NextStart, 1),
            Some((2, 0))
        );
        assert_eq!(
            boundary("t.py", src, 0, MethodBoundary::NextStart, 2),
            Some((6, 4))
        );
        // Python defs end at the last statement char, not a brace.
        let cursor = src.find("class").unwrap();
        assert_eq!(
            boundary("t.py", src, cursor, MethodBoundary::PrevEnd, 1),
            Some((3, 7))
        );
    }

    #[test]
    fn ruby_methods_end_on_end_keyword() {
        let src = "\
def a
  1
end

class C
  def self.b
  end
end
";
        // Cursor is ON `def a`, so the next start is `def self.b`.
        assert_eq!(
            boundary("t.rb", src, 0, MethodBoundary::NextStart, 1),
            Some((5, 2))
        );
        // ]M lands on the `d` of `end`.
        assert_eq!(
            boundary("t.rb", src, 0, MethodBoundary::NextEnd, 1),
            Some((2, 2))
        );
    }

    #[test]
    fn bash_and_php_functions_are_targets() {
        let sh = "\
greet() {
    echo hi
}
";
        assert_eq!(
            boundary("t.sh", sh, 0, MethodBoundary::NextEnd, 1),
            Some((2, 0))
        );

        let php = "\
<?php
function a() {}
class C {
    function b() {}
}
";
        assert_eq!(
            boundary("t.php", php, 0, MethodBoundary::NextStart, 1),
            Some((1, 0))
        );
        assert_eq!(
            boundary("t.php", php, 0, MethodBoundary::NextStart, 2),
            Some((3, 4))
        );
    }

    #[test]
    fn unsupported_language_yields_no_targets() {
        assert_eq!(
            boundary("t.json", "{\"a\": 1}\n", 0, MethodBoundary::NextStart, 1),
            None
        );
    }
}
