use alloy_primitives::{U256, Uint};

use crate::sanctioned_locations_list::{
    is_outside_of_geo_location_of_sanctioned_country_A, 
    is_outside_of_geo_location_of_sanctioned_country_B, 
    is_outside_of_geo_location_of_sanctioned_country_C
};

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