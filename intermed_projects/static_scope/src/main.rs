mod static_local;

fn main() {
    do_static_local_scope();
}

fn do_static_local_scope() {
    println!("Static local scope!");
    static_local::do_it();
}
