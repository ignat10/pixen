use pixen::{find_best, find_best_with_hint};

mod images;
use images::*;

#[test]
fn found_best() {
    let coords = find_best(&*SCREEN, &*SAMPLE);
    assert_eq!(coords, Some(COORDS));

    let coords = find_best(&*PNG_SCREEN, &*PNG_SAMPLE);
    assert_eq!(coords, Some(PNG_COORDS));
}

#[test]
fn not_found_best() {
    let coords = find_best(&*SCREEN, &*DIFFERENT_SAMPLE);
    assert_eq!(coords, None);

    let coords = find_best(&*PNG_SCREEN, &*PNG_NOT_SAMPLE);
    assert_eq!(coords, None);
}

#[test]
fn found_best_with_hint() {
    let coords = find_best_with_hint(&*SCREEN, &*SAMPLE, COORDS);
    assert_eq!(coords, Some(COORDS));

    let coords = find_best_with_hint(&*SCREEN, &*SAMPLE, [0, 0]);
    assert_eq!(coords, Some(COORDS));

    let coords = find_best_with_hint(&*PNG_SCREEN, &*PNG_SAMPLE, PNG_COORDS);
    assert_eq!(coords, Some(PNG_COORDS));

    let coords = find_best_with_hint(&*PNG_SCREEN, &*PNG_SAMPLE, [0, 0]);
    assert_eq!(coords, Some(PNG_COORDS));
}

#[test]
fn not_found_with_hint() {
    let coords = find_best_with_hint(&*SCREEN, &*DIFFERENT_SAMPLE, COORDS);
    assert_eq!(coords, None);

    let coords = find_best_with_hint(&*SCREEN, &*DIFFERENT_SAMPLE, [0, 0]);
    assert_eq!(coords, None);

    let coords = find_best_with_hint(&*PNG_SCREEN, &*PNG_NOT_SAMPLE, COORDS);
    assert_eq!(coords, None);

    let coords = find_best_with_hint(&*PNG_SCREEN, &*PNG_NOT_SAMPLE, [0, 0]);
    assert_eq!(coords, None);
}
