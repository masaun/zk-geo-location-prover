echo "Read the environment variables..."
. ./.env # load the environment variables from the .env file for deployment

echo "Compile the contracts to be tested..."
forge build

echo "Running the test of the GeoLocationProofVerifier on BASE Sepolia testnet..."
forge test --optimize --optimizer-runs 5000 --evm-version cancun --match-contract GeoLocationProofVerifierTest --rpc-url ${BASE_TESTNET_RPC:?} -vvv