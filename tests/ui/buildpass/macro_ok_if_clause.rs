// Check that successful cases compile ok, don't check result.
// Build pass: Compilation and linking should succeed but don't run.
// build-pass
extern crate rcomps;
use rcomps::comp;

/// Test that the if clause can capture expressions correctly.
fn test_ok_if_clause(){
    // Ok, basic.
    comp!([for i in 1..4 => 0; if true]);
    comp!({for i in 1..4 => 0; if true});
    comp!({for i in 1..4 => 0, 0; if true});

    // Ok, if has many expressions.
    comp!([for i in 1..4 => 0; if !true && !false]);
    comp!({for i in 1..4 => 0; if !true && !false});
    comp!({for i in 1..4 => 0, 0; if !true && !false});
}

fn main() {}