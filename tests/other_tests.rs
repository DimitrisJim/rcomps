// Tests run from the normal test runner.
use rcomps::comp;

#[test]
fn simple_test() {
    let v = comp!([20 for n in 20]);
    assert_eq!(v, vec![20, 50, 20]);
}