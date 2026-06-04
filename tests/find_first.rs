use pixen::find_first;

mod images;
use images::*;


#[test]
fn found_nth() {
    let result = find_first(&*SCREEN, &*SAMPLE);
    assert_eq!(result, Some([1, 1]));
}
