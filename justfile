build:
	cargo build --locked

release:
	cargo build --locked --release

install: release
	cargo install --locked --path .

debug *args: build
	./target/debug/vt {{args}}

ci:
	cargo check --workspace
	cargo test --workspace
	cargo fmt --all -- --check
	cargo clippy --workspace --all-targets -- --deny warnings
	cargo shear
	@echo "✅ All checks passed"
