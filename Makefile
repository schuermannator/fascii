target/release/ascii:
	cargo build --release

target/release/std:
	cargo build --release

bench: target/release/ascii target/release/std
	hyperfine --warmup 3 'target/release/ascii' 'target/release/std'

test:
	cargo t --verbose

check:
	cargo check
	cargo fmt -- --check
	cargo clippy 

.PHONY: bench check test

