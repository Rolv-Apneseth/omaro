alias c := check
alias f := format
alias t := test
alias b := build
alias d := develop
alias r := run
alias rc := run-config
alias rC := run-clean
alias g := gif
alias dg := develop-gif
alias p := publish

# COMMANDS -----------------------------------------------------------------------------------------

# List commands
default:
    @just --list

# Check
check:
    cargo check && cargo clippy --all -- -W clippy::all

# Format
format:
    cargo +nightly fmt --all

# Test
test: check format
    cargo test --all
    cargo msrv verify
    cargo deny check

# Build
build: test
    cargo build --release

# Re-run tests on any change
develop: format
    bacon test

# Run the TUI
run:
    cargo run

# Run the TUI with the default config
run-clean:
    cargo run -- --clean

# Run the TUI with the minimal config
run-config CONFIG:
    cargo run -- --config ./configs/{{ CONFIG }}.toml

# Generate the demo GIF
gif:
    vhs assets/demo.tape --output assets/demo.gif

# Re-generate the demo GIF whenever `demo.tape` is modified
develop-gif:
    echo assets/demo.tape | entr vhs /_ --output assets/demo.gif

# Publish
publish: test
    cargo publish
