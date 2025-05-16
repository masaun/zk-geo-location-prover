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

mod geo_location_checker;
mod sanctioned_locations_list;
use geo_location_checker::{is_geo_location_acceptable};

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
    let (geo_location_x, geo_location_y): (U256, U256) = <(U256, U256)>::abi_decode(&input_bytes, true).unwrap();
    println!("In the guest program - 'geo_location_x': {:?}", geo_location_x);
    println!("In the guest program - 'geo_location_y': {:?}", geo_location_y);

    // @dev - Constraint, which check whether or not a given input geo-location is outside of the acceptable geo-location.   
    let is_outside_of_acceptable_location: bool = is_geo_location_acceptable(geo_location_x, geo_location_y); // A given input coordinates (x, y)
    assert!(is_outside_of_acceptable_location == true, "A given input geo location must be outside of unacceptable geo location");
    println!("In the guest program - 'is_outside_of_acceptable_location': {:?}", is_outside_of_acceptable_location);

    // Commit the journal that will be received by the application contract.
    // Journal is encoded using Solidity ABI for easy decoding in the app contract.
    // @dev - Commit the result of the geo-location check as a "journal" (= pubicInput). 
    // @dev - This enable a DePIN device to prove whether or not the DePIN device is existing outside of the acceptable geo-location without revealing the exact location (coordinate (x, y)).
    env::commit_slice(is_outside_of_acceptable_location.abi_encode().as_slice()); // True or False
}