//! Module documentation.
//!
//! The form (I want) supported is
//!
//!  comp!([for ident in expr => expr; if expr])

// NOTE: Unfortunately, since procedural function-like macros cannot be used in an
// expression, we cannot use them. Instead, we'll need to use macro_rules!
// TODO: Add a lazy comp! (note: can we?)

/// Macro documentation.
#[macro_export]
macro_rules! comp {
    // Notes:
    // ======
    // 1. Need to also think about unpacking identifiers.

    // Vector comp
    ([for $fid:ident in $($it:expr)+ => $($target:expr)+ $(; if $($cond:expr)+)?]) => {
        for _todo in $($it)+{
            println!("{}", "gotcha vec!");
        }
    };
    // Set comp
    ({for $fid:ident in $($it:expr)+ => $($target:expr)+ $(; if $($cond:expr)+)?}) => {
        println!("{}", "gotcha set!");
    };
    // Tuple comp
    ((for $fid:ident in $($it:expr)+ => $($target:expr)+ $(; if $($cond:expr)+)?)) => {
        println!("{}", "gotcha tup!");
    };
    // Map comp
    ({for $fid:ident in $($it:expr)+ => $($key:expr)+, $($value:expr)+ $(; if $($cond:expr)+)?}) => {
        println!("{}", "gotcha tup!");
    };
    // Catch empty invocation.
    () => {
        compile_error!("Empty expression.");
    };
    // Catch everything else.
    ($_:tt) => {
        // TODO: How can I improve reporting of errors?
        compile_error!("Unable to parse expression.");
    }
}


/// Keep around (for a bit).
#[allow(unused_macros)]
macro_rules! comp_old {
    // Internal rules.
    // ===============
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
    // TODO: Match all cases of range. Can prob call match_range in here.
    (@match_iterator $begin:tt .. $end:tt) => {
        $begin..$end
    };
    (@match_iterator) => {
        compile_error!("An iterator must be supplied.");
    };
    // unpack_target: unpacks the target munch to its constituent parts.
    (@unpack_target $todo:tt) => {
        $todo
    };
    // match_range: match the different range formats.
    (@match_range $todo:tt) => {
        $todo
    };

    // Notes:
    // ------
    // - Double brackets around match arm are actually needed. W/o let isn't allowed (why?)
    // - $target and iterable are tt. expr can't be followed by things like 'for'.

    // Dict comprehension. todo: update based on vec
    ({$key:tt : $value:tt for $id:tt in $($iterable:tt)* $(if $cond:expr)?}) => {{
        let comp!(@need_id $id) = 30;
    }};

    // Set comp. todo: update based on vec
    ({$target:tt for $id:tt in $($iterable:tt)* $(if $cond:expr)?}) => {{
        let comp!(@need_id $id) = 30;
    }};

    // Vector comprehension.
    //
    // iterable uses TT bundler to grab as much as it can. then we try and
    // filter it with match_iterator (NOTE: Change it to use `+`
    // target should also use TT bundler, [1+2+...+n for _ in _] is parse error.
    ([$target:tt for $id:tt in $($iterable:tt)* $(if $($cond:expr)*)?]) => {{
        // TODO: Do we really need id? Since it won't get leaked, we can just ignore it.
        // TODO: But, can't remove let, we get complaints it isn't found in scope.
        let comp!(@need_id $id) = 30;
        comp!(@target_as_expr $target);

        let mut vec = Vec::new();
        for _todo in comp!(@match_iterator $($iterable)*){
            vec.push(_todo);
        }
        vec

    }};

    // Tuple comprehension. todo: update based on vec
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
