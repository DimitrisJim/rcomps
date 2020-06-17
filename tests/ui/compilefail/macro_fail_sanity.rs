// Check cases that compilefail.
//
// See https://rustc-dev-guide.rust-lang.org/tests/adding.html#error-annotations
// for annotating the snippets with the expected error.
// ignore-windows
extern crate rcomps;
use rcomps::comp;

/// Small sanity checks.
fn test_sanity(){
    comp!([20]);                       //~ ERROR Unable to parse expression.
    comp!([for n in 1..2 30]);         //~ ERROR Unable to parse expression.
    // todo: look into this.
    comp!([for n in n => n]);          //~ ERROR cannot find value `n` in this scope [E0425]

    comp!([for n in n]);               //~ ERROR Unable to parse expression.
    comp!([for n in 1..2 => 30; if]);  //~ ERROR Unable to parse expression.

    // Error for solo trailing greek question mark:
    // comp!([for i in 1..2 => 0;]);
    // comp!({for u in 1..2 => 0;});
    // comp!({for u in 1..2 => 0, 0;});
}

fn main() {}

