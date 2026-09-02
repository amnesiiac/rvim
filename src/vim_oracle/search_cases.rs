use super::OracleCase;

/// Search-family cases: `/`, `?`, `n`, `N`, `*`, `#`, `gn`, `gN`, and the
/// search prompt editing keys, verified against real Neovim. Both editors
/// default to case-sensitive search with wrapscan on, so plain-text patterns
/// compare directly. Nevi's search is a literal substring match (no regex),
/// so every pattern here is literal on both sides.
pub(super) const SEARCH_CASES: &[OracleCase] = &[
    // Forward search basics.
    OracleCase {
        name: "search forward lands on match start",
        initial_text: "alpha\nsome beta here\ngamma\n",
        keys: "/beta<CR>",
    },
    OracleCase {
        name: "search forward skips match at cursor",
        initial_text: "beta x beta\n",
        keys: "/beta<CR>",
    },
    OracleCase {
        name: "search forward wraps to top",
        initial_text: "beta\nalpha\ngamma\n",
        keys: "G/beta<CR>",
    },
    OracleCase {
        name: "search not found leaves cursor",
        initial_text: "alpha\nbeta\n",
        keys: "/zzz<CR>",
    },
    // Incremental typing moves the cursor while a prefix matches; a failed
    // final pattern must land back on the original position like nvim.
    OracleCase {
        name: "search not found after partial match restores cursor",
        initial_text: "alpha\nbetx\n",
        keys: "/betz<CR>",
    },
    OracleCase {
        name: "search is case sensitive",
        initial_text: "Beta\nbeta\n",
        keys: "/beta<CR>",
    },
    OracleCase {
        name: "empty search repeats last pattern",
        initial_text: "alpha\nbeta\nbeta tail\n",
        keys: "/beta<CR>gg/<CR>",
    },
    // Backward search.
    OracleCase {
        name: "search backward lands on previous match",
        initial_text: "beta\nalpha\nbeta\ngamma\n",
        keys: "G?beta<CR>",
    },
    OracleCase {
        name: "search backward wraps to bottom",
        initial_text: "alpha\nbeta\n",
        keys: "?beta<CR>",
    },
    OracleCase {
        name: "search backward within line",
        initial_text: "beta beta x\n",
        keys: "$?beta<CR>",
    },
    // Empty ? repeats the last pattern backward AND flips the stored
    // direction, so a following n keeps going backward.
    OracleCase {
        name: "empty backward search flips direction for n",
        initial_text: "beta\nx\nbeta\nx\nbeta\n",
        keys: "/beta<CR>G?<CR>n",
    },
    // n / N repeat.
    OracleCase {
        name: "next match",
        initial_text: "beta\nbeta\nbeta\n",
        keys: "/beta<CR>n",
    },
    OracleCase {
        name: "next match wraps",
        initial_text: "beta\nbeta\nbeta\n",
        keys: "/beta<CR>nn",
    },
    OracleCase {
        name: "previous match reverses direction",
        initial_text: "beta\nbeta\nbeta\n",
        keys: "/beta<CR>nN",
    },
    OracleCase {
        name: "n after backward search continues backward",
        initial_text: "beta\nalpha\nbeta\nbeta\n",
        keys: "G?beta<CR>n",
    },
    OracleCase {
        name: "counted next match",
        initial_text: "beta\nbeta\nbeta\nbeta\n",
        keys: "/beta<CR>2n",
    },
    OracleCase {
        name: "counted previous match",
        initial_text: "beta\nbeta\nbeta\nbeta\n",
        keys: "/beta<CR>nn2N",
    },
    // Word-under-cursor search.
    OracleCase {
        name: "star searches word forward",
        initial_text: "beta alpha\ngamma\nbeta tail\n",
        keys: "*",
    },
    OracleCase {
        name: "star wraps to only occurrence",
        initial_text: "beta\nalpha\n",
        keys: "*",
    },
    OracleCase {
        name: "star from middle of word",
        initial_text: "beta alpha beta\n",
        keys: "ll*",
    },
    OracleCase {
        name: "star then n continues forward",
        initial_text: "beta\nalpha beta\nbeta\n",
        keys: "*n",
    },
    OracleCase {
        name: "hash searches word backward",
        initial_text: "beta\nalpha\nbeta gamma\n",
        keys: "G#",
    },
    OracleCase {
        name: "hash from middle of word",
        initial_text: "beta x\nbeta\n",
        keys: "jl#",
    },
    // gn / gN select the match in visual mode. gn leaves the cursor on the
    // match end, gN on the match start.
    OracleCase {
        name: "gn selects current match",
        initial_text: "alpha\nbeta\ngamma beta\n",
        keys: "/beta<CR>gn",
    },
    OracleCase {
        name: "gn selects next match from outside",
        initial_text: "alpha\nbeta\ngamma beta\n",
        keys: "/beta<CR>gggn",
    },
    OracleCase {
        name: "counted gn selects a later match",
        initial_text: "beta\nbeta\nbeta\n",
        keys: "/beta<CR>gg2gn",
    },
    OracleCase {
        name: "gN selects match backward",
        initial_text: "beta\nbeta\ngamma\n",
        keys: "/beta<CR>GgN",
    },
    // Search prompt editing (vim cmdline keys).
    OracleCase {
        name: "prompt backspace reanchors at origin",
        initial_text: "ac\nab\nac\n",
        keys: "/ab<BS>c<CR>",
    },
    OracleCase {
        name: "prompt ctrl-w deletes word",
        initial_text: "alpha\nbeta\n",
        keys: "/junk<C-w>beta<CR>",
    },
    OracleCase {
        name: "prompt ctrl-u clears input",
        initial_text: "alpha\nbeta\n",
        keys: "/alpha<C-u>beta<CR>",
    },
    OracleCase {
        name: "prompt ctrl-b inserts at start",
        initial_text: "alpha\nbeta\n",
        keys: "/eta<C-b>b<CR>",
    },
    OracleCase {
        name: "prompt ctrl-e returns to end",
        initial_text: "alpha\nbeta\n",
        keys: "/et<C-b>b<C-e>a<CR>",
    },
    OracleCase {
        name: "prompt history recall",
        initial_text: "alpha\nbeta\nbeta tail\n",
        keys: "/beta<CR>gg/<Up><CR>",
    },
    OracleCase {
        name: "prompt register insert",
        initial_text: "alpha\nbeta\nalpha end\n",
        keys: "yiwj/<C-r>0<CR>",
    },
    // Cancelling restores the position the search started from.
    OracleCase {
        name: "escape cancels and restores cursor",
        initial_text: "alpha\nbeta\n",
        keys: "/beta<Esc>",
    },
    OracleCase {
        name: "escape restores viewport after distant match",
        initial_text: super::SCREEN_POSITION_TEXT,
        keys: "50Gzz/line 090<Esc>",
    },
];

// Known divergence, deliberately not an active case (it would fail CI):
// vim's * searches with word boundaries (\<abc\>), Nevi searches the literal
// word text, so * on "abc" also matches inside "abcdef".
//   initial_text: "abc abcdef\nabc\n", keys: "*"
//   nvim lands on (1, 0); Nevi lands on (0, 4).
