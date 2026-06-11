pub mod images;

use pixen::{matches, matches_at, matches_with_hint};

use images::*;

#[test]
fn matched() {
    let result = matches(&*SCREEN, &*SAMPLE);
    assert!(result);

    let result = matches(&*PNG_SCREEN, &*PNG_SAMPLE);
    assert!(result);
}

#[test]
fn not_matched() {
    let result = matches(&*SCREEN, &*DIFFERENT_SAMPLE);
    assert!(!result);

    let result = matches(&*PNG_SCREEN, &*PNG_NOT_SAMPLE);
    assert!(!result);
}

#[test]
fn matched_at() {
    let result = matches_at(&*SCREEN, &*SAMPLE, COORDS);
    assert!(result);

    let result = matches_at(&*PNG_SCREEN, &*PNG_SAMPLE, PNG_COORDS);
    assert!(result);
}

#[test]
fn not_matched_at() {
    let result = matches_at(&*SCREEN, &*SAMPLE, [0, 0]);
    assert!(!result);

    assert!(!result);

    let result = matches_at(&*SCREEN, &*DIFFERENT_SAMPLE, [0, 0]);
    assert!(!result);

    let result = matches_at(&*PNG_SCREEN, &*PNG_SAMPLE, [0, 0]);
    assert!(!result);

    let result = matches_at(&*PNG_SCREEN, &*PNG_NOT_SAMPLE, [0, 0]);
    assert!(!result);
}

#[test]
fn matched_with_hint() {
    let result = matches_with_hint(&*SCREEN, &*SAMPLE, COORDS);
    assert!(result);

    let result = matches_with_hint(&*SCREEN, &*SAMPLE, [0, 0]);
    assert!(result);

    let result = matches_with_hint(&*PNG_SCREEN, &*PNG_SAMPLE, PNG_COORDS);
    assert!(result);

    let result = matches_with_hint(&*PNG_SCREEN, &*PNG_SAMPLE, [0, 0]);
    assert!(result);
}

#[test]
fn not_matched_with_hint() {
    let result = matches_with_hint(&*SCREEN, &*DIFFERENT_SAMPLE, [0, 0]);
    assert!(!result);

    let result = matches_with_hint(&*PNG_SCREEN, &*PNG_NOT_SAMPLE, [0, 0]);
    assert!(!result);
}
