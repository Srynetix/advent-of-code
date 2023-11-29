_default:
    @just --list

# Run a toolkit command
tk *ARGS:
    cargo run --release -p aoc-sx-cli {{ ARGS }}

# Run AoC tests
test YEAR:
    cargo test --release -p aoc-{{ YEAR }}

# Run all AoC tests
test-all:
    cargo test --release

# Format code
fmt:
    cargo fmt --all

# Run lints
lint:
    cargo clippy --all