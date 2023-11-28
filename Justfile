_default:
    @just --list

# Run a toolkit command
tk *ARGS:
    cargo run -p aoc-sx-cli {{ ARGS }}

# Run AoC tests
test YEAR:
    cargo test -p aoc-{{ YEAR }}

# Run all AoC tests
test-all:
    cargo test
