use pixen::find_sample;

mod images;
use images::*;


#[test]
fn found() {
    let coords = find_sample(&*SCREEN, &*SAMPLE);
    assert_eq!(coords, Some([1, 1]));
}

#[test]
fn not_found() {
    let coords = find_sample(&*SCREEN, &*DIFFERENT_SAMPLE);
    assert_eq!(coords, None);
}


#[test]
fn found_png() {
    let coords = find_sample(&*PNG_SCREEN, &*PNG_SAMPLE);
    assert_eq!(coords, Some([565, 715]))
}

#[test]
fn not_found_png() {
    let coords = find_sample(&*PNG_SCREEN, &*PNG_NOT_SAMPLE);
    assert_eq!(coords, None);
}
