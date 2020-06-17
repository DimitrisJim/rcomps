// Check cases that compilefail.
//
// See https://rustc-dev-guide.rust-lang.org/tests/adding.html#error-annotations
// for annotating the snippets with the expected error.
// ignore-windows
extern crate rcomps;
use rcomps::comp;

/// Failures if improper iterator is supplied
fn test_fail_no_it() {
    // list comp
    comp!([for n in 1 => 20]);              //~ ERROR `{integer}` is not an iterator [E0277]
    comp!([for i in Vec => 0]);             //~ ERROR expected value, found struct `Vec` [E0423]
    comp!([for i in => 2]);                 //~ ERROR Unable to parse expression.
    comp!([for i in                => 0]);  //~ ERROR Unable to parse expression.
    comp!([for n in () => n]);              //~ ERROR `()` is not an iterator [E0277]

    // set comp
    comp!({for n in 1 => 20});              //~ ERROR `{integer}` is not an iterator [E0277]
    comp!({for i in Vec => 0});             //~ ERROR expected value, found struct `Vec` [E0423]
    comp!({for i in => 2});                 //~ ERROR Unable to parse expression.
    comp!({for i in                => 0});  //~ ERROR Unable to parse expression.
    comp!({for n in () => n});              //~ ERROR `()` is not an iterator [E0277]

    // map comp
    comp!({for n in 1 => 20, 0});              //~ ERROR `{integer}` is not an iterator [E0277]
    comp!({for i in Vec => 0, 0});             //~ ERROR expected value, found struct `Vec` [E0423]
    comp!({for i in => 2, 0});                 //~ ERROR Unable to parse expression.
    comp!({for i in                => 0, 0});  //~ ERROR Unable to parse expression.
    comp!({for n in () => n, n});              //~ ERROR `()` is not an iterator [E0277]
}

fn main(){}