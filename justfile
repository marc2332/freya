f:
    taplo fmt
    cargo +nightly fmt --all -- --error-on-unformatted --unstable-features

c:
    taplo check
    cargo clippy --workspace --examples --features "all-debug" -- -D warnings
    cargo doc --workspace --features "all-debug"

e example:
    cargo run --example {{example}}

t:
    cargo test --doc --workspace
    cargo nextest run --workspace --exclude examples

d:
    cargo doc --workspace --features "all, docs" --open

tc:
    cargo nextest run --workspace --exclude examples

pe:
    cargo run --example perf --release

ps:
    cargo run --example perf --features "hotpath" --release

pa:
    cargo run --example perf --features "hotpath, hotpath-alloc-bytes-total" --release

ps-ci:
    cargo run --example perf --features "hotpath, hotpath-ci" --release

pa-ci:
    cargo run --example perf --features "hotpath, hotpath-ci, hotpath-alloc-bytes-total" --release

ba:
    cargo build --all-targets --all-features

bindings:
    cargo build --package freya --package freya-testing --features "mocked-engine, all-debug" --no-default-features

dev-app:
    cargo run --package freya-devtools-app

setup:
    cargo install cargo-binstall
    cargo binstall taplo-cli
    cargo install cargo-nextest
    git submodule update --recursive --remote

set shell := ["bash", "-cu"]

run-examples:
    for ex_file in examples/*.rs; do \
        ex=$(basename "$ex_file" .rs); \
        echo "🚀 Running example: $ex"; \
        cargo run --example "$ex"; \
        echo; \
    done
