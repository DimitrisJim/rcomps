// Check that successful cases compile ok, don't check result.
// check-pass
extern crate rcomps;
use rcomps::comp;

/// Ok. Applies pattern to comp!. Can't use map.
#[macro_use]
macro_rules! apply_pattern {
    ($($pat:tt)*) => {
        comp!(($($pat)*));  // vec
        comp!([$($pat)*]);  // tup
        comp!({$($pat)*});  // set
    }
}

fn ok_basiccomp(){
    // Using a range.
    let empty_vec: Vec<i32> = vec![];
    apply_pattern!(20 for n in 1..2);
    apply_pattern!(20 for n in &empty_vec);
    comp!({10: 20 for n in 1..2});
    comp!([20 for n in &empty_vec]);

    apply_pattern!(n for n in 1..2);
    apply_pattern!(n for n in &empty_vec);
    comp!({n:n for n in 1..2});
    comp!({n:n for n in &empty_vec});
}

fn test_range_forms(){
    // Need to implement them it first.
    todo!();
}

/// Leave main alone.
fn main() {}
