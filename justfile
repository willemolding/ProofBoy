serve:
    cd proofboy-recorder && trunk serve --open

build_web:
    cd proofboy-recorder && wasm-pack build --release --target web

check:
    cargo check --workspace --all