mod images;

use pixen::images_match;

use images::*;

#[test]
fn test_match() {
    let result = images_match(&*SCREEN, &*SAMPLE, [1u8, 1u8]);
    assert!(result);
}

#[test]
fn test_not_match() {
    let result = images_match(&*SCREEN, &*SAMPLE, [1u8, 0u8]);
    assert!(!result);
}