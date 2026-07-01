use pixen::get_nth_tolerance_in_region;

use crate::data::*;

#[test]
fn test_got_tolerance_in_region() {
    let (tolerance, point) = get_nth_tolerance_in_region(&SCREEN, &SAMPLE, [[1, 0], [14, 14]], 0);
    assert_eq!(tolerance, 3);
    assert_eq!(point, COORDS);
}

#[test]
fn test_got_tolerance_in_region_png() {
    let (tolerance, point) = get_nth_tolerance_in_region(&PNG_SCREEN, &PNG_SAMPLE, [[120, 432], [720, 890]], 0);
    assert!(tolerance < 2);
    assert_eq!(point, PNG_COORDS);
}

#[test]
fn test_got_tolerance_in_region_nth_png() {
    let (tolerance, point) = get_nth_tolerance_in_region(&NTH_SCREEN, &PNG_SAMPLE, [[1, 0], [678, 901]], 3);
    assert!(tolerance < DARK_TOLERANCE);
    assert_eq!(point, [524, 255]);
}

#[test]
fn test_not_got_tolerance_in_region_nth_png() {
    let (tolerance, _) = get_nth_tolerance_in_region(&NTH_SCREEN, &PNG_SAMPLE, [[1, 0], [680, 1100]], 23);
    assert!(tolerance > TOLERANCE);
}