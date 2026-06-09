pub mod images;

use pixen::is_present;

use images::*;

#[test]
fn test_match() {
    assert!(is_present(&*SCREEN, &*SAMPLE, Some([1, 1])));
    assert!(is_present(&*SCREEN, &*SAMPLE, None));
}

#[test]
fn test_png_match() {
    assert!(is_present(&*PNG_SCREEN, &*PNG_SAMPLE, Some([565, 715])));
    assert!(is_present(&*PNG_SCREEN, &*PNG_SAMPLE, None));
}