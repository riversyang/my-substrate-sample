#
# cd node
# cargo build --release --features runtime-benchmarks
# cd ..
#
./target/release/node-template benchmark --chain=dev --execution=wasm --wasm-execution=compiled --pallet=$1 --extrinsic=$2 --steps=1 --repeat=50000 --output=./pallets/$3/src/weights.rs
