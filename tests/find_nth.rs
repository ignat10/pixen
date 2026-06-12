use pixen::find_nth;

mod images;
use images::{NTH_SCREEN, PNG_SAMPLE};

#[test]
fn found_nth() {
    let result = find_nth(&*NTH_SCREEN, &*PNG_SAMPLE, 6);
    assert_eq!(result, Some([394, 927]));
}

#[test]
fn not_found_nth() {
    let result = find_nth(&*NTH_SCREEN, &*PNG_SAMPLE, 666);
    assert_eq!(result, None);
}
