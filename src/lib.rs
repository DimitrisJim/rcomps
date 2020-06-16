//! Module documentation.
//!
//! Provides `comp!`, a `macro_rules` macro that supports comprehensions for the collections in
//! `std::collections`. All comprehensions follow a similar pattern:
//!
//! ```text
//! for identifier in iterable => expression; if condition
//! ```
//! Disambiguating between resulting collections can be done with a combination of:
//!  - Grouping characters
//!  - A collection type annotation
//!
//! ### Grouping Characters
//!
//! By using different grouping characters different resulting collections can be created. Two
//! different bracket forms are used, namely square brackets `[]`, and curly brackets `{}`.
//!
//!  - Square brackets `[]` result in Sequences
//!  - Curly brackets `{}` result in Maps and Sets.
//!
//! By default, `[]` result in a `Vec` and `{}` for sets into a `HashSet` and `HashMap` for maps.
//!
//! ### Type Annotation
//!
//! Each of the grouping characters results in a default collection. If a different Sequence, Set
//! or Map is desired. This can be placed after the enclosing brackets preceeded by a comma
//! separator.
//!
//! ### Vector
//!
//! For vectors, square brackets are used as the grouping operator, the syntax used is as follows:
//!
//! ```text
//! comp!([for ident in iterable => expression; if condition], collection_type)
//! ```
//! where `collection_type` is one of `Vec`, `VecDeque`, `LinkedList` (`Vec` is used by default).
//!
//! ### Set
//!
//! For sets, curly brackets are used as the grouping operator, the syntax used is as follows:
//!
//! ```text
//! comp!({for ident in iterable => expression; if condition}, collection_type)
//! ```
//! where `collection_type` is one of `HashSet` and `BTreeSet` (`HashSet` is used by default).
//!
//! ### Map
//!
//! For maps, curly brackets are used as the grouping operator, the syntax used is as follows:
//!
//! ```text
//! comp!({for ident in iterable => keyexpr, valueexpr; if condition}, collection_type)
//! ```
//!
//! where `collection_type` is one of `HashTree` and `BTreeSet` (`HashTree` is used by default).
//!


// note: Unfortunately, since procedural function-like macros cannot be used in an
// expression, we cannot use them. Instead, we'll need to use macro_rules!

/// Creates a collection using a comprehension.
///
/// `comp!` implements a comprehension like facility for succinct imperative creation of collections.
/// Three forms of the macro exist, each corresponding to a different type of collection:
///
/// ### Vector comprehensions
///
/// Creates a vector using a comprehension. The syntax for creating a vector follows the
/// following pattern:
///
/// ```text
/// comp!([for ident in iterable => expression; if condition], collection_type)
/// ```
///
/// Where `collection_type` is an optional type among `Vec`, `VecDeque` and `LinkedList` that
/// controls the type of vector returned, by default, `Vec` is used. For example:
///
/// ```
/// # #[macro_use] extern crate rcomps;
/// # fn main(){
/// let evens = comp!([for i in 0..=10 => i; if (i % 2) == 0]);
/// assert_eq!(evens, vec![0, 2, 4, 6, 8, 10]);
/// # }
/// ```
///
/// A similar collection of values contained in a `VecDeque` can be created with:
///
/// ```no_run
/// # #[macro_use] extern crate rcomps;
/// # use std::collections::VecDeque;
/// # fn main() {
/// let evens = comp!([for i in 0..=10 => i; if (i % 2) == 0], VecDeque);
/// # }
/// ```
///
/// ### Set comprehensions
///
/// Creates a set using a comprehension. The syntax for creating a set follows the
/// following pattern:
///
/// ```text
/// comp!({for ident in iterable => expression; if condition}, set_type)
/// ```
///
/// Where `set_type` is an optional type among `HashSet` and `BTreeSet` that
/// controls the type of set returned. By default, `HashSet` is used. For example:
///
/// ```
/// # #[macro_use] extern crate rcomps;
/// # use std::collections::HashSet;
/// # use std::iter::FromIterator;
/// # fn main(){
/// let check = comp!({for i in vec![1, 2, 3, 4, 3, 2, 1] => i});
/// let s = HashSet::from_iter(vec![1, 2, 3, 4].drain(..));
/// assert_eq!(check, s);
/// # }
/// ```
///
/// A similar set of values contained in a `BTreeSet` can be created with:
///
/// ```no_run
/// # #[macro_use] extern crate rcomps;
/// # use std::collections::BTreeSet;
/// # fn main() {
/// let check = comp!({for i in vec![1, 2, 3, 4, 3, 2, 1] => i}, BTreeSet);
/// # }
/// ```
///
/// ### Map comprehensions
///
/// Creates a map using a comprehension. The syntax for creating a map follows the
/// following pattern:
///
/// ```text
/// comp!({for ident in iterable => key, value; if condition}, map_type)
/// ```
///
/// Where `map_type` is an optional type among `HashMap` and `BTreeMap` that
/// controls the type of map returned. By default, `HashMap` is used. For example:
///
/// ```
/// # #[macro_use] extern crate rcomps;
/// # use std::collections::HashMap;
/// # use std::iter::FromIterator;
/// # fn main(){
/// let check = comp!({for i in 0..=10 => i, -i});
/// let expected = HashMap::<i32, i32>::from_iter(
///  (0..=10).map(|i| {(i, -i)})
/// );
/// assert_eq!(check, expected);
/// # }
/// ```
///
/// A similar map of values contained in a `BTreeMap` can be created with:
///
/// ```no_run
/// # #[macro_use] extern crate rcomps;
/// # use std::collections::BTreeMap;
/// # fn main() {
/// let check = comp!({for i in 0..=10 => i, -i}, BTreeMap);
/// # }
/// ```
///
#[macro_export]
macro_rules! comp {
    // note: Any *other* way to make `HashSet, HashMap` visible? `pub` on use doesn't seem to work.
    // note: Create macro to generate the similar stuff for vec, set, map?
    // note: We could also just use iterators throughout.
    // fixme: Any type can be passed as tp, need to restrain these.

    // Internal rules:
    // ===============
    //
    // match_if: Returns true if no condition was passed else the condition.
    (@match_if ()) => { true };
    (@match_if $($cond:expr)+) => { $($cond)+ };

    // match_type <type>: Matches ident. If one isn't provided, defaults are
    // used. If they are, pass them through.
    (@match_type vec ()) => { Vec<_> };
    (@match_type vec ($tp:ident)) => { $tp<_> };

    (@match_type set ()) => {
        // note: HashSet is brought into scope in the rule.
        HashSet<_>
    };
    (@match_type set ($tp:ident)) => { $tp<_> };

    (@match_type map ()) => {
        // note: HashMap is brought into scope in the rule.
        HashMap<_, _>
    };
    (@match_type map ($tp:ident)) => { $tp<_, _> };

    // Vector comprehension.
    ([for $fid:ident in $($it:expr)+ => $($target:expr)+ $(; if $($cond:expr)+)?] $(, $tp:ident)?) => {{
        use std::iter::FromIterator;
        use std::ops::Index;

        fn _seq<T> (_: &T)
        where
            T: Index<usize> {}

        // Iterate through $it and build vector.
        let mut res = Vec::new();
        for $fid in $($it)+{
            // Grab the condition.
            let cond = comp!(@match_if ($($($cond)+)?));
            if cond {
                res.push($($target)+);
            }
        }
        let mut r = <comp!(@match_type vec ($($tp)?))>::from_iter(res.drain(..));
        _seq(&r);
        r
    }};
    // Set comprehension.
    ({for $fid:ident in $($it:expr)+ => $($target:expr)+ $(; if $($cond:expr)+)?} $(, $tp:ident)?) => {{
        // Need to bring in the default.
        use std::collections::HashSet;
        use std::ops::{BitAnd, BitOr, BitXor};

        fn _st<T> (_: &T)
        where
            T: BitAnd + BitOr + BitXor {}


        // Iterate through $it and build set.
        let mut res = <comp!(@match_type set ($($tp)?))>::new();
        for $fid in $($it)+{
            // Grab the condition.
            let cond = comp!(@match_if ($($($cond)+)?));

            if cond {
                res.insert($($target)+);
            }
        }
        _st(&&res);
        res
        // let mut r = <comp!(@match_type set ($($tp)?))>::from_iter(res.drain());
        // r
    }};
    // Map comprehension.
    ({for $fid:ident in $($it:expr)+ => $($k:expr)+, $($v:expr)+ $(; if $($cond:expr)+)?} $(, $tp:ident)?) => {{
        // Need to bring in the default.
        use std::collections::HashMap;
        use std::ops::Index;

        fn _mt<'a, T, K: 'a> (_: &T)
        where
            T: Index<&'a K> {}

        // Iterate through $it and build map.
        let mut res = <comp!(@match_type map ($($tp)?))>::new();
        for $fid in $($it)+{
            // Grab the condition.
            let cond = comp!(@match_if ($($($cond)+)?));
            if cond {
                res.insert($($k)+, $($v)+);
            }
        }
        _mt::<_, _>(&res);
        res
    }};
    // Otras:
    () => {
        compile_error!("Empty expression.");
    };
    ($_:tt) => {
        compile_error!("Unable to parse expression.");
    }
}
