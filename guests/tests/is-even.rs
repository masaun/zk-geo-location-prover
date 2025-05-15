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

use alloy_primitives::U256;
use alloy_sol_types::SolValue;
use guests::IS_EVEN_ELF;
use risc0_zkvm::{default_executor, ExecutorEnv};

#[test] // [Result]: Successful to pass.
fn proves_geo_location_is_outside_of_unacceptable_geo_location() {
    // @dev - A given input geo-location (x, y) is outside of the unacceptable geo-location (x, y).
    let geo_location_x = 10; /// @dev - Acceptable coordidates (x, y) for the location. This will be used for the constraint.
    let geo_location_y = 20; /// @dev - Acceptable coordidates (x, y) for the location. This will be used for the constraint.
    let input_bytes = (U256::from(geo_location_x), U256::from(geo_location_y)).abi_encode();

    let env = ExecutorEnv::builder()
        .write_slice(&input_bytes)
        .build()
        .unwrap();

    // NOTE: Use the executor to run tests without proving.
    let session_info = default_executor().execute(env, IS_EVEN_ELF).unwrap();

    // @dev - Check whether or not a journal (publicOutputs) is same with a given input.
    let is_outside_of_acceptable_location = bool::abi_decode(&session_info.journal.bytes, true).unwrap();
    println!("In the test of the guest program - 'is_outside_of_acceptable_location': {:?}", is_outside_of_acceptable_location);
    assert_eq!(is_outside_of_acceptable_location, true);
}

#[test]
#[should_panic(expected = "A given input geo location must be outside of unacceptable geo location")]
fn rejects_geo_location() {
    // @dev - A given input geo-location (x, y) is outside of the unacceptable geo-location (x, y).
    let geo_location_x = 15; /// @dev - Acceptable coordidates (x, y) for the location. This will be used for the constraint.
    let geo_location_y = 10; /// @dev - Acceptable coordidates (x, y) for the location. This will be used for the constraint.
    let input_bytes = (U256::from(geo_location_x), U256::from(geo_location_y)).abi_encode();

    let env = ExecutorEnv::builder()
        .write_slice(&input_bytes)
        .build()
        .unwrap();

    // NOTE: Use the executor to run tests without proving.
    let session_info = default_executor().execute(env, IS_EVEN_ELF).unwrap();
}
