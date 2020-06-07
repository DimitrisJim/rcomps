/// Runtime tests that evaluate that the output is as expected.
/// (NOTE: Could also use RunPass, I believe.)
use rcomps::{comp};


// TODO: Ok for now, needs updating when macro actually does stuff.
#[test]
fn test_basic() {
    let tvec = vec![20, 30, 40];
    let mut v = comp!([2 for n in tvec]);
    assert_eq!(v, vec![20, 30, 40]);

    v = comp!([1 for n in 1..4]);
    assert_eq!(v, vec![1, 2, 3]);
}