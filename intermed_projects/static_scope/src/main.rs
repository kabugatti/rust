#![allow(dead_code)]

mod static_local;
mod static_global;

use static_global::{GLOBAL_MESSAGE, GLOBAL_TIMESTAMP};

fn main() {
    // do_static_local_scope();
    do_static_global_scope();
}

fn do_static_local_scope() {
    println!("Static local scope!");
    static_local::do_it();
}

fn do_static_global_scope() {
    println!("GLOBAL MESSAGE: {}, GLOBAL TIMESTAMP: {}", GLOBAL_MESSAGE, GLOBAL_TIMESTAMP.format("%T"));
}