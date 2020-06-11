//! Module documentation.
//!
//! The form (I want) supported is
//!
//!  comp!([for ident in expr => expr; if expr])
//!
//! TODO: Expand.

// NOTE: Unfortunately, since procedural function-like macros cannot be used in an
// expression, we cannot use them. Instead, we'll need to use macro_rules!

/// Macro documentation.
/// TODO: Expand
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
    // Tuple comprehension.
    ((for $fid:ident in $($it:expr)+ => $($target:expr)+ $(; if $($cond:expr)+)?) $(, $tp:ident)?) => {{
        // fixme: Actually do things.
        ();
    }};
    // Otras
    () => {
        compile_error!("Empty expression.");
    };
    ($_:tt) => {
        // TODO: Improve report.
        compile_error!("Unable to parse expression.");
    }
}
