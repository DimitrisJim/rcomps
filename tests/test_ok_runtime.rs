/// Runtime tests that evaluate that the output is as expected.
/// (NOTE: Could also use RunPass, I believe.)
use rcomps::comp;
use std::collections::HashSet;

// TODO: Ok for now, needs updating when macro actually does stuff.
#[test]
fn test_basic() {
    let tvec = vec![20, 30, 40];
    let res = comp!([for n in tvec => 2 + n + 5]);
    assert_eq!(res, vec![27, 37, 47]);
    let t = comp!([for n in 1..10 => 0; if 2 > 4]);
    assert_eq!(t, Vec::<i32>::new());
    let s = comp!{{for n in 1..5 => n; if true}};
    let s2: HashSet<i32> = (1..5).collect();
    assert_eq!(s, s2);
}