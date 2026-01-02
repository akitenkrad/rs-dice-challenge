# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

```bash
# Build
cargo build
cargo build --release

# Run
cargo run -- -n 3 -i 0.5    # 3 dice, 0.5s interval
cargo run -- --resume       # Resume from saved state
./target/release/dice-challenge -n 2 -i 1.0

# Test
cargo test                   # Run all tests
cargo test -p shared         # Test shared crate only
cargo test test_is_all_same  # Run specific test

# Check/Lint
cargo check
cargo clippy
cargo fmt
```

## Architecture

This is a Rust workspace with two crates:

- **dice-challenge** (root) - CLI binary using clap for argument parsing and crossterm for terminal control
- **shared** - Library crate containing dice logic and error types

### Key Modules

- `src/bin/app.rs` - Main entry point, CLI args, display loop with terminal overwriting, Ctrl+C handling
- `shared/src/dice.rs` - Dice rolling, probability calculations (geometric distribution), Unicode display
- `shared/src/errors.rs` - `AppError` type with thiserror
- `shared/src/state.rs` - `GameState` for persistence, save/load to `.dice-challenge-state.json`

### Display Behavior

The CLI overwrites output in-place using crossterm cursor control. Each trial shows dice faces (Unicode ⚀-⚅), trial count, elapsed time, and probability of first match at that trial.

## Rust Edition

Uses Rust 2024 edition (workspace-level).
