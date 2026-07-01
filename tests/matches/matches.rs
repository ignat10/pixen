use pixen::matches;

use crate::data::*;

#[test]
fn matched() {
    let result = matches(&*SCREEN, &*SAMPLE, TOLERANCE);
    assert!(result);

    let result = matches(&*PNG_SCREEN, &*PNG_SAMPLE, TOLERANCE);
    assert!(result);
}

#[test]
fn not_matched() {
    let result = matches(&*SCREEN, &*DIFFERENT_SAMPLE, TOLERANCE);
    assert!(!result);

    let result = matches(&*PNG_SCREEN, &*PNG_NOT_SAMPLE, TOLERANCE);
    assert!(!result);
}

