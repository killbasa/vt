build:
	cargo build --locked

debug *args:
	cargo run -- {{args}}

release:
	cargo build --locked --release

install: release
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

build-tui:
	cargo build --locked -p vt_tui

debug-tui *args:
	cargo run -p vt_tui -- {{args}}

ci-tui:
	cargo check -p vt_tui
	cargo test -p vt_tui
	cargo fmt -p vt_tui -- --check
	cargo clippy -p vt_tui --all-targets -- --deny warnings
	@echo "✅ All checks passed"
