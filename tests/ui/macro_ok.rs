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
    apply_pattern!(for n in 1..2 => 20);
    apply_pattern!(for n in &empty_vec => 20);
    comp!({for n in 1..2 => 10, 20});
    comp!({for n in &empty_vec => 10, 20});

    apply_pattern!(for n in 1..2 => n);
    apply_pattern!(for n in &empty_vec => n);
    comp!({for n in 1..2 => n, n});
    comp!({for n in &empty_vec => n, n});
}

fn test_range_forms(){
    // Need to implement them it first.
    todo!();
}


/// Leave main alone.
fn main() {}
