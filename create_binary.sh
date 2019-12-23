
RUSTFLAGS='-C link-arg=-s' cargo build --release
mkdir -p bin
cp ./target/release/secretsanta bin/santa

rm santa.zip
zip -r santa.zip bin/santa bin/README.md