// Check cases that compilefail.
//
// See https://rustc-dev-guide.rust-lang.org/tests/adding.html#error-annotations
// for annotating the snippets with the expected error.
// ignore-windows
extern crate rcomps;
use rcomps::comp;
use std::collections::*;

fn test_vec_wrong_types(){
    // vectors
    comp!([for i in 1..20 => i], HashSet);         //~ ERROR
    comp!([for i in 1..20 => i], BTreeSet);        //~ ERROR
    comp!([for i in 1..20 => i], HashMap);         //~ ERROR
    comp!([for i in 1..20 => i], BTreeMap);        //~ ERROR
}

fn test_set_wrong_types(){
    // sets
    comp!({for i in 1..20 => i}, Vec);            //~ ERROR
    comp!({for i in 1..20 => i}, VecDeque);       //~ ERROR
    comp!({for i in 1..20 => i}, HashMap);        //~ ERROR
    // comp!({for i in 1..20 => i}, BTreeMap);    // todo: no stderr generated.
}

fn test_map_wrong_types(){
    // maps
    comp!({for i in 1..20 => i, i}, HashSet);     //~ ERROR
    // comp!({for i in 1..20 => i, i}, BTreeSet); // todo: no stderr generated.
    // comp!({for i in 1..20 => i, i}, VecDeque); // todo: line that fails 
    comp!({for i in 1..20 => i, i}, Vec);         //~ ERROR
}

fn main(){}