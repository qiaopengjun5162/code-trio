# code-trio/Makefile
all: build test run

build:
    cargo build --release --manifest-path packages/rust/Cargo.toml
    go build ./... --manifest-path packages/go
    pip install -r packages/python/requirements.txt

test:
    cargo test --manifest-path packages/rust/Cargo.toml
    go test ./... --manifest-path packages/go
    pytest packages/python/tests

run:
    python scripts/run_all.py
