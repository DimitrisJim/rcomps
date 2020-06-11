/// Runtime tests for vectors that evaluate that the output
/// is as expected.
use rcomps::comp;
use std::iter::FromIterator;
use std::collections::{LinkedList, VecDeque};

macro_rules ! create {
    (ll $($exp:expr), *) => {{
        let mut _ll = LinkedList::new();
        $(
            _ll.push_back($exp);
        )*
        _ll
    }};
    (vd $($exp:expr),*) => {{
        let mut _vdeq = VecDeque::new();
         $(
            _vdeq.push_back($exp);
        )*
        _vdeq
    }}
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


#[test]
fn test_ll() {
    // note: similar tests to test_vec
    // create empty vector with empty iterator:
    let mut v = comp!([for i in 0..0 => i], LinkedList);
    assert_eq!(v, create!(ll ));

    // Create empty vector with if false.
    v = comp!([for i in 0..10 => i; if false], LinkedList);
    assert_eq!(v, create![ll ]);

    // create simple vectors for each case:
    v = comp!([for i in 0..10 => i], LinkedList);
    assert_eq!(v, LinkedList::from_iter(0..10));

    // stupid filtering.
    v = comp!([for i in 0..10 => i; if i == 4], LinkedList);
    assert_eq!(v, create![ll 4]);
    // create closure to filter.
    let odds = |n| { n % 2 == 1};
    v = comp!([for i in 0..10 => i; if odds(i)], LinkedList);
    assert_eq!(v, create![ll 1, 3, 5, 7, 9]);

    // pass comp as iter
    v = comp!([for i in comp!([for i in 0..5 => i], LinkedList) => i], LinkedList);
    assert_eq!(v, comp![[for i in 0..5 => i], LinkedList]);
}