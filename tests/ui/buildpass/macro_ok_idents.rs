// Check that successful cases compile ok, don't check result.
// Build pass: Compilation and linking should succeed but don't run.
// build-pass
extern crate rcomps;
use rcomps::comp;

/// Ok identifiers.
fn test_ok_idents(){
    comp!([for _mynot_so_intricate_but_long_identifier in 1..2 => 0]);
    comp!({for _mynot_so_intricate_but_long_identifier in 1..2 => 0});
    comp!({for _my_not_so_intricate_but_long_identifier in 1..2 => 0, 0});

    // try raw identifier.
    comp!([for r#try in 3..6 => 0]);
    comp!({for r#try in 3..6 => 0});
    comp!({for r#try in 3..6 => 0, 0});
}

fn main() {}