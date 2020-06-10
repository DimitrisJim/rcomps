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
    // TODO: Can somehow handle ty we pass to allow VecDeque, LinkedList, etc
    // TODO: Any *other* way to make `HashSet, HashMap` visible? `pub` on use doesn't seem to work.
    // TODO: Create macro to generate the similar stuff for vec, set, map?
    // TODO: We could also just use iterators throughout.
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
        // todo: ok, for now, HashSet is not available in prelude.
        use std::collections::HashSet;
        HashSet<_>
    };
    (@match_type set ($tp:ident)) => { $tp<_> };

    (@match_type map ()) => {
        // todo: ok, for now, HashMap is not available in prelude.
        use std::collections::HashMap;
        HashMap<_>
    };
    (@match_type map ($tp:ident)) => { $tp<_> };


    // Vector comprehension.
    ([for $fid:ident in $($it:expr)+ => $($target:expr)+ $(; if $($cond:expr)+)?] $(, $tp:ident)?) => {{
        use std::iter::FromIterator;
        // Grab the condition.
        let cond = comp!(@match_if ($($($cond)+)?));

        // Iterate through $it and build vector.
        let mut res = Vec::new();
        for $fid in $($it)+{
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

        // Grab the condition.
        let cond = comp!(@match_if ($($($cond)+)?));
        // Iterate through $it and build set.
        let mut res = HashSet::new();
        for $fid in $($it)+{
            if cond {
                res.insert($($target)+);
            }
        }
        res
        // todo: return based on tp
    }};
    // Map comprehension.
    ({for $fid:ident in $($it:expr)+ => $($k:expr)+, $($v:expr)+ $(; if $($cond:expr)+)?} $(, $tp:ident)?) => {{
        // Bring in required names.
        use std::iter::FromIterator;
        use std::collections::HashMap;

        // Grab the condition.
        let cond = comp!(@match_if ($($($cond)+)?));
        // Iterate through $it and build map.
        let mut res = HashMap::new();
        for $fid in $($it)+{
            if cond {
                res.insert($($k)+, $($v)+);
            }
        }
        res
        // todo: return based on tp
    }};
    // Tuple comprehension.
    ((for $fid:ident in $($it:expr)+ => $($target:expr)+ $(; if $($cond:expr)+)?) $(, $tp:ident)?) => {{
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
