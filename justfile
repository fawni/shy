set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
    @just --list

# Install to path
install:
    cargo install --path .

# Uninstall
uninstall:
    cargo uninstall shy

# Build
build:
    cargo build --locked --release

# Clean
clean:
    cargo clean

# Runs clippy
check:
    cargo clippy --locked -- -D warnings
