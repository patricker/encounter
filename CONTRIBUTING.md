# Contributing to encounter

Thanks for considering a contribution. encounter is a small, focused crate — patches are welcome whether they fix bugs, sharpen the docs, add a useful test, or extend the protocol surface.

## Dev environment

You need a recent Rust toolchain. The project pins `rust-version = "1.85"` (Edition 2024 floor), so `rustup install stable` is the simplest path.

```bash
git clone https://github.com/patricker/encounter.git
cd encounter
cargo build
cargo test
```

## Before opening a PR

CI runs the same checks locally. Please make sure all of these pass:

```bash
cargo build --all-targets
cargo test
cargo test --doc
cargo clippy --all-targets -- -D warnings
cargo fmt --all -- --check
cargo doc --no-deps
```

`cargo fmt` (without `--check`) will format in place.

## Tests

- **Unit + integration tests** live in `src/**/tests` and `tests/*.rs`. Run with `cargo test`.
- **Doctests** live in `///` blocks on public items. They run with `cargo test --doc` and also as part of the standard `cargo test`.
- **Examples** in `examples/` are compiled by `cargo build --all-targets` and runnable with `cargo run --example <name>`.

### `tests/real_catalog.rs`

This integration test exercises encounter against a real catalog from a sibling repository (`../societas/catalog`). If you don't have that sibling checked out, the test self-skips — no setup required, but the test will silently produce vacuous results. To exercise it for real, clone the `societas` repository as a sibling of `encounter`.

## Commits and PRs

- Conventional-commit style is preferred (`feat:`, `fix:`, `docs:`, `chore:`, etc.).
- Each PR should be self-contained and pass CI on its own.
- For changes to public API, update the `CHANGELOG.md` entry under "Unreleased".

## Reporting issues

Open an issue on [GitHub](https://github.com/patricker/encounter/issues) with:
- What you tried to do
- What you expected to happen
- What actually happened
- A minimal reproduction if you can spare the time

## License

By contributing, you agree that your contributions will be dual-licensed under MIT or Apache-2.0, matching the rest of the crate.
