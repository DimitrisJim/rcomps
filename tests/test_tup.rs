/// Runtime tests for vectors that evaluate that the output
/// is as expected.
use rcomps::comp;


#[test]
fn test_tup() {
    // returns (), temporarily.
    comp!((for i in 1..2 => 0));
}