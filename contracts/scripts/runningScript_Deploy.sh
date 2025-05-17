echo "Read the environment variables..."
. ./.env # load the environment variables from the .env file for deployment

echo "Compile the contracts to be deployed..."
forge build

# echo "Deploying the contract on Ethereum Sepolia testnet..."
# forge script contracts/scripts/Deploy.s.sol --rpc-url ${RPC_URL:?} --broadcast -vv

echo "Deploying the GeoLocationProofVerifier contract on Base Sepolia Testnet..."
forge script contracts/scripts/Deploy.s.sol \
    --broadcast \
    --rpc-url ${BASE_TESTNET_RPC} \
    --chain-id ${BASE_TESTNET_CHAIN_ID} \
    --private-key ${BASE_TESTNET_PRIVATE_KEY} \
    ./GeoLocationProofVerifier.sol:GeoLocationProofVerifier --skip-simulation --legacy

# [NOTE - Adding the "--legacy" option]: Due to this error - Error: Failed to estimate EIP1559 fees. This chain might not support EIP1559, try adding --legacy to your command.

echo "Verify the deployed-GeoLocationProofVerifier contract on Base Sepolia Testnet Explorer..."
forge script contracts/scripts/Deploy.s.sol \
    --rpc-url ${BASE_TESTNET_RPC} \
    --chain-id ${BASE_TESTNET_CHAIN_ID} \
    --private-key ${BASE_TESTNET_PRIVATE_KEY} \
    --resume \
    --verify \
    --verifier etherscan \
    --verifier-url https://api-sepolia.basescan.org/api \
    --etherscan-api-key ${BASESCAN_API_KEY} \