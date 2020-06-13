## rcomps

A `macro_rules` macro to populate collection from a comprehension. 

#### Vector comprehensions

Enclosed in brackets, by default generates a `Vec` with the elements 
from the comprehension. The macro call:

```rust
let v = comp!([for ident in iterable => expr; if condexpr], collection_type);
```

is roughly equivalent to:

```rust 
vector = Vec::new();
for ident in iterable{
    if condexpr {
        vector.push(expr);
    }
}
type::from_iter(vector.iter());
```

Collection type can be either one of `Vec`, `VecDeque` and `LinkedList`.

Note: This is overview, certain optimizations (e.g preallocation based on iterator size) could be 
performed.    
    
#### Mapping comprehensions

Enclosed in curly brackets, by default generates a `HashMap` with the elements 
from the comprehension. The macro call:

```rust
let m = comp!({for ident in iterable => keyexpr, valuexpr; if condexpr}, collection_type);
```

is roughly equivalent to:

```rust 
hm = HashMap::new();
for ident in iterable{
    if condexpr {
        hm.insert(keyexpr, valueexpr);
    }
}
type::from_iter(hm.iter());
```

Collection type can be either one of `HashMap` or `BTreeMap`.

Note: This is overview, certain optimizations (e.g preallocation based on iterator size) could be 
performed.        

#### Set comprehensions

Enclosed in curly brackets, by default generates a `HashSet` with the elements 
from the comprehension. Distinguished from map case by requiring a single `expr` (which should 
be enclosed in parentheses if a tuple is needed). The macro call:

```rust
let s = comp!({for ident in iterable => expr; if condexpr}, collection_type);
```

is roughly equivalent to:

```rust 
hm = HashSet::new();
for ident in iterable{
    if condexpr {
        hm.insert(expr);
    }
}
type::from_iter(hm.iter());
```

Collection type can be either one of `HashSet` or `BTreeSet`.

Note: This is overview, certain optimizations (e.g preallocation based on iterator size) could be 
performed.        

### Notes:

Destroy this section eventually.

 - Currently developed, not good to push to cargo.
 - Tuples: can we statically define it?
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