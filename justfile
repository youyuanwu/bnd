default: generate

generate: generate-openssl

generate-linux:
    cargo run -p bnd-linux-gen

generate-openssl: generate-linux
    cargo run -p bnd-openssl-gen

check:
    cargo check

fmt-check:
    cargo fmt --all -- --check

clippy:
    cargo clippy --all-targets -- -D warnings

build:
    cargo build --all-targets

test:
    cargo test --all -- --nocapture

ci: check fmt-check clippy build test
    git diff --exit-code
    git diff --cached --exit-code
