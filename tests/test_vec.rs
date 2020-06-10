/// Runtime tests for vectors that evaluate that the output
/// is as expected.
use rcomps::comp;
use std::collections::{LinkedList, VecDeque};

macro_rules ! create {
    (ll $($exp:expr),+) => {{
        let mut ll = LinkedList::new();
        $(
            ll.push_back($exp);
        )+
        ll
    }};
    (vd $($exp:expr),+) => {{
        let mut vdeq = VecDeque::new();
         $(
            vdeq.push_back($exp);
        )+
        vdeq
    }}
}


#[test]
fn test_vec() {
    let t = comp!([for n in 1..10 => 0; if 2 > 4]);
    let u: Vec<i32> = vec![];
    assert_eq!(t, u);
}


#[test]
fn test_vecdeq() {
    let res = comp!([for n in 0..5 => 2 + n + 5], VecDeque);
    let vd = create!(vd 7, 8, 9, 10, 11);
    assert_eq!(res, vd);
}


#[test]
fn test_ll() {
    let res = comp!([for n in 0..5 => 2 + n + 5], LinkedList);
    let l = create!(ll 7, 8, 9, 10, 11);
    assert_eq!(res, l);
}