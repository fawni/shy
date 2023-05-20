set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
    @just --list

# Install to path
install:
    cargo install --path .

# Uninstall
uninstall:
    cargo uninstall shy

# Exessive clippy lints
lint:
    cargo clippy --locked -- -W clippy::pedantic -W clippy::nursery
