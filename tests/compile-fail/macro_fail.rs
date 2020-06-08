// Check cases that fail.
//
// See https://rustc-dev-guide.rust-lang.org/tests/adding.html#error-annotations
// for annotating the snippets with the expected error.
// todo: Mostly contains cases for vector comp. Add for the rest.
extern crate rcomps;
use rcomps::comp;

/// Evaluate all combinations for empty comprehensions.
fn test_fail_empty_comp(){
    comp!();   //~ ERROR Empty expression.
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

/// Failures if improper iterator is supplied (todo: merge with test_fail_range_forms?)
fn test_fail_no_it() {
    comp!([for n in 1 => 20]);              //~ ERROR `{integer}` is not an iterator [E0277]
    comp!([for i in Vec => 0]);             //~ ERROR expected value, found struct `Vec` [E0423]
    comp!([for i in => 2]);                 //~ ERROR Unable to parse expression.
    comp!([for i in                => 0]);  //~ ERROR Unable to parse expression.
    comp!([for n in () => n]);              //~ ERROR `()` is not an iterator [E0277]
}

/// Bad identifier checks
fn test_fail_bad_ident(){
    let c = || {42};
    comp!([for 40 in 1..2 => 30]);     //~ ERROR Unable to parse expression.
    comp!((for 40 in 1..2 => 0));      //~ ERROR Unable to parse expression.
    comp!({for 40 in 1..2 => 0});      //~ ERROR Unable to parse expression.
    comp!({for 40 in 1..2 => 0, 0});   //~ ERROR Unable to parse expression.

    // Second error concerns mismatched types. TODO: Why is it raised?
    comp!([for (2 > 1) in 1..2 => 20]);     //~ ERROR Unable to parse expression.
    comp!((for (2 > 1) in 1..2 => 20));     //~ ERROR Unable to parse expression.
    comp!({for (2 > 1) in 1..2 => 20});     //~ ERROR Unable to parse expression.
    comp!({for (2 > 1) in 1..2 => 20, 20}); //~ ERROR Unable to parse expression.

    comp!([for c() in 1..2 => 0]);     //~ ERROR Unable to parse expression.
    comp!([for (c()) in 1..2 => 0]);   //~ ERROR Unable to parse expression.

    comp!([for r#self in 3..6 => 0]);       //~ ERROR `self` cannot be a raw identifier
    comp!({for r#self in 3..6 => 0});       //~ ERROR `self` cannot be a raw identifier
    comp!((for r#self in 3..6 => 0));       //~ ERROR `self` cannot be a raw identifier
    comp!({for r#self in 3..6 => 0, 0});    //~ ERROR `self` cannot be a raw identifier
}

/// Ranges that aren't allowed.
fn test_fail_range_forms(){
    // todo: add rest when we add iterators to their arms.
    comp!([for n in ..2 => 0; if true]);
    //~^ ERROR `std::ops::RangeTo<{integer}>` is not an iterator [E0277]
    comp!([for n in .. => 0; if true]);
    //~^ ERROR `std::ops::RangeFull` is not an iterator [E0277]
    comp!([for n in ..=3 => 9]);
    //~^ ERROR `std::ops::RangeToInclusive<{integer}>` is not an iterator [E0277]
}

/// Small sanity checks.
fn test_sanity(){
    comp!([20]);                       //~ ERROR Unable to parse expression.
    comp!([for n in 1..2 30]);         //~ ERROR Unable to parse expression.
    // todo: look into this.
    comp!([for n in n => n]);          //~ ERROR cannot find value `n` in this scope [E0425]

    comp!([for n in n]);               //~ ERROR Unable to parse expression.
    comp!([for n in 1..2 => 30; if]);  //~ ERROR Unable to parse expression.

    // Error for solo trailing greek question mark:
    comp!([for i in 1..2 => 0;]);      //~ ERROR Unable to parse expression.
    comp!((for i in 1..2 => 0;));      //~ ERROR Unable to parse expression.
    comp!({for i in 1..2 => 0;});      //~ ERROR Unable to parse expression.
    comp!({for i in 1..2 => 0, 0;});   //~ ERROR Unable to parse expression.
}

/// Leave main alone.
fn main() {}

