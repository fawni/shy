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

# Runs exessive clippy lints (possible false positives so just warn)
lint:
    cargo clippy --locked -- -W clippy::pedantic -W clippy::nursery
