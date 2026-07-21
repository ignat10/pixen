use pixen::find_all;

use crate::data::*;

#[test]
fn test_find_all() {
    let result = find_all(&*NTH_SCREEN, &*PNG_SAMPLE, 1);
    assert_eq!(
        result,
        vec![
            [73, 50],
            [524, 255],
            [193, 295],
            [36, 450],
            [560, 525],
            [565, 715],
            [394, 928],
        ]
    );
}
