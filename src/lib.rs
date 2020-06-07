//! Module documentation.
//!
//! The form (I want) supported is
//!
//!  comp!([expr for ident in expr if expr])

// NOTE: Unfortunately, since procedural function-like macros cannot be used in an
// expression, we cannot use them. Instead, we'll need to use macro_rules!

/// Macro documentation.
#[macro_export]
macro_rules! comp {
    // Internal rules.
    // ---------------
    // TODO: Any way we can also report what we got?
    //
    // need_id: Catch if an id isn't supplied for for clause.
    (@need_id $id:ident) => {$id};
    (@need_id $other:tt) => {
        compile_error!("An identifier is required in the 'for ident in it' clause.");
    };

    // as_expr: Requiring an expression for target and iterable.
    (@target_as_expr $e:expr) => {$e};
    (@target_as_expr $other:tt) => {
            compile_error!("Target must be a valid expression.");
    };
    // match_iterator: Match either single expression denoting an
    // iterator or a x..y range.
    (@match_iterator $it:expr) => {$it};
    // TODO: Match all cases of range.
    (@match_iterator $begin:tt .. $end:tt) => {
        $begin..$end
    };

    // Notes:
    // ------
    // - Double brackets around match arm are actually needed. W/o let isn't allowed (why?)
    // - $target and iterable are tt. expr can't be followed by things like 'for'.

    // Dict comprehension
    ({$key:tt : $value:tt for $id:tt in $($iterable:tt)* $(if $cond:expr)?}) => {{
        let comp!(@need_id $id) = 30;
    }};

    // Set comp.
    ({$target:tt for $id:tt in $($iterable:tt)* $(if $cond:expr)?}) => {{
        let comp!(@need_id $id) = 30;
    }};

    // Vector comprehension.
    //
    // iterable uses TT bundler to grab as much as it can. then we try and
    // filter it with match_iterator
    ([$target:tt for $id:tt in $($iterable:tt)* $(if $cond:expr)?]) => {{
        // TODO: Do we really need id? Since it won't get leaked, we can just ignore it.
        let comp!(@need_id $id) = 30;
        comp!(@target_as_expr $target);

        for _todo in comp!(@match_iterator $($iterable)*){
            println!("Iterating");
        }
        let mut vec = Vec::new();
        vec.push($target);
        vec.push($id);
        vec
    }};

    // Tuple comprehension.
    {($target:tt for $id:tt in $($iterable:tt)* $(if $cond:expr)?)} => {{
        let comp!(@need_id $id) = 30;
        // NOTE: We can use $( $capturname, )* to build
        // a tuple.
        let tup = ($id,);

        tup
    }};
    ($_:tt) => {
        // TODO: Improve report.
        compile_error!("Unable to parse expression.");
    }
}

