# ZK Geo Location Prover

## Technical Stack


<br>

## Overview

The problem to be solved by this project is the provacy-preserving related problem that an exact geo location of a DePIN Device (i.e. Helium, XNET) can be shared.

This ZK Geo Location Prover protocol can prove and verify whether or not a DePIN Device is working in legal region without revealing their exact locations (GPS coordinates) - thanks to ZKP (Zero-Knowledge Proof) produced via Boundless-powered by RISC Zero's zkVM.

In other words, this protocol can attest a "proof of location" without revealing a DePIN Device's exact locations.



<br>

### Run the test of the guest program
- Run the ZK guest program (`main()` in the `./guests/tests/runningGuestProgram_location-prover-test.sh`):
```bash
sh ./guests/tests/runningGuestProgramTest_geo-location-prover-test.sh
```

<br>

### Run the (backend) App
- Run the `./apps/src/main.rs`:
```bash
sh ./apps/runningApp_main.sh
```

<br>

### Run the test of the smart contract
- Run the test of the smart contract (`./contracts/test/GeoLocationProofVerifier.t.sol`):
```bash
sh ./contracts/test/runningTest_GeoLocationProofVerifier.sh
```


<br>

### Deploy the smart contracts
- 1/ Deploy the `xxx` contract on [`BASE`]() testnet:
```bash
sh ./contracts/scripts/runningScript_Deploy.sh
```

<br>

<hr>

# Boundless Foundry Template

This template serves as a starting point for developing an application with verifiable compute provided by [Boundless][boundless-homepage].
It is built around a simple smart contract, `GeoLocationProofVerifier`, and its associated RISC Zero guest, `is-even`.

## Build

To build the example run:

```bash
# Install RISC Zero toolchain if not already installed
curl -L https://risczero.com/install | bash
rzup install

# Populate the `./lib` submodule dependencies
git submodule update --init --recursive
cargo build
forge build
```

## Test

Test the Solidity smart contracts with:

```bash
forge test -vvv
```

Test the Rust code including the guest with:

```bash
cargo test
```

## Deploy to Testnet

### Set up your environment

Export your Sepolia testnet wallet private key as an environment variable:

```bash
export WALLET_PRIVATE_KEY="YOUR_WALLET_PRIVATE_KEY"
```

To allow provers to access your zkVM guest binary, it must be uploaded to a public URL. For this example we will upload to IPFS using Pinata. Pinata has a free tier with plenty of quota to get started. Sign up at [[Pinata](https://pinata.cloud/)](https://pinata.cloud/), generate an API key, and set the JWT as an environment variable:

```bash
export PINATA_JWT="YOUR_PINATA_JWT"
```

A [`.env`](./.env) file is provided with the Boundless contract deployment information for Sepolia.
The example app reads from this `.env` file automatically.

### Deploy the contract

To deploy the `GeoLocationProofVerifier` contract run:

```bash
. ./.env # load the environment variables from the .env file for deployment
forge script contracts/scripts/Deploy.s.sol --rpc-url ${RPC_URL:?} --broadcast -vv
```

Save the `GeoLocationProofVerifier` contract address to an env variable:

<!-- TODO: Update me -->

```bash
# First contract deployed and top of logs is GeoLocationProofVerifier
export GEO_LOCATION_PROOF_VERIFIER_ADDRESS=#COPY EVEN NUMBER ADDRESS FROM DEPLOY LOGS
```

> You can also use the following command to set the contract address if you have [`jq`][jq] installed:
>
> ```bash
> export GEO_LOCATION_PROOF_VERIFIER_ADDRESS=$(jq -re '.transactions[] | select(.contractName == "GeoLocationProofVerifier") | .contractAddress' ./broadcast/Deploy.s.sol/11155111/run-latest.json)
> ```

### Run the example

The [example app](apps/src/main.rs) will upload your zkVM guest to IPFS, submit a request to the market for a proof that "4" is an even number, wait for the request to be fulfilled, and then submit that proof to the GeoLocationProofVerifier contract, setting the value to "4".


To run the example:

```bash
RUST_LOG=info cargo run --bin app -- --even-number-address ${GEO_LOCATION_PROOF_VERIFIER_ADDRESS:?} --number 4
```

[jq]: https://jqlang.github.io/jq/
[boundless-homepage]: https://beboundless.xyz
[sepolia]: https://ethereum.org/en/developers/docs/networks/#sepolia
