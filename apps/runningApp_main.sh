echo "Read the environment variables"
. ./.env # load the environment variables from the .env file for deployment

echo "Update the guest program"
cargo build

echo "Running the app (./apps/src/main.rs) with the following environment variables:"
RUST_LOG=info cargo run --bin app -- --even-number-address ${GEO_LOCATION_PROOF_VERIFIER_ADDRESS:?} --number 4
