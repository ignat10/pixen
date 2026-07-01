use pixen::count_in_region;

use crate::data::*;

#[test]
fn test_count_in_region() {
    let result = count_in_region(&NTH_SCREEN, &PNG_SAMPLE, [[01, 23], [456, 789]], DARK_TOLERANCE);
    assert_eq!(result, 3);
}