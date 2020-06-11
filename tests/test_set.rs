/// Runtime tests for vectors that evaluate that the output
/// is as expected.
use rcomps::comp;
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
    let mut hs = create!(hs );
    let mut hscomp = comp!({for u in 0..5 => u; if false});
    assert_eq!(hscomp, hs);

    hs = create!(hs 1, 2, 3, 4, 5);
    hscomp = comp!({for i in 1..6 => i});
    assert_eq!(hscomp, hs);
}

#[test]
fn test_btreeset() {
    let mut bt = create!(bt );
    let mut btcomp = comp!({for u in 0..5 => u; if false}, BTreeSet);
    assert_eq!(btcomp, bt);

    bt = create!(bt 1, 2, 3, 4, 5);
    btcomp = comp!({for i in 1..6 => i}, BTreeSet);
    assert_eq!(btcomp, bt);
}