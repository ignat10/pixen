use pixen::get_nth_tolerance;

mod images;
use images::*;


#[test]
fn test_get_nth_tolerance() {
    let result = get_nth_tolerance(&NTH_SCREEN, &PNG_SAMPLE, 5);
    assert!(result.0 < 3);
    assert_eq!(result.1, [560, 525]);
}