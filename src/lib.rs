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
    (@as_expr $e:expr) => {$e};
    // Notes:
    // ------
    // Double brackets around match arm are actually needed. W/o let isn't allowed (why?)
    // TODO: Dict comprehension
    ({$key:tt : $value:tt for $id:ident in $iterable:tt $(if $cond:expr)?}) => {{
        20
    }};

    // Set comp.
    ({$target:tt for $id:ident in $iterable:tt $(if $cond:expr)?}) => {{
        20
    }};

    // Vector comprehension.
    ([$target:tt for $id:ident in $iterable:tt $(if $cond:expr)?]) => {{
        println!("{}", comp!(@as_expr $iterable));
        println!("{}", comp!(@as_expr $target));
        let $id = 50;
        let mut vec = Vec::new();
        vec.push($target);
        vec.push($id);
        vec.push($iterable);
        vec
    }};

    // Tuple comprehension.
    {($target:tt for $id:ident in $iterable:tt $(if $cond:expr)?)} => {{
        // ok, this actually does work.
        let $id = 50;
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

