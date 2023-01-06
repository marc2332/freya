for example in examples/*.rs
do
    cargo run --example "$(basename "${example%.rs}")" --features "linux" &
done