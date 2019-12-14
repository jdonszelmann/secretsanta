
cargo build --release
mkdir -p bin
cp ./target/release/secretsanta bin/santa

zip -r santa.zip bin/