/// Runtime tests for vectors that evaluate that the output
/// is as expected.
use rcomps::comp;
use std::collections::{HashMap, BTreeMap};

#[test]
fn test_map(){
    let _unused = BTreeMap::<i32, i32>::new();
    let hm = HashMap::<i32, i32>::new();
    let hmcomp = comp!({for u in 0..5 => u, u; if false});
    assert_eq!(hmcomp, hm);
}
