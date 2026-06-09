pub mod images;

use pixen::{matches, matches_at};

use images::*;

#[test]
fn test_match() {
    let result = matches_at(&*SCREEN, &*SAMPLE, [1, 1]);
    assert!(result);

    let result = matches(&*SCREEN, &*SAMPLE);
    assert!(result);
}

#[test]
fn test_png_match() {
    let result = matches_at(&*PNG_SCREEN, &*PNG_SAMPLE, [565, 715]);
    assert!(result);

    let result = matches(&*PNG_SCREEN, &*PNG_SAMPLE);
    assert!(result);
}