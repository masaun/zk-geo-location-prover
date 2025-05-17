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

import {console2} from "forge-std/console2.sol";
import {Test} from "forge-std/Test.sol";
import {RiscZeroCheats} from "risc0/test/RiscZeroCheats.sol";
import {Receipt as RiscZeroReceipt} from "risc0/IRiscZeroVerifier.sol";
import {RiscZeroMockVerifier} from "risc0/test/RiscZeroMockVerifier.sol";
import {VerificationFailed} from "risc0/IRiscZeroVerifier.sol";
import {GeoLocationProofVerifier} from "../src/GeoLocationProofVerifier.sol";
import {ImageID} from "../src/ImageID.sol";

contract GeoLocationProofVerifierTest is RiscZeroCheats, Test {
    GeoLocationProofVerifier public geoLocationProofVerifier;
    RiscZeroMockVerifier public verifier;

    function setUp() public {
        verifier = new RiscZeroMockVerifier(0);
        geoLocationProofVerifier = new GeoLocationProofVerifier(verifier);
        //assertEq(geoLocationProofVerifier.get(), 0);
    }

    function test_verifyGeoLocationProof() public {
        uint256 number = 12345678;
        RiscZeroReceipt memory receipt = verifier.mockProve(ImageID.GEO_LOCATION_PROVER_ID, sha256(abi.encode(true)));

        bool isValidProof = geoLocationProofVerifier.verifyGeoLocationProof(receipt.seal, true);
        assertEq(isValidProof, true);
    }

    // Try using a proof for the evenness of 4 to set 1 on the contract.
    function test_RejectInvalidProof() public {
        RiscZeroReceipt memory receipt = verifier.mockProve(ImageID.GEO_LOCATION_PROVER_ID, sha256(abi.encode(true)));

        vm.expectRevert(VerificationFailed.selector);
        bool isValidProof = geoLocationProofVerifier.verifyGeoLocationProof(receipt.seal, false);
        assertEq(isValidProof, false);
    }
}
