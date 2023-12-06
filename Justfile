_default:
    @just --list

# Run a toolkit command
tk *ARGS:
    cargo run --release -p aoc-sx-cli {{ ARGS }}

# Run AoC tests
test-year YEAR:
    cargo test --release -p aoc-{{ YEAR }} -- -Zunstable-options --report-time

# Run AoC test day
test-day YEAR DAY:
    cargo test --release -p aoc-{{ YEAR }} day{{ DAY }} -- -Zunstable-options --report-time

# Run all AoC tests
test-all:
    cargo test --release -- -Zunstable-options --report-time

# Format code
fmt:
    cargo fmt --all

# Run lints
lint:
    cargo clippy --all
