## rcomps

What I would ideally like this to be able to do:

#### Vector comprehensions

Coarse code sample I'd like to support:

```rust
let v = comp!([for expr in <iterable> => expr; if expr]);
```


#### Mapping comprehensions

Coarse code sample I'd like to support:

```rust
let m = comp!({for expr in <iterable> => expr, expr; if expr});
```

Having `:` as the separator isn't required. `(key, value)` looks 
like it could also work.

#### Set comprehensions

Coarse code sample I'd like to support:

```rust
let s = comp!({expr for expr in <iterable> => expr; if expr});
```


#### Tuple comprehensions

Coarse code sample I'd like to support:

```rust
let t = comp!((for expr in <iterable> => expr; if expr));
```

### Notes:

Destroy this section eventually.

 - Can't use procedural function-like macros. Need to use 
   `macro_rules` (function-like macros don't support use 
   in expressions, which we need.)
 - `macro_rules` are visible after definition so tests can be 
   placed inside `lib.rs` as is usually done.
 - See personal Notes/Rust/macros.md.

### Licence

Licensed under either of:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.