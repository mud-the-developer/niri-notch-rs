# Contributing

Thanks for helping improve `niri-notch-rs`.

## Local setup

Arch/CachyOS dependencies:

```bash
sudo pacman -S --needed rust gtk3 gtk-layer-shell pkgconf waybar mako playerctl
```

Ubuntu CI-equivalent dependencies:

```bash
sudo apt-get install -y libgtk-3-dev libgtk-layer-shell-dev pkg-config
```

## Quality gates

Run these before opening a PR:

```bash
cargo fmt -- --check
cargo test --locked
cargo clippy --locked --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps
cargo build --release --locked
```

## Architecture rules

- Keep behavior in pure modules where possible.
- `src/core.rs` owns reducer/state/effects.
- `src/install.rs` owns pure config transformations.
- `src/main.rs` is allowed to perform GTK, filesystem, and process side effects.
- New behavior needs a failing test first.
- Do not add ad-hoc shell mutations without backup/idempotency.

## Commit style

Use conventional commit style where practical:

```text
feat: add install planner
fix: preserve newline when uninstalling managed block
ci: add locked docs gate
```
