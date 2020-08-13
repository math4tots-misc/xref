//! TODO: write some good tests
use super::XRef;
use std::cell::RefCell;


#[test]
fn into_owned() {
    let cell = RefCell::new(vec![1, 2, 3]);
    let xr = XRef::Ref(cell.borrow());
    let vec = xr.into_owned();
    assert_eq!(vec, vec![1, 2, 3]);
}
