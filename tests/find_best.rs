use pixen::find_best;

mod images;
use images::*;

#[test]
fn found_best() {
    let coords = find_best(&*SCREEN, &*SAMPLE, TOLERANCE);
    assert_eq!(coords, Some(COORDS));

    let coords = find_best(&*PNG_SCREEN, &*PNG_SAMPLE, TOLERANCE);
    assert_eq!(coords, Some(PNG_COORDS));

    let coords = find_best(&*DIRT_SCREEN, &*FOOD, TOLERANCE);
    assert_eq!(coords, Some([318, 589]));

    let coords = find_best(&*BLUR_SCREEN, &*BLUR_MAP, TOLERANCE);
    assert_eq!(coords, Some([40, 1200]));

    let coords = find_best(&*BLUR_SCREEN, &*BLUR_CITY, TOLERANCE);
    assert_eq!(coords, Some([0, 707]));
}

#[test]
fn not_found_best() {
    let coords = find_best(&*SCREEN, &*DIFFERENT_SAMPLE, TOLERANCE);
    assert_eq!(coords, None);

    let coords = find_best(&*PNG_SCREEN, &*PNG_NOT_SAMPLE, TOLERANCE);
    assert_eq!(coords, None);

    let coords = find_best(&*DIRT_SCREEN, &*BLUR_MAP, DARK_TOLERANCE);
    assert_eq!(coords, None);

    let coords = find_best(&*DIRT_SCREEN, &*BLUR_CITY, DARK_TOLERANCE);
    assert_eq!(coords, None);
}
