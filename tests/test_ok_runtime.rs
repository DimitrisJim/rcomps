/// Runtime tests that evaluate that the output is as expected.
/// (NOTE: Could also use RunPass, I believe.)
use rcomps::comp;

// TODO: Ok for now, needs updating when macro actually does stuff.
#[test]
fn test_basic() {
    let tvec = vec![20, 30, 40];
    comp!([for n in tvec => 2 + 4 + 5]);
}