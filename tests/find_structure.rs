use pixen::find_best;

mod images;

use images::SCREEN;
use images::DARK;

#[test]
fn found_structure() {
    let result = find_best(&*SCREEN, &*DARK);
    assert_eq!(result, Some([0, 0]))
}