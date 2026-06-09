use pixen::{find_best, find_best_with_hint};

mod images;
use images::*;


#[test]
fn found_best() {
    let coords = find_best(&*SCREEN, &*SAMPLE);
    assert_eq!(coords, Some([1, 1]));

    let coords = find_best_with_hint(&*SCREEN, &*SAMPLE, [1, 1]);
    assert_eq!(coords, Some([1, 1]));

    let coords = find_best_with_hint(&*SCREEN, &*SAMPLE, [0, 0]);
    assert_eq!(coords, Some([1, 1]))
}

#[test]
fn not_found_best() {
    let coords = find_best(&*SCREEN, &*DIFFERENT_SAMPLE);
    assert_eq!(coords, None);

    let coords = find_best_with_hint(&*SCREEN, &*DIFFERENT_SAMPLE, [0, 0]);
    assert_eq!(coords, None);
}


#[test]
fn found_best_png() {
    let coords = find_best(&*PNG_SCREEN, &*PNG_SAMPLE);
    assert_eq!(coords, Some([565, 715]));

    let coords = find_best_with_hint(&*PNG_SCREEN, &*PNG_SAMPLE, [565, 715]);
    assert_eq!(coords, Some([565, 715]));

    let coords = find_best_with_hint(&*PNG_SCREEN, &*PNG_SAMPLE, [0, 0]);
    assert_eq!(coords, Some([565, 715]));
}

#[test]
fn not_found_png() {
    let coords = find_best(&*PNG_SCREEN, &*PNG_NOT_SAMPLE);
    assert_eq!(coords, None);

    let coords = find_best_with_hint(&*PNG_SCREEN, &*PNG_NOT_SAMPLE, [012, 123]);
    assert_eq!(coords, None);
}
