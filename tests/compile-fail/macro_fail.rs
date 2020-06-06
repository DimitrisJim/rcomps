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

fn need_ident_for_for(){
    comp!([20 for 30 in 40]);
    //~^ ERROR An identifier is required in the 'for ident in it' clause.
}

fn main() {}
