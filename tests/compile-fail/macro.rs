extern crate rcomps;
use rcomps::comp;

fn main() {
    // ok, both work.
    comp!([]); //~ ERROR Unable to parse expression.
    comp!({
        for i in 1..5 {
            println!("{}", i);
        }
    }) //~^^^^ ERROR Unable to parse expression.
}
