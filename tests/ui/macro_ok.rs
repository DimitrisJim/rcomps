// Check that successful cases pass ok.
// check-pass
extern crate rcomps;
use rcomps::comp;

fn ok_listcomp(){
    // TODO: This is ok, *for now*.
    comp!([20 for n in 1..2]);
    comp!((20 for n in 1..2));
    comp!({10: 20 for n in 1..2});
}

fn main() {}
