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