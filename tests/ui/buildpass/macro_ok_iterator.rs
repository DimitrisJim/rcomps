// Check that successful cases compile ok, don't check result.
// Build pass: Compilation and linking should succeed but don't run.
// build-pass
extern crate rcomps;
use rcomps::comp;


/// Iterator valid
fn test_ok_it(){
    use std::collections::HashSet;

    let cl = || {vec![1, 2, 3]};
    comp!([for i in cl() => 0]);
    comp!({for i in cl() => 0});
    comp!({for i in cl() => 0, 0});

    let s = comp!({for i in 0..=20 => i});
    comp!([for i in s.iter() => i+1]);
    comp!({for i in s.iter() => i+1});

    let mut u = HashSet::new();
    u.insert(20);

    comp!({for i in u.iter() => i, i});
}

fn main() {}
