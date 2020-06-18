// Check cases that compilefail.
//
// See https://rustc-dev-guide.rust-lang.org/tests/adding.html#error-annotations
// for annotating the snippets with the expected error.
// ignore-windows
extern crate rcomps;
use rcomps::comp;

/// Evaluate all combinations for empty comprehensions.
fn test_fail_empty_comp(){
    comp!();   //~ ERROR Empty expression.
    comp!{};   //~ ERROR Empty expression.
    comp![];   //~ ERROR Empty expression.
    comp!(()); //~ ERROR Unable to parse expression.
    comp![()]; //~ ERROR Unable to parse expression.
    comp!{()}; //~ ERROR Unable to parse expression.
}

fn main(){}