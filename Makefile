build:
	cargo build

run:
	cargo run

build-release:
	cargo build --release

run-release:
	cargo run --release

fix:
	cargo clippy --fix --allow-dirty

check:
	cargo clippy
