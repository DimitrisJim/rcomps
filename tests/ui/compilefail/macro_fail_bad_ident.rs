// Check cases that compilefail.
//
// See https://rustc-dev-guide.rust-lang.org/tests/adding.html#error-annotations
// for annotating the snippets with the expected error.
// ignore-windows
extern crate rcomps;
use rcomps::comp;

/// Bad identifier checks
fn test_fail_bad_ident(){
    let c = || {42};
    comp!([for 40 in 1..2 => 30]);     //~ ERROR Unable to parse expression.
    comp!({for 40 in 1..2 => 0});      //~ ERROR Unable to parse expression.
    comp!({for 40 in 1..2 => 0, 0});   //~ ERROR Unable to parse expression.

    // Second error concerns mismatched types. TODO: Why is it raised?
    comp!([for (2 > 1) in 1..2 => 20]);     //~ ERROR Unable to parse expression.
    comp!({for (2 > 1) in 1..2 => 20});     //~ ERROR Unable to parse expression.
    comp!({for (2 > 1) in 1..2 => 20, 20}); //~ ERROR Unable to parse expression.

    comp!([for c() in 1..2 => 0]);       //~ ERROR Unable to parse expression.
    comp!([for (c()) in 1..2 => 0]);     //~ ERROR Unable to parse expression.
    comp!({for c() in 1..2 => 0});       //~ ERROR Unable to parse expression.
    comp!({for (c()) in 1..2 => 0});     //~ ERROR Unable to parse expression.
    comp!({for c() in 1..2 => 0, 0});    //~ ERROR Unable to parse expression.
    comp!({for (c()) in 1..2 => 0, 0});  //~ ERROR Unable to parse expression.

    comp!([for r#self in 3..6 => 0]);       //~ ERROR `self` cannot be a raw identifier
    comp!({for r#self in 3..6 => 0});       //~ ERROR `self` cannot be a raw identifier
    comp!({for r#self in 3..6 => 0, 0});    //~ ERROR `self` cannot be a raw identifier
}

fn main(){}