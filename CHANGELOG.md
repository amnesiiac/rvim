# Changelog

## 0.3.0 - UNRELEASED

Nevi 0.3.0 brings further improvements.

### Highlights

- New segmented statusline: mode-colored powerline segments, git branch and diff stats, diagnostic counts, and an event-driven LSP activity indicator. Set `[ui] basic = true` in config.toml for a plain-ASCII look with the same layout (automatic if you had `use_nerd_font_icons = false`).
- `:checkhealth` now reports the active UI mode, a Nerd Font glyph probe, and the raw LSP status string (which no longer renders in the statusline).
- Statusline width math is now Unicode-aware, fixing right-side misalignment with double-width (e.g. CJK) filenames.
- Themes gain an optional `[ui.statusline] section_bg` key for the mid statusline segments; it falls back to `cursor_line`, so existing themes need no changes.
- `:Format` now runs the external formatter from `languages.toml` when one is configured for the current language, and falls back to LSP otherwise.
- Added Vim motions `g_`, `|`, `gM`, `gm`, `go`, `[[`, `]]`, `][`, `[]`, `[{`, `]}`, `[(`, and `])`.
- `j`/`k` now keep the preferred column (Vim `curswant`) when moving across short or blank lines, and `$` makes vertical motion stick to line ends.
- Added Bash/shell tree-sitter highlighting for `.sh`/`.bash`/`.zsh` files, common rc/profile names (`.bashrc`, `.bash_profile`, `.zshrc`, `PKGBUILD`, …), and shebang detection for extensionless scripts.
- Added `[lsp.servers.shell]` with `bash-language-server` (same config shape as Go/Ruby).
- Fixed `J`/`gJ` on the last line silently deleting the file's trailing newline; they are now a no-op like Vim.
- Files without a final newline now gain one on load and save (Neovim's `fixendofline` default), fixing the cursor landing on a phantom line when opening a line at end of file.

## 0.2.0 - 2026-07-07

Nevi 0.2.0 is a feature and performance release focused on making the editor
feel faster, safer, and easier to adopt.

### Highlights

- Added damage-aware partial rendering for common cursor movement and edit paths.
- Improved long-line and large-file responsiveness, including clearer large-file mode visibility.
- Added render regression coverage and a frame budget guard to catch future UI regressions earlier.
- Added an in-memory `:FlightRecorder` / `:WhySlow` performance report for debugging latency.
- Added Vim oracle parity coverage and macOS/Linux CI validation.
- Added labeled jump navigation with `:Jump` and `<Space>j`.
- Added Swiss-army CLI modes: `nevi view`, `nevi diff`, and `nevi pick`.
- Added previewed project-wide replace with an explicit apply step.
- Added `:ToolInstall`, `:ConfigDefaults`, and expanded `:checkhealth` reporting.
- Added Go and Ruby language support.
- Added more Vim/Neovim-compatible keybindings, including window movement/resizing, visual block insert/append, `ZZ`, and normal-mode Enter motion.
- Improved Homebrew, Linux/source install, and update documentation.

### Performance

- Partially repaint only affected editor rows for many normal and insert-mode operations.
- Limit search highlights and labeled-jump scans to visible rows.
- Optimize long-line rendering for minified and very wide files.
- Throttle LSP status redraws and hide benign LSP request errors.
- Add input event coalescing coverage to guard responsiveness.

### Safety And Diagnostics

- Guard saves against overwriting files changed externally on disk.
- Open health, config defaults, and generated reports in read-only buffers.
- Add keymap health checks and external tool checks.
- Add project replace safeguards for preview/apply workflows.

### Install And Upgrade

Homebrew users can upgrade after this release with:

```bash
brew update
brew upgrade nevi
```

If installed with the fully qualified formula name:

```bash
brew upgrade anthonyamaro15/nevi/nevi
```

Verify the installed version:

```bash
nevi --version
```

## 0.1.0 - Initial Release

- Initial public release of Nevi.
