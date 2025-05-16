use alloy_primitives::{U256, Uint};

/**
 * @notice - Check whether or not a given geo location (x, y) is outside the geo-location of sanctioned country A, B, and C
 */
pub fn is_geo_location_acceptable(input_geo_location_x: U256, input_geo_location_y: U256) -> bool {
    let mut is_geo_location_acceptable: bool = true; // Default value is true (acceptable geo-location)
    
    // @dev - Check whether or not a given geo location (x, y) is outside the geo-location of sanctioned country A, B, and C
    if is_outside_of_geo_location_of_sanctioned_country_A(input_geo_location_x, input_geo_location_y) == false {
        is_geo_location_acceptable = false; // The geo-location is inside of the unacceptable geo-location
    } else if is_outside_of_geo_location_of_sanctioned_country_B(input_geo_location_x, input_geo_location_y) == false {
        is_geo_location_acceptable = false; // The geo-location is inside of the unacceptable geo-location
    } else if is_outside_of_geo_location_of_sanctioned_country_C(input_geo_location_x, input_geo_location_y) == false {
        is_geo_location_acceptable = false; // The geo-location is inside of the unacceptable geo-location
    }

    is_geo_location_acceptable
}


/**
 * @notice - Check whether or not a given geo location (x, y) is outside the geo-location of sanctioned country A
 */
pub fn is_outside_of_geo_location_of_sanctioned_country_A(input_geo_location_x: U256, input_geo_location_y: U256) -> bool {
    let mut is_outside_of_geo_location_of_sanctioned_country_A: bool = true; // Default value is true (acceptable geo-location)
    
    // @dev - Check whether or not a given input (geo location) is acceptable
    // @dev - If both condition for respective coordinates (x, y) are satisfied, then the geo-location can be judged as the inside of the unacceptable geo-location.
    if U256::from(60 as u32) <= input_geo_location_x && input_geo_location_x <= U256::from(70 as u32) {     // 50 <= input_geo_location_x <= 100
        if U256::from(110 as u32) <= input_geo_location_y && input_geo_location_y <= U256::from(120 as u32) {  // 50 <= input_geo_location_y <= 100
            is_outside_of_geo_location_of_sanctioned_country_A = false;  // The geo-location is inside of the unacceptable geo-location
        }
    }

    is_outside_of_geo_location_of_sanctioned_country_A
}

/**
 * @notice - Check whether or not a given geo location (x, y) is outside the geo-location of sanctioned country B
 */
pub fn is_outside_of_geo_location_of_sanctioned_country_B(input_geo_location_x: U256, input_geo_location_y: U256) -> bool {
    let mut is_outside_of_geo_location_of_sanctioned_country_B: bool = true; // Default value is true (acceptable geo-location)
    
    // @dev - Check whether or not a given input (geo location) is acceptable
    // @dev - If both condition for respective coordinates (x, y) are satisfied, then the geo-location can be judged as the inside of the unacceptable geo-location.
    if U256::from(70 as u32) <= input_geo_location_x && input_geo_location_x <= U256::from(80 as u32) {     // 50 <= input_geo_location_x <= 100
        if U256::from(120 as u32) <= input_geo_location_y && input_geo_location_y <= U256::from(130 as u32) {  // 50 <= input_geo_location_y <= 100
            is_outside_of_geo_location_of_sanctioned_country_B = false;  // The geo-location is inside of the unacceptable geo-location
        }
    }

    is_outside_of_geo_location_of_sanctioned_country_B
}

/**
 * @notice - Check whether or not a given geo location (x, y) is outside the geo-location of sanctioned country C
 */
pub fn is_outside_of_geo_location_of_sanctioned_country_C(input_geo_location_x: U256, input_geo_location_y: U256) -> bool {
    let mut is_outside_of_geo_location_of_sanctioned_country_C: bool = true; // Default value is true (acceptable geo-location)
    
    // @dev - Check whether or not a given input (geo location) is acceptable
    // @dev - If both condition for respective coordinates (x, y) are satisfied, then the geo-location can be judged as the inside of the unacceptable geo-location.
    if U256::from(50 as u32) <= input_geo_location_x && input_geo_location_x <= U256::from(100 as u32) {     // 50 <= input_geo_location_x <= 100
        if U256::from(50 as u32) <= input_geo_location_y && input_geo_location_y <= U256::from(100 as u32) {  // 50 <= input_geo_location_y <= 100
            is_outside_of_geo_location_of_sanctioned_country_C = false;  // The geo-location is inside of the unacceptable geo-location
        }
    }

    is_outside_of_geo_location_of_sanctioned_country_C
}