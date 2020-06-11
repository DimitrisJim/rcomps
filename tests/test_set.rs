/// Runtime tests for vectors that evaluate that the output
/// is as expected.
use rcomps::comp;
use std::iter::FromIterator;
use std::collections::{HashSet, BTreeSet};

// note: _name is used because we get complaints for unused mut when set is empty
macro_rules ! create {
    (hs $($exp:expr),*) => {{
        let mut _hs = HashSet::new();
        $(
            _hs.insert($exp);
        )*
        _hs
    }};
    (bt $($exp:expr),*) => {{
        let mut _bt = BTreeSet::new();
         $(
            _bt.insert($exp);
        )*
        _bt
    }}
}

#[test]
fn test_hashset() {
    // create empty set with empty iterator:
    let mut s = comp!({for i in vec![] => i});
    assert_eq!(s, create!(hs ));

    // Create empty set with if false.
    s = comp!({for i in 0..10 => i; if false});
    assert_eq!(s, create![hs ]);

    // create simple sets for each case:
    s = comp!({for i in 0..10 => i});
    assert_eq!(s, HashSet::from_iter(0..10));

    // stupid filtering.
    s = comp!({for i in 0..10 => i; if i == 4});
    assert_eq!(s, create!(hs 4));
    // create closure to filter.
    let odds = |n| { n % 2 == 1};
    s = comp!({for i in 0..10 => i; if odds(i)});
    assert_eq!(s, create![hs 1, 3, 5, 7, 9]);

    // pass comp as iter
    s = comp!({for i in comp!({for i in 0..5 => i}) => i});
    assert_eq!(s, comp![{for i in 0..5 => i}]);

    // filter dupes.
    s = comp!({for i in vec![0, 0, 0, 21, 0, 1, 1, 23] => i});
    assert_eq!(s, create!(hs 0, 21, 1, 23));
}

#[test]
fn test_btreeset() {
    // create empty set with empty iterator:
    let mut s = comp!({for i in vec![] => i}, BTreeSet);
    assert_eq!(s, create!(bt ));

    // Create empty vector with if false.
    s = comp!({for i in 0..10 => i; if false}, BTreeSet);
    assert_eq!(s, create![bt ]);

    // create simple vectors for each case:
    s = comp!({for i in 0..10 => i}, BTreeSet);
    assert_eq!(s, BTreeSet::from_iter(0..10));

    // stupid filtering.
    s = comp!({for i in 0..10 => i; if i == 4}, BTreeSet);
    assert_eq!(s, create!(bt 4));
    // create closure to filter.
    let odds = |n| { n % 2 == 1};
    s = comp!({for i in 0..10 => i; if odds(i)}, BTreeSet);
    assert_eq!(s, create![bt 1, 3, 5, 7, 9]);

    // pass comp as iter
    s = comp!({for i in comp!({for i in 0..5 => i}, BTreeSet) => i}, BTreeSet);
    assert_eq!(s, comp![{for i in 0..5 => i}, BTreeSet]);

    // filter dupes.
    s = comp!({for i in vec![0, 0, 0, 21, 0, 1, 1, 23] => i}, BTreeSet);
    assert_eq!(s, create!(bt 0, 21, 1, 23));
}