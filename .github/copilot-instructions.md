# Copilot Instructions

## Repository context
- This repository is used for **minimal repros**; start by checking out the relevant branch for the issue you are investigating (`README.md`).
- Never make repro code changes on `main`.
- For each new repro, always create and/or switch to an appropriately named branch before making any changes.

## Build, test, and lint commands
- Build: `cargo build`
- Run all tests: `cargo test`
- Run a single test: `cargo test tests::it_works` (or `cargo test it_works`)
- Lint: `cargo clippy --all-targets`

## High-level architecture
- Single Rust crate (`Cargo.toml`) with one library target in `src/lib.rs`.
- Core behavior is currently implemented as small, focused library functions (for example `add`).
- Unit tests are colocated in the same file under `#[cfg(test)]` modules, so verification is close to the repro code.
- There is no multi-layer app structure here; each branch is expected to contain a compact, issue-focused repro.

## Key conventions
- Keep changes tightly scoped to the repro scenario for the current branch.
- Prefer small, direct code in `src/lib.rs` with colocated unit tests unless the branch-specific repro requires a different layout.
- On each repro branch, replace the `README.md` with a branch-specific description of the repro: what it demonstrates, how to run it, and any relevant links.
