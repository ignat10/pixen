use pixen::find_all_in_region;

use crate::data::*;

#[test]
fn test_find_all_in_region() {
    let result = find_all_in_region(&NTH_SCREEN, &PNG_SAMPLE, [[01, 23], [456, 789]], DARK_TOLERANCE);
    assert_eq!(result, vec![[73, 50], [193, 295], [36, 450]]);
}
