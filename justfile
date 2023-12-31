set dotenv-load
set positional-arguments

default:
  @just --list

build_web:
    cd crates/proofboy-recorder && wasm-pack build --target web

check:
    cd crates && cargo check --workspace --exclude proofboy-verifier-cannon

build_verifier:
	cd crates && docker run --rm -v `pwd`:/code -v `pwd`/../roms:/roms -w="/code" ghcr.io/badboilabs/cannon-rs/builder:main cargo build -p proofboy-verifier-cannon --release -Zbuild-std 

patch_elf:
    cannon load-elf --path ./crates/target/mips-unknown-none/release/proofboy-verifier-cannon --patch stack --out=verifier-initial-state.json

emulate txhash:
    TXN_HASH={{txhash}} SERVE_PREIMAGES=true cannon run \
        --input ./verifier-initial-state.json \
        --info-at %1000000 --stop-at never -- \
        cargo run --manifest-path ./crates/proofboy-verifier-native/Cargo.toml

verify txhash:
    TXN_HASH={{txhash}} cargo run --release --quiet --manifest-path ./crates/proofboy-verifier-native/Cargo.toml

recorder:
    cargo run --manifest-path ./crates/proofboy-recorder/Cargo.toml -- -o journal.bin -m nft_metadata.json