rustup target add wasm32-unknown-unknown
cargo build --release --target=wasm32-unknown-unknown
cp ../target/wasm32-unknown-unknown/release/flappy_bird.wasm flappy_bird.wasm