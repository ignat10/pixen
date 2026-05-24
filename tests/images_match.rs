mod images;

use pixen::images_match;

use images::*;

#[test]
fn test_match() {
    let result = images_match(&*SCREEN, &*SAMPLE, [1, 1]);
    assert!(result);
}

#[test]
fn test_not_match() {
    let result = images_match(&*SCREEN, &*SAMPLE, [1, 0]);
    assert!(!result);
}