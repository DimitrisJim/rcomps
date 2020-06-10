/// Runtime tests for vectors that evaluate that the output
/// is as expected.
use rcomps::comp;
use std::collections::{HashSet, BTreeSet};

#[test]
fn test_set() {
    let _unused = BTreeSet::<i32>::new();
    let hs = HashSet::<i32>::new();
    let hscomp = comp!({for u in 0..5 => u; if false});
    assert_eq!(hscomp, hs);
}