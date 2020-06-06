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
    (@it_as_expr $e:expr) => {$e};
    (@it_as_expr $other:tt) => {
            compile_error!("Iterable must be a valid expression or identifier.");
    };

    // Notes:
    // ------
    // - Double brackets around match arm are actually needed. W/o let isn't allowed (why?)
    // - $target and iterable are tt. expr can't be followed by things like 'for'.

    // TODO: Dict comprehension
    ({$key:tt : $value:tt for $id:ident in $iterable:tt $(if $cond:expr)?}) => {{
        20
    }};

    // Set comp.
    ({$target:tt for $id:tt in $iterable:tt $(if $cond:expr)?}) => {{
        let comp!(@need_id $id) = 30;
    }};

    // Vector comprehension.
    ([$target:tt for $id:tt in $iterable:tt $(if $cond:expr)?]) => {{
        // TODO: Do we really need id? Since it won't get leaked, we can just ignore it.
        let comp!(@need_id $id) = 30;
        comp!(@target_as_expr $target);
        let mut vec = Vec::new();
        vec.push($target);
        vec.push($id);
        vec.push($iterable);
        vec
    }};

    // Tuple comprehension.
    {($target:tt for $id:tt in $iterable:tt $(if $cond:expr)?)} => {{
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

