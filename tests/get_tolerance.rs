use pixen::get_tolerance;

mod images;
use images::*;


#[test]
fn got_tolerance() {
    let result = get_tolerance(&SCREEN, &*SAMPLE);
    assert_eq!(result, (3, COORDS));
}

#[test]
fn tolerance() {
    let result = get_tolerance(&PNG_SCREEN, &*PNG_SAMPLE);
    assert_eq!(result, (0, PNG_COORDS));
}