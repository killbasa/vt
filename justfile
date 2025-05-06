debug *args:
	cargo run -- {{args}}

build:
	cargo build --locked -p vt -p vt-tui

release:
	cargo build --locked --release -p vt -p vt-tui

install:
	cargo install --locked --path .

ci: check test fmt clippy shear
	@echo "✅ All checks passed"

check:
	cargo check --workspace

test:
	cargo test --workspace

fmt:
	cargo fmt --all -- --check

clippy:
	cargo clippy --workspace --all-targets -- --deny warnings

shear:
	cargo shear

# TUI #

debug-tui *args:
	cargo run -p vt-tui -- {{args}}

build-tui:
	cargo build --locked -p vt-tui

release-tui:
	cargo build --locked --release -p vt-tui

ci-tui:
	cargo check -p vt-tui
	cargo test -p vt-tui
	cargo fmt -p vt-tui -- --check
	cargo clippy -p vt-tui --all-targets -- --deny warnings
	@echo "✅ All checks passed"
