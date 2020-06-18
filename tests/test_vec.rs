/// Runtime tests for vectors that evaluate that the output
/// is as expected.
use rcomps::comp;
use std::iter::FromIterator;
use std::collections::VecDeque;

macro_rules ! create {
    (vd $($exp:expr),*) => {{
        let mut _vdeq = VecDeque::new();
         $(
            _vdeq.push_back($exp);
        )*
        _vdeq
    }}
}

#[test]
fn test_emtpy(){
    let u: Vec<i32> = comp!([]);
    let i: Vec<i32> = comp!{[]};
    let z: Vec<i32> = comp![[]];
    assert_eq!(u, i);
    assert_eq!(i, z);
}

#[test]
fn test_vec() {
    // create empty vector with empty iterator:
    let mut v = comp!([for i in vec![] => i]);
    assert_eq!(v, vec![]);
    // Create empty vector with if false.
    v = comp!([for i in 0..10 => i; if false]);
    assert_eq!(v, vec![]);

    // create simple vectors for each case:
    v = comp!([for i in 0..10 => i]);
    assert_eq!(v, Vec::from_iter(0..10));

    // stupid filtering.
    v = comp!([for i in 0..10 => i; if i == 4]);
    assert_eq!(v, vec![4]);
    // create closure to filter.
    let odds = |n| { n % 2 == 1};
    v = comp!([for i in 0..10 => i; if odds(i)]);
    assert_eq!(v, vec![1, 3, 5, 7, 9]);

    // pass comp as iter
    v = comp!([for i in comp!([for i in 0..5 => i]) => i]);
    assert_eq!(v, comp![[for i in 0..5 => i]]);
}


#[test]
fn test_vecdeq() {
    // note: similar tests to test_vec
    // create empty vector with empty iterator:
    let mut v = comp!([for i in 0..0 => i], VecDeque);
    assert_eq!(v, create!(vd ));

    // Create empty vector with if false.
    v = comp!([for i in 0..10 => i; if false], VecDeque);
    assert_eq!(v, create![vd ]);

    // create simple vectors for each case:
    v = comp!([for i in 0..10 => i], VecDeque);
    assert_eq!(v, VecDeque::from_iter(0..10));

    // stupid filtering.
    v = comp!([for i in 0..10 => i; if i == 4], VecDeque);
    assert_eq!(v, create![vd 4]);
    // create closure to filter.
    let odds = |n| { n % 2 == 1};
    v = comp!([for i in 0..10 => i; if odds(i)], VecDeque);
    assert_eq!(v, create![vd 1, 3, 5, 7, 9]);

    // pass comp as iter
    v = comp!([
        for i in comp!([for i in 0..5 => i], VecDeque) => i
    ], VecDeque);
    assert_eq!(v, comp![[for i in 0..5 => i], VecDeque]);
}
