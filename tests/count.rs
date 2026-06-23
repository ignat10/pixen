use pixen::count;

mod images;
use images::*;



#[test]
fn test_count() {
    let result = count(&*NTH_SCREEN, &*PNG_SAMPLE, 0);
    assert_eq!(result, 7);
}