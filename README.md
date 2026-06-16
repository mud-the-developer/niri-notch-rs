# niri-notch-rs

Animated Dynamic Island-style companion notch for the [Niri](https://github.com/YaLTeR/niri) Wayland compositor.

This is packaged as a **Niri companion daemon**, not an in-process compositor plugin. It runs a GTK3 + `gtk-layer-shell` overlay surface and integrates with Niri, Waybar, mako, and playerctl.

## Current features

- Top-center animated layer-shell notch
- Compact and expanded states with damped spring sizing
- Niri workspace and focused-window context
- mako notification interception:
  - auto-open on new notification
  - cache notification text/chip
  - dismiss duplicate mako card after a short delay
- playerctl media fallback
- Mouse interactions:
  - hover opens
  - left click clears notification or toggles media
  - right click opens the user's Niri control center
  - wheel up/down switches Niri workspaces
- Waybar companion styling pattern: Waybar as left/right secondary status rails, notch as primary center surface
- Testable state machine architecture:
  - `src/core.rs`: pure reducer, no I/O
  - `src/main.rs`: GTK UI, snapshot collection, effect runner
  - `tests/`: reducer and CLI parser tests

## CLI

```bash
niri-notch-rs doctor
niri-notch-rs install --user --dry-run
niri-notch-rs uninstall --user --dry-run
niri-notch-rs preview
```

`doctor` checks the basic runtime environment:

- `niri`
- `gtk-layer-shell-0` via pkg-config
- `waybar`
- `makoctl`
- `playerctl`
- Niri config validity
- Niri layer query availability

## Build

System dependencies on Arch/CachyOS-like systems:

```bash
sudo pacman -S --needed rust gtk3 gtk-layer-shell pkgconf waybar mako playerctl
```

Build and install locally:

```bash
cargo build --release
install -Dm755 target/release/niri-notch-rs ~/.local/bin/niri-notch-rs
~/.local/bin/niri-notch-rs doctor
```

## Niri integration

Add the daemon to `~/.config/niri/config.kdl` after Waybar:

```kdl
spawn-sh-at-startup "waybar"
spawn-sh-at-startup "~/.local/bin/niri-notch-rs"
```

Recommended Waybar setup:

```json
"modules-center": []
```

Then style Waybar as companion rails rather than another strong center island.

## Quality gates

Run before shipping:

```bash
cargo fmt -- --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo build --release
```

## Architecture

```text
Event / snapshot / pointer input
  -> AppEvent
  -> reduce(AppState, AppEvent) -> (AppState, Vec<Effect>)
  -> render AppState
  -> run Effect through a narrow effect runner
```

The reducer is intentionally pure so behavior can be tested without GTK, Niri, mako, or playerctl.

## Status

Early daily-driver prototype. The local setup has been verified on CachyOS + Niri + Waybar at 1920x1080@180Hz, but public packaging/install automation is still in progress.
