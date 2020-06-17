// Check cases that compilefail.
//
// See https://rustc-dev-guide.rust-lang.org/tests/adding.html#error-annotations
// for annotating the snippets with the expected error.
// ignore-windows
extern crate rcomps;
use rcomps::comp;

/// Ranges that aren't allowed.
fn test_fail_range_forms(){
    comp!([for n in ..2 => 0; if true]);
    //~^ ERROR `std::ops::RangeTo<{integer}>` is not an iterator [E0277]
    comp!([for n in .. => 0; if true]);
    //~^ ERROR `std::ops::RangeFull` is not an iterator [E0277]
    comp!([for n in ..=3 => 9]);
    //~^ ERROR `std::ops::RangeToInclusive<{integer}>` is not an iterator [E0277]

    comp!({for n in ..2 => 0; if true});
    //~^ ERROR `std::ops::RangeTo<{integer}>` is not an iterator [E0277]
    comp!({for n in .. => 0; if true});
    //~^ ERROR `std::ops::RangeFull` is not an iterator [E0277]
    comp!({for n in ..=3 => 9});
    //~^ ERROR `std::ops::RangeToInclusive<{integer}>` is not an iterator [E0277]

    comp!({for n in ..2 => 0, 0; if true});
    //~^ ERROR `std::ops::RangeTo<{integer}>` is not an iterator [E0277]
    comp!({for n in .. => 0, 0; if true});
    //~^ ERROR `std::ops::RangeFull` is not an iterator [E0277]
    // todo: this doesn't compilefail with E0277 for some odd reason.
    // comp!({for n in ..=3 => 9, 9});
}

fn main(){}