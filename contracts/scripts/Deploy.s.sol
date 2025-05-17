// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pragma solidity ^0.8.20;

import {Script, console2} from "forge-std/Script.sol";
import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {GeoLocationProofVerifier} from "../src/GeoLocationProofVerifier.sol";

contract Deploy is Script {
    function run() external {
        // load ENV variables first
        uint256 key = vm.envUint("WALLET_PRIVATE_KEY");
        address verifierAddress = vm.envAddress("VERIFIER_ROUTER_ADDRESS");
        vm.startBroadcast(key);

        IRiscZeroVerifier verifier = IRiscZeroVerifier(verifierAddress);
        GeoLocationProofVerifier geoLocationProofVerifier = new GeoLocationProofVerifier(verifier);
        address geoLocationProofVerifierAddress = address(geoLocationProofVerifier);
        console2.log("Deployed GeoLocationProofVerifier to", geoLocationProofVerifierAddress);

        vm.stopBroadcast();
    }
}
