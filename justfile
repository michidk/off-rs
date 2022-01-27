set windows-powershell

build-dev:
	cargo build

build-release:
	cargo build --release

build: build-release

check:
	cargo check --all-targets

test:
	cargo test --all-targets

clippy:
	cargo clippy --all-targets -- -Dwarnings

checkfmt:
	cargo fmt --all -- --check

lint: checkfmt clippy

clean:
	cargo clean

doc:
	cargo doc --all --document-private-items

# utility
# can i commit
cic: test lint doc
