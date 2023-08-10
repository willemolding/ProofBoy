serve:
    cd crates/proofboy-recorder && trunk serve --open

build_web:
    cd crates/proofboy-recorder && wasm-pack build --release --target web

check:
    cd crates && cargo check --workspace --exclude proofboy-verifier-cannon

build_verifier:
	cd crates && docker run --rm -v `pwd`:/code -w="/code" ghcr.io/badboilabs/cannon-rs/builder:main cargo build -p proofboy-verifier-cannon --release -Zbuild-std 
