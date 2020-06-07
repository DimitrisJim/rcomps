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

/// Check that an identifier is reported for the for loop id.
fn need_ident_for_for(){
    let c = || {42};
    comp!([20 for 30 in 1..2]);
    //~^ ERROR An identifier is required in the 'for ident in it' clause.

    // Second error concerns mismatched types. TODO: Why is it raised?
    comp!([20 for (2 > 1) in 1..2]);
    //~^ ERROR An identifier is required in the 'for ident in it' clause.

    comp!([20 for c() in 1..2]);
    //~^ ERROR Unable to parse expression.
    comp!([20 for (c()) in 1..2]);
    //~^ ERROR An identifier is required in the 'for ident in it' clause.


    comp!((20 for 40 in 1..2));
    //~^ ERROR An identifier is required in the 'for ident in it' clause.
    comp!({20 for 40 in 1..2});
    //~^ ERROR An identifier is required in the 'for ident in it' clause.
    comp!({30:20 for 40 in 1..2});
    //~^ ERROR An identifier is required in the 'for ident in it' clause.
}

/// Check that not supplying an iterator bails.
/// Using verbatim error message kinda tricky and I'm sure might easily brake with future
/// releases of rust so, leave it bare.
fn check_no_it() {
    comp!([20 for n in 1]);   //~ ERROR
    comp!([0 for i in Vec]);  //~ ERROR
    comp!([0 for i in ]); //~ ERROR An iterator must be supplied.
    comp!([0 for i in                ]);
    //~^ ERROR An iterator must be supplied.
    comp!([n for n in ()]); // "62:23: 62:25: `()` is not an iterator [E0277]",
    //~^ ERROR

}

fn sanity(){
    comp!([20]); //~ ERROR Unable to parse expression.
    comp!([for n in 1..2 30]); //~ ERROR Unable to parse expression.
    // todo: how do we handle this?
    comp!([n for n in n]); //~ ERROR
}
fn main() {}
