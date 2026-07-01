use pixen::get_nth_tolerance;

use crate::data::*;


#[test]
fn test_get_nth_tolerance() {
    let (tolerance, point) = get_nth_tolerance(&NTH_SCREEN, &PNG_SAMPLE, 5);
    assert!(tolerance < DARK_TOLERANCE);
    assert_eq!(point, [560, 525]);
}
