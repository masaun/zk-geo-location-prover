# ZK Geo Location Prover

## Tech stack

- ZK program (`ZK circuit`): Implemented in [`Boundless`](https://beboundless.xyz/) powered by [RISC Zero](https://risczero.com/)'s zkVM (Written in Rust)
- Smart Contract: Written in Solidity (Framework: Foundry)
- Blockchain: [`Base Sepolia`](https://sepolia.basescan.org/) testnet


<br>

## Overview

This is the ZK Geo Location Prover protocol, which solve the provacy-preserving related problem that an exact geo location of a DePIN device can be shared (revealed) among DePIN network (i.e. Helium, XNET).

The ZK Geo Location Prover protocol can prove/generate a **`ZK Proof (Zero-Knowledge Proof) of location` of DePIN device** and verify whether or not a DePIN device is working in legal region without revealing their exact locations (GPS coordinates) - thanks to thanks to [`Boundless`](https://beboundless.xyz/) powered by [RISC Zero](https://risczero.com/)'s zkVM.

The ZK Geo Location Prover protocol is consist of the two parts:
- ZK guest program (`./guests/geo-location-prover/main.rs`)
- Smart Contract Verifier (`./contracts/src/GeoLocationProofVerifier.sol`)

ZK guest program would prove/generate a **`ZK Proof of location` of DePIN device**.  
(In other words, this protocol can attest a `"proof of location"` without revealing a DePIN device's exact locations.)


<br>

## DEMO Video

- DEMO Video: https://vimeo.com/1085187248/bdc86dc443

<br>

## Deployed-addresses on [`BASE Sepolia`](https://sepolia.basescan.org) testnet

| Contract Name | Descripttion | Deployed-contract addresses on BASE Sepolia Testnet | Contract Source Code Verified |
| :-------------: | :----------------------------------------------------------------| :----------------------------------------------------------------:| :------------------------------------------:|
| GeoLocationProofVerifier | This contract enable to verify whether or not a ZK Proof, which show a DePIN device is working in legal region, is a valid proof without revealing their exact locations (GPS coordinates) | [0xa96B774483f83c39f009e86A8B16f1A21f2570FC](https://sepolia.basescan.org/address/0xa96b774483f83c39f009e86a8b16f1a21f2570fc#code) | [Contract Source Code Verified](https://sepolia.basescan.org/address/0xa96b774483f83c39f009e86a8b16f1a21f2570fc#code) |


<br>

## Installtion

### Install the cargo packages

- 1/ Install the Cargo packages
```bash
cargo build
```

- 2/ Within the `contracts/test/Elf.sol`, the **path** (`SMART_METER_PATH`) should be fixed like this:
```solidity
library Elf {
    string public constant GEO_LOCATION_PROVER_PATH =
        "../target/riscv-guest/guests/geo-location-prover/riscv32im-risc0-zkvm-elf/release/geo-location-prover.bin";
}
```

<br>

### Compile the smart contracts

- Compile the smart contracts
```bash
forge build
```

<br>


### Run the test of the ZK guest program
- Run the ZK guest program (`main()` in the `./guests/tests/runningGuestProgram_location-prover-test.sh`):
```bash
sh ./guests/tests/runningGuestProgramTest_geo-location-prover-test.sh
```

<br>

### Set up your environment

Add your Sepolia testnet wallet private key to an `env` file:

```bash
WALLET_PRIVATE_KEY="YOUR_WALLET_PRIVATE_KEY"
```

To allow provers to access your zkVM guest binary, it must be uploaded to a public URL. For this example we will upload to IPFS using Pinata. Pinata has a free tier with plenty of quota to get started. Sign up at [Pinata](https://pinata.cloud/), generate an API key, and set the JWT as an environment variable:

```bash
PINATA_JWT="YOUR_PINATA_JWT"
```

A `.env` file is provided with the Boundless contract deployment information for Sepolia.
The example app reads from this `.env` file automatically.

<br>

### Run the (backend) app
- Run the `./apps/src/main.rs`:
```bash
sh ./apps/runningApp_main.sh
```

<br>

### Run the test of the smart contract
- 1/ Within the `contracts/test/Elf.sol`, the **path** (`SMART_METER_PATH`) should be fixed like this:
```solidity
library Elf {
    string public constant GEO_LOCATION_PROVER_PATH =
        "../target/riscv-guest/guests/geo-location-prover/riscv32im-risc0-zkvm-elf/release/geo-location-prover.bin";
}
```

- 2/ Run the test of the smart contract (`./contracts/test/GeoLocationProofVerifier.t.sol`):
```bash
sh ./contracts/test/runningTest_GeoLocationProofVerifier.sh
```

<br>

### Deploy the smart contracts
- [NOTE]: The `GeoLocationProofVerifier` contract has [already been deployed on BASE testnet](https://github.com/masaun/zk-geo-location-prover?tab=readme-ov-file#deployed-addresses-on-base-sepolia-testnet). Hence, You can use it and therefore you basically do not need to deploy via the following command:
- Deploy the `GeoLocationProofVerifier` contract on [`BASE` testnet](https://sepolia.basescan.org):
```bash
sh ./contracts/scripts/runningScript_Deploy.sh
```

<br>

## References and Resources

- RISC Zero: https://risczero.com/  
  - Doc (zkVM / Bonsai): https://dev.risczero.com/api  
  - Boundless: https://beboundless.xyz/  
    - Boundless Doc: https://docs.beboundless.xyz/introduction/why-boundless
    - Deployed contract addresses:
      - Deployed Verifier contract addresses: https://docs.beboundless.xyz/developers/smart-contracts/verifier-contracts#base-sepolia-84532


- BASE Sepolia testnet: 
  - Block Explorer: https://sepolia.basescan.org/