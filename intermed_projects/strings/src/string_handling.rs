pub fn do_it() {
    using_string_literals();
}

fn using_string_literals() {
    let s1 = "hello";
    let s2: &'static str = "world";

    println!("s1: {}, ptr: {:p}, len: {}", s1, s1.as_ptr(), s1.len());
    println!("s2: {}, ptr: {:p}, len: {}", s2, s2.as_ptr(), s2.len());
}