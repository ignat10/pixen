use pixen::get_tolerance_in_region;

use crate::data::*;

#[test]
fn test_get_tolerance_in_region() {
    let (tolerance, point) = get_tolerance_in_region(&SCREEN, &SAMPLE, [[1, 0], [15, 14]]);
    assert!(tolerance < TOLERANCE);
    assert_eq!(point, COORDS);
}

#[test]
fn test_get_tolerance_in_region_png() {
    let (tolerance, point) = get_tolerance_in_region(&PNG_SCREEN, &PNG_SAMPLE, [[400, 300], [710, 890]]);
    assert!(tolerance < DARK_TOLERANCE);
    assert_eq!(point, PNG_COORDS);
}