use pixen::matches_at;

use crate::data::*;

#[test]
fn matched_at() {
    let result = matches_at(&*SCREEN, &*SAMPLE, COORDS, TOLERANCE);
    assert!(result);

    let result = matches_at(&*PNG_SCREEN, &*PNG_SAMPLE, PNG_COORDS, TOLERANCE);
    assert!(result);
}

#[test]
fn not_matched_at() {
    let result = matches_at(&*SCREEN, &*SAMPLE, [0, 0], TOLERANCE);
    assert!(!result);

    assert!(!result);

    let result = matches_at(&*SCREEN, &*DIFFERENT_SAMPLE, [0, 0], TOLERANCE);
    assert!(!result);

    let result = matches_at(&*PNG_SCREEN, &*PNG_SAMPLE, [0, 0], TOLERANCE);
    assert!(!result);

    let result = matches_at(&*PNG_SCREEN, &*PNG_NOT_SAMPLE, [0, 0], TOLERANCE);
    assert!(!result);
}
