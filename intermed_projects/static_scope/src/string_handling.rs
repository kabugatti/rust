pub fn do_it() {
    using_string_literals();
}

fn using_string_literals() {
    let s = "hello";
    let w: &'static str = "world";

    println!("s: {}, address: {:p}, size: {}", s, s.as_ptr(), s.len());
    println!("s & w: {} {}", s, w);
}