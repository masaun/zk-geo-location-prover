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

import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {ImageID} from "./ImageID.sol"; // auto-generated contract after running `cargo build`.

/// @title A starter application using RISC Zero.
/// @notice This basic application holds a number, guaranteed to be even.
/// @dev This contract demonstrates one pattern for offloading the computation of an expensive
///      or difficult to implement function to a RISC Zero guest.
contract GeoLocationProofVerifier {
    /// @notice RISC Zero verifier contract address.
    IRiscZeroVerifier public immutable verifier;
    /// @notice Image ID of the only zkVM binary to accept verification from.
    ///         The image ID is similar to the address of a smart contract.
    ///         It uniquely represents the logic of that guest program,
    ///         ensuring that only proofs generated from a pre-defined guest program
    ///         (in this case, checking if a number is even) are considered valid.
    bytes32 public constant imageId = ImageID.GEO_LOCATION_PROVER_ID;

    /// @notice A number that is guaranteed, by the RISC Zero zkVM, to be even.
    ///         It can be set by calling the `set` function.
    uint256 public number;

    /// @notice Initialize the contract, binding it to a specified RISC Zero verifier.
    constructor(IRiscZeroVerifier _verifier) {
        verifier = _verifier;
        number = 0;
    }

    /**
     * @notice - Veriify a geo location proof.
     * @dev - "Seal" is equal meaning to a "Proof". 
     * @dev - "Journal" is equal meaning to a "PublicInputs".
     */
    function verifyGeoLocationProof(bytes calldata proof, bool isOutsideOfAcceptableLocation) public view returns (bool _isValidProof) {
        bytes memory publicInputs = abi.encode(isOutsideOfAcceptableLocation);
        verifier.verify(proof, imageId, sha256(publicInputs));
        //bool isValidProof = verifier.verify(proof, imageId, sha256(publicInputs));
        bool isValidProof = true;
        return isValidProof;
    }


    // /// @notice Set the even number stored on the contract. Requires a RISC Zero proof that the number is even.
    // function set(uint256 x, bytes calldata seal) public { /// @dev - The `seal` is the proof generated by the RISC Zero guest program.
    //     // Construct the expected journal data. Verify will fail if journal does not match.
    //     bytes memory journal = abi.encode(x);
    //     verifier.verify(seal, imageId, sha256(journal));  /// @dev - This verify() function is a "view" function.
    //     number = x;
    // }

    // /// @notice Returns the number stored.
    // function get() public view returns (uint256) {
    //     return number;
    // }
}
