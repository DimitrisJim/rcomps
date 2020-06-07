// Check cases that fail.
//
// See https://rustc-dev-guide.rust-lang.org/tests/adding.html#error-annotations
// for annotating the snippets with the expected error.
extern crate rcomps;
use rcomps::comp;

/// Evaluate all combinations for empty comprehensions.
fn empty_comp(){
    comp!(); //~ ERROR Empty expression.
    comp!{()}; //~ ERROR Unable to parse expression.
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

fn check_no_it() {
    comp!([for n in 1 => 20]);   //~ ERROR
    comp!([for i in Vec => 0]);  //~ ERROR
    comp!([for i in => 2]); //~ ERROR Unable to parse expression.
    comp!([for i in                => 0]);
    //~^ ERROR Unable to parse expression.
    comp!([for n in () => n]); // "62:23: 62:25: `()` is not an iterator [E0277]",
    //~^ ERROR

    let c = || {42};
    comp!([for 30 in 1..2 => 30]);
    //~^ ERROR Unable to parse expression.

    // Second error concerns mismatched types. TODO: Why is it raised?
    comp!([for (2 > 1) in 1..2 => 20]);
    //~^ ERROR Unable to parse expression.

    comp!([for c() in 1..2 => 0]);
    //~^ ERROR Unable to parse expression.
    comp!([for (c()) in 1..2 => 0]);
    //~^ ERROR Unable to parse expression.


    comp!((for 40 in 1..2 => 0));
    //~^ ERROR Unable to parse expression.
    comp!({for 40 in 1..2 => 0});
    //~^ ERROR Unable to parse expression.
    comp!({for 40 in 1..2 => 0, 0});
    //~^ ERROR Unable to parse expression.
}

fn sanity(){
    comp!([20]); //~ ERROR Unable to parse expression.
    comp!([for n in 1..2 30]); //~ ERROR Unable to parse expression.

    // "58:21: 58:22: cannot find value `n` in this scope [E0425]"
    comp!([for n in n => n]); //~ ERROR

    comp!([for n in n]); //~ ERROR Unable to parse expression.
    comp!([for n in 1..2 => 30; if]) //~ ERROR Unable to parse expression.
}

fn main() {}

