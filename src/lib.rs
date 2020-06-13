//! Module documentation.
//!
//! Provides `comp!`, a `macro_rules` macro that supports comprehensions for the collections in
//! `std::collections`. All comprehensions follow a similar pattern:
//!
//!  `for identifier in iterable => expression; if condition`
//!
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
//!  `comp!([for ident in iterable => expression; if condition], collection_type)`
//!
//! where `collection_type` is one of `Vec`, `VecDeque`, `LinkedList` (`Vec` is used by default).
//!
//! ### Set
//!
//! For sets, curly brackets are used as the grouping operator, the syntax used is as follows:
//!
//!  `comp!({for ident in iterable => expression; if condition}, collection_type)`
//!
//! where `collection_type` is one of `HashSet` and `BTreeSet` (`HashSet` is used by default).
//!
//! ### Map
//!
//! For maps, curly brackets are used as the grouping operator, the syntax used is as follows:
//!
//!  `comp!({for ident in iterable => keyexpr, valueexpr; if condition}, collection_type)`
//!
//! where `collection_type` is one of `HashTree` and `BTreeSet` (`HashTree` is used by default).
//!


// note: Unfortunately, since procedural function-like macros cannot be used in an
// expression, we cannot use them. Instead, we'll need to use macro_rules!

/// Creates a collection using a comprehension.
///
/// `comp!` implements a comprehension like facility for succinct imperative creation of collections.
/// Four forms of the macro exist, each corresponding to a different type of collection:
///
/// ### Vector comprehensions
///
/// ```
/// # #[macro_use] extern crate rcomps;
/// # fn main(){
/// let evens = comp!([for i in 0..=10 => i; if (i % 2) == 0]);
/// assert_eq!(evens, vec![0, 2, 4, 6, 8, 10]);
/// # }
/// ```
///
/// ### Set comprehensions
///
/// ```
/// # #[macro_use] extern crate rcomps;
/// # fn main(){
/// let check = comp!({for i in &[1, 2, 3, 4, 3, 2, 1] => i});
/// # }
/// ```
///
/// ### Map comprehensions
///
/// ```
/// # #[macro_use] extern crate rcomps;
/// # fn main(){
/// let evens = comp!({for i in 0..=10 => i, i});
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

        // Iterate through $it and build vector.
        let mut res = Vec::new();
        for $fid in $($it)+{
            // Grab the condition.
            // note: Needs to be here, failed lookup if outside of loop (that uses $fid
            let cond = comp!(@match_if ($($($cond)+)?));
            if cond {
                res.push($($target)+);
            }
        }
        let mut r = <comp!(@match_type vec ($($tp)?))>::from_iter(res.drain(..));
        r
    }};
    // Set comprehension.
    ({for $fid:ident in $($it:expr)+ => $($target:expr)+ $(; if $($cond:expr)+)?} $(, $tp:ident)?) => {{
        // Bring in required names.
        use std::iter::FromIterator;
        use std::collections::HashSet;

        // Iterate through $it and build set.
        let mut res = HashSet::new();
        for $fid in $($it)+{
            // Grab the condition.
            // note: Needs to be here, failed lookup if outside of loop (that uses $fid
            let cond = comp!(@match_if ($($($cond)+)?));

            if cond {
                res.insert($($target)+);
            }
        }
        let mut r = <comp!(@match_type set ($($tp)?))>::from_iter(res.drain());
        r
    }};
    // Map comprehension.
    ({for $fid:ident in $($it:expr)+ => $($k:expr)+, $($v:expr)+ $(; if $($cond:expr)+)?} $(, $tp:ident)?) => {{
        // Bring in required names.
        use std::iter::FromIterator;
        use std::collections::HashMap;

        // Iterate through $it and build map.
        let mut res = HashMap::new();
        for $fid in $($it)+{
            // Grab the condition.
            // note: Needs to be here, failed lookup if outside of loop (that uses $fid
            let cond = comp!(@match_if ($($($cond)+)?));
            if cond {
                res.insert($($k)+, $($v)+);
            }
        }
        let mut r = <comp!(@match_type map ($($tp)?))>::from_iter(res.drain());
        r
    }};
    // Tuple comprehension; do not think this can actually be done.
    /*
    ((for $fid:ident in $($it:expr)+ => $($target:expr)+ $(; if $($cond:expr)+)?)) => {{
        // Iterate through $it and build intermediary vector.
        let mut res = Vec::new();
        for $fid in $($it)+{
            // Grab the condition.
            // note: Needs to be here, failed lookup if outside of loop (that uses $fid
            let cond = comp!(@match_if ($($($cond)+)?));
            if cond {
                res.push($($target)+);
            }
        }
        comp!(@build_tuple res [0; ])
    }};
    (@build_tuple ($v:expr) [($len:expr)*]) => {{
        let mut _ind = 0;
        let mut inc = || {_ind += 1; _ind}
        let t = $(
            $v[$len + inc()]
        )*
        t
    }}
    */
    // Otras
    () => {
        compile_error!("Empty expression.");
    };
    ($_:tt) => {
        // TODO: Improve report.
        compile_error!("Unable to parse expression.");
    }
}
