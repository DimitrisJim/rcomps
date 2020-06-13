// Check that successful cases compile ok, don't check result.
// check-pass
extern crate rcomps;
use rcomps::comp;

/// Ok. Applies pattern to comp!. Can't use map.
/// (NOTE: Cannot reuse in macro_fail. error comments don't jive well with it.)
#[macro_use]
macro_rules! apply_pattern {
    ($($pat:tt)*) => {
        // comp!(($($pat)*));  // note: uncomment if I find a way to implement tuple.
        comp!([$($pat)*]);  // vec
        comp!({$($pat)*});  // set
    }
}

/// Basic checks.
fn test_ok_basiccomp(){
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

/// Iterator valid
fn test_ok_it(){
    // todo: merge with test_ok_range_forms?
    let cl = || {vec![1, 2, 3]};
    apply_pattern!(for i in cl() => 0);
    comp!({for i in cl() => 0, 0});
}

/// Test that the if clause can capture expressions correctly.
fn test_ok_if_clause(){
    // Ok, basic.
    apply_pattern!(for n in 1..2 => 0; if true);
    comp!({for i in 1..4 => 0, 0; if true});
    // Ok, if has many expressions.
    apply_pattern!(for n in 1..2 => 0; if !true && !false);
    comp!({for i in 1..4 => 0, 0; if !true && !false});
}

/// Test that the different forms for ranges are caught ok.
fn test_ok_range_forms(){
    // Ok, Range.
    apply_pattern!(for i in 1..2 => 0);
    comp!({for i in 1..2 => 0, 0});
    // Ok, RangeFrom is accepted (todo: is there much point in it?)
    apply_pattern!(for i in 1.. => 0);
    comp!({for i in 1.. => 0, 0});
    // Ok, Range inclusive.
    apply_pattern!(for i in 0..=5 => 0);
    comp!({for i in 0..=5 => 0, 0});
}

/// Ok identifiers.
fn test_ok_idents(){
    apply_pattern!(for _my_not_so_intricate_but_long_identifier in 1..2 => 0);
    comp!({for _my_not_so_intricate_but_long_identifier in 1..2 => 0, 0});

    // try raw identifier.
    apply_pattern!(for r#try in 3..6 => 0);
    comp!({for r#try in 3..6 => 0, 0});
}

/// Leave main alone.
fn main() {}
