/// Runtime tests for maps that evaluate that the output
/// is as expected.
use rcomps::comp;
use std::iter::FromIterator;
use std::collections::{HashMap, BTreeMap};

// note: _name is used because we get complaints for unused mut when set is empty
macro_rules ! create {
    (hm $(($k:expr, $v:expr)),*) => {{
        let mut _hs = HashMap::new();
        $(
            _hs.insert($k, $v);
        )*
        _hs
    }};
    (bt $(($k:expr, $v:expr)),*) => {{
        let mut _bt = BTreeMap::new();
         $(
            _bt.insert($k, $v);
        )*
        _bt
    }}
}

#[test]
fn test_emtpy(){
    let u: HashMap<i32, i32> = comp!({:});
    let i: HashMap<i32, i32> = comp![{:}];
    let z: HashMap<i32, i32> = comp!{{:}};
    assert_eq!(u, i);
    assert_eq!(i, z);
}

#[test]
fn test_hashmap(){
    // create empty set with empty iterator:
    let mut m = comp!({for i in vec![] => i, i});
    assert_eq!(m, create!(hm ));

    // Create empty set with if false.
    m = comp!({for i in 0..10 => i, i; if false});
    assert_eq!(m, create![hm ]);

    // create simple sets for each case:
    m = comp!({for i in 0..10 => i, i});
    let expected = HashMap::<i32, i32>::from_iter(
        (0..10).map(|i| {(i, i)})
    );
    assert_eq!(m, expected);

    // stupid filtering.
    m = comp!({for i in 0..10 => i, i; if i == 4});
    assert_eq!(m, create!(hm (4, 4)));

    // create closure to filter.
    let odds = |n| { n % 2 == 1};
    m = comp!({for i in 0..10 => i, i; if odds(i)});
    assert_eq!(m, create![hm (1, 1), (3, 3), (5, 5), (7, 7), (9, 9)]);

    // pass comp as iter
    m = comp!({for i in comp!({for i in 0..5 => i}) => i, i});
    assert_eq!(m, comp![{for i in 0..5 => i, i}]);

    // filter dupe keys.
    m = comp!({for i in vec![0, 0, 0, 21, 0, 1, 1, 23] => i, i});
    assert_eq!(m, create!(hm (0, 0), (21, 21), (1, 1), (23, 23)));
}


#[test]
fn test_btreemap(){
    // create empty map with empty iterator:
    let mut m = comp!({for i in vec![] => i, i}, BTreeMap);
    assert_eq!(m, create!(bt ));

    // Create empty map with if false.
    m = comp!({for i in 0..10 => i, i; if false}, BTreeMap);
    assert_eq!(m, create![bt ]);

    // create simple maps for each case:
    m = comp!({for i in 0..10 => i, i}, BTreeMap);
    let expected = BTreeMap::<i32, i32>::from_iter(
        (0..10).map(|i| {(i, i)})
    );
    assert_eq!(m, expected);

    // stupid filtering.
    m = comp!({for i in 0..10 => i, i; if i == 4}, BTreeMap);
    assert_eq!(m, create!(bt (4, 4)));

    // create closure to filter.
    let odds = |n| { n % 2 == 1};
    m = comp!({for i in 0..10 => i, i; if odds(i)}, BTreeMap);
    assert_eq!(m, create![bt (1, 1), (3, 3), (5, 5), (7, 7), (9, 9)]);

    // pass comp as iter
    m = comp!({for i in comp!({for i in 0..5 => i}) => i, i}, BTreeMap);
    assert_eq!(m, comp![{for i in 0..5 => i, i}, BTreeMap]);

    // filter dupe keys.
    m = comp!({for i in vec![0, 0, 0, 21, 0, 1, 1, 23] => i, i}, BTreeMap);
    assert_eq!(m, create!(bt (0, 0), (21, 21), (1, 1), (23, 23)));
}