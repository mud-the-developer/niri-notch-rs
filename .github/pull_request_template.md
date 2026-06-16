## Summary

-

## Test plan

- [ ] `cargo fmt -- --check`
- [ ] `cargo test --locked`
- [ ] `cargo clippy --locked --all-targets --all-features -- -D warnings`
- [ ] `RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps`
- [ ] `cargo build --release --locked`

## Screenshots / notes

