use pixen::count;

use crate::data::*;



#[test]
fn test_count() {
    let result = count(&*NTH_SCREEN, &*PNG_SAMPLE, 1);
    assert_eq!(result, 7);
}