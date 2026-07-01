use pixen::get_tolerance;

use crate::data::*;


#[test]
fn got_tolerance() {
    let (tolerance, point) = get_tolerance(&SCREEN, &SAMPLE);
    assert!(tolerance < TOLERANCE);
    assert_eq!(point, COORDS);
}

#[test]
fn got_png_tolerance() {
    let (tolerance, point) = get_tolerance(&PNG_SCREEN, &PNG_SAMPLE);
    assert!(tolerance < DARK_TOLERANCE);
    assert_eq!(point, PNG_COORDS);
}