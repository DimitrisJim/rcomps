// Check that successful cases compile ok, don't check result.
// Build pass: Compilation and linking should succeed but don't run.
// build-pass
extern crate rcomps;
use rcomps::comp;

/// Basic checks.
fn test_ok_basiccomp(){
    // Using a range.
    let empty_vec: Vec<i32> = vec![];
    comp!([for n in 1..2 => 20]);
    comp!({for n in 1..2 => 20});
    comp!({for n in 1..2 => 10, 20});

    comp!([for n in &empty_vec => 10]);
    comp!({for n in &empty_vec => 20});
    comp!({for n in &empty_vec => 10, 20});

    comp!([for n in 1..2 => n]);
    comp!({for n in 1..2 => n});
    comp!({for n in 1..2 => n, n});

    comp!([for n in &empty_vec => n]);
    comp!({for n in &empty_vec => n});
    // todo: why does this compilefail but l28 in test_map doesn't?
    // comp!({for n in &empty_vec => n, n});
}

fn main() {}
