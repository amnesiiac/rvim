---
status: accepted
---

# Mouse capture is on by default

Nevi aims to feel like nvim, and nvim has shipped `mouse=nvi` since 0.8: the wheel scrolls the buffer, not the terminal scrollback (issue #274). To match that, nevi asks the terminal for mouse events whenever it runs. That takes over clicks and drags too, so selecting text with the terminal itself needs the terminal's bypass key while nevi is open. That key is Option in iTerm2 and Shift in most other terminals.

We chose nvim parity over terminal-native selection. Capture is on by default, `mouse = false` under `[editor]` in config.toml turns it off for good, and `:set nomouse` or `:set mouse=` turns it off for the current session. The `:set mouse=` form matters because it is the reflex vim users already have for "let me copy something real quick".

## Considered Options

- Off by default, opt-in. Rejected because it leaves issue #274 unfixed for anyone who never reads release notes, and it breaks with nvim's default, which is the whole point of the project.
- Always on with no way to turn it off. Rejected because losing native selection with no escape hatch punishes people who copy text out of their terminal all day.

## What this changes going forward

- `mouse` became the first option that `:set` actually applies. The way it parses (an empty value means off, any flags like `a` or `nvi` mean on) is the precedent for future runtime options.
- Wheel and click behavior is pinned to nvim's: 3 lines per wheel tick, scrolling targets the pane under the pointer, and the cursor only moves when it would fall off screen.
- Anyone who prefers the old behavior sets `mouse = false` once and never has to think about it again.
