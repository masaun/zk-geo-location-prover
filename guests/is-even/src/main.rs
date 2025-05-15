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

use std::io::Read;

use alloy_primitives::{U256, Uint};
use alloy_sol_types::SolValue;
use risc0_zkvm::guest::env;

fn main() {    
    // Read the input data for this application.
    let mut input_bytes = Vec::<u8>::new();
    env::stdin().read_to_end(&mut input_bytes).unwrap();
    
    // Decode and parse the input
    //let number = <U256>::abi_decode(&input_bytes, true).unwrap();
    let (number, geo_location_x, geo_location_y): (U256, U256, U256) = <(U256, U256, U256)>::abi_decode(&input_bytes, true).unwrap();
    println!("In the guest program - 'number': {:?}", number);
    println!("In the guest program - 'geo_location_x': {:?}", geo_location_x);
    println!("In the guest program - 'geo_location_y': {:?}", geo_location_y);

    // Run the computation.
    // In this case, asserting that the provided number is even.
    assert!(!number.bit(0), "number is not even");

    // @dev - Acceptable coordidates (x, y) for the location. This will be used for the constraint.
    // Define acceptable and unacceptable geo-locations
    let input_geo_location = (geo_location_x, geo_location_y);  // A given input coordinates (x, y)
    assert!(is_geo_location_acceptable(input_geo_location), "input_geo_location must be outside of unacceptable geo_location");

    // Commit the journal that will be received by the application contract.
    // Journal is encoded using Solidity ABI for easy decoding in the app contract.
    env::commit_slice(number.abi_encode().as_slice());
}


/**
 * @notice - Check if the geo-location is acceptable.
 */
fn is_geo_location_acceptable(input_geo_location: (U256, U256)) -> bool {
    let unacceptable_geo_location_x = Uint::from(15 as u64);  // Example acceptable x of coordinates (x, y)
    let unacceptable_geo_location_y = Uint::from(10 as u64);  // Example acceptable y of coordinates (x, y)
    let unacceptable_geo_location = (unacceptable_geo_location_x, unacceptable_geo_location_y);  // Example unacceptable coordinates (x, y)

    // Check if the geo-location is acceptable
    input_geo_location != unacceptable_geo_location
}