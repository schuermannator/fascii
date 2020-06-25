target/release/ascii:
	cargo build --release

target/release/std:
	cargo build --release

bench: target/release/ascii target/release/std
	hyperfine --warmup 3 'target/release/ascii' 'target/release/std'

.PHONY: bench
