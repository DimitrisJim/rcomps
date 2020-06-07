// Runtime tests that evaluate that the output is
// as expected. (NOTE: Could also use RunPass, I believe.)
use rcomps::comp;

#[test]
fn simple_test() {
    let tvec = vec![20, 30, 40];
    let v = comp!([20 for n in tvec]);
    assert_eq!(v, vec![20, 30]);
}