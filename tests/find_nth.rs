use pixen::find_nth;

mod images;
use images::{NTH_SCREEN, PNG_SAMPLE};

#[test]
fn found_nth() {
    let result = find_nth(&*NTH_SCREEN, &*PNG_SAMPLE, 0,6);
    assert_eq!(result, Some([394, 928]));
}

#[test]
fn not_found_nth() {
    let result = find_nth(&*NTH_SCREEN, &*PNG_SAMPLE, 100, 666);
    assert_eq!(result, None);
}
