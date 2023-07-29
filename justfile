serve:
    cd crates/proofboy-recorder && trunk serve --open

build_web:
    cd crates/proofboy-recorder && wasm-pack build --release --target web

check:
    cd crates && cargo check --workspace --all