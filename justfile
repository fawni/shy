set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
    @just --list

# Build
build:
    cargo build --locked --release

# Clean
clean:
    cargo clean --locked

# Runs clippy
check:
    cargo clippy --locked -- -D warnings