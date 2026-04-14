toolchain := `sed -nr 's/channel = "(.*)"/\1/p' rust-toolchain.toml`
nightly_toolchain := `sed -nr 's/channel = "(.*)"/\1/p' rust-toolchain-nightly.toml`

rv:
    @echo '{{toolchain}}'

rv-nightly:
    @echo {{nightly_toolchain}}

f:
    taplo fmt
    RUSTUP_TOOLCHAIN={{nightly_toolchain}} cargo fmt --all -- --error-on-unformatted --unstable-features

f-check:
    taplo fmt --check
    RUSTUP_TOOLCHAIN={{nightly_toolchain}} cargo fmt --all --check -- --error-on-unformatted --unstable-features

f-nix:
    alejandra flake.nix

c:
    taplo check
    cargo clippy --workspace --examples --features "all-debug" -- -D warnings
    cargo doc --no-deps --workspace --features "all-debug"

c-ci:
    taplo check
    cargo clippy --workspace --examples --features "all-debug" -- -D warnings
    RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --workspace --features "all-debug"

e example:
    cargo run --example {{example}}

t:
    cargo test --doc --workspace
    cargo nextest run --workspace --exclude examples --features all-tests

t-layout:
    cargo nextest run --package torin

d:
    RUSTDOCFLAGS="--cfg docsrs" RUSTUP_TOOLCHAIN={{nightly_toolchain}} cargo doc --no-deps --workspace --features "all, docs" --open

tc:
    cargo nextest run --workspace --exclude examples --features all-tests

t-components:
    cargo nextest run --package freya-components --features freya/gif,freya/markdown

t-core:
   cargo nextest run --package freya-core --package ragnarok

pe:
    cargo run --example dev_perf --release

ps:
    cargo run --example dev_perf --features "hotpath" --release

pa:
    cargo run --example dev_perf --features "hotpath, hotpath-alloc" --release

ps-ci:
    cargo run --example dev_perf --features "hotpath" --release

pa-ci:
    cargo run --example dev_perf --features "hotpath, hotpath-alloc" --release

ba:
    cargo build --all-targets --workspace -F freya/all-debug

bindings:
    cargo build --package freya --package freya-testing --features "mocked-engine, all-bindings" --no-default-features

dev-app:
    cargo run --package freya-devtools-app

run-examples:
    for ex_file in examples/*.rs; do \
        ex=$(basename "$ex_file" .rs); \
        echo "running $ex"; \
        cargo run --example "$ex"; \
        echo; \
    done
