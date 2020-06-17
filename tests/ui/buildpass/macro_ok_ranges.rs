// Check that successful cases compile ok, don't check result.
// Build pass: Compilation and linking should succeed but don't run.
// build-pass
extern crate rcomps;
use rcomps::comp;

/// Test that the different forms for ranges are caught ok.
fn test_ok_range_forms(){
    // Ok, Range.
    comp!([for i in 1..2 => 0]);
    comp!({for i in 1..2 => 0});
    comp!({for i in 1..2 => 0, 0});

    // Ok, RangeFrom is accepted [todo: is there much point in it?]
    comp!([for i in 1.. => 0]);
    comp!({for i in 1.. => 0});
    comp!({for i in 1.. => 0, 0});

    // Ok, Range inclusive.
    comp!([for i in 0..=5 => 0]);
    comp!({for i in 0..=5 => 0});
    comp!({for i in 0..=5 => 0, 0});
}

fn main() {}