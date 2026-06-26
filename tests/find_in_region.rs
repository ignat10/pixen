use pixen::find_in_region;

mod images;
use images::*;

#[test]
fn found_in_region() {
    let result = find_in_region(&*SCREEN, &*SAMPLE, [[1, 0], [13, 13]], TOLERANCE);
    assert_eq!(result, Some(COORDS));

    let result = find_in_region(&*PNG_SCREEN, &*PNG_SAMPLE, [[300, 500], [720, 1000]], TOLERANCE);
    assert_eq!(result, Some(PNG_COORDS));
}

#[test]
fn not_found_in_region() {
    let result = find_in_region(&*SCREEN, &*SAMPLE, [[2, 3], [13, 14]], DARK_TOLERANCE);
    assert_eq!(result, None);

    let result = find_in_region(&*PNG_SCREEN, &*PNG_SAMPLE, [[100, 200], [500, 600]], TOLERANCE);
    assert_eq!(result, None);
}