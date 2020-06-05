// Check cases that fail.
//
// See https://rustc-dev-guide.rust-lang.org/tests/adding.html#error-annotations
// for annotating the snippets with the expected error.
extern crate rcomps;
use rcomps::comp;

/// Evaluate all combinations for empty comprehensions.
fn empty_comp(){
    comp!([]); //~ ERROR Unable to parse expression.
    comp!({}); //~ ERROR Unable to parse expression.
    comp!(()); //~ ERROR Unable to parse expression.

    comp!{[]}; //~ ERROR Unable to parse expression.
    comp!{{}}; //~ ERROR Unable to parse expression.
    comp!{()}; //~ ERROR Unable to parse expression.

    comp![[]]; //~ ERROR Unable to parse expression.
    comp![{}]; //~ ERROR Unable to parse expression.
    comp![()]; //~ ERROR Unable to parse expression.
}

fn main() {}
