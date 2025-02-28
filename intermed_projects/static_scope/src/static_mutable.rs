use std::sync::atomic::{AtomicI32, Ordering};

static GLOBAL_COUNT: AtomicI32 = AtomicI32::new(0);

pub fn do_it() {

    println!("\nIn static_mutable::do_it()");

	f1();
	f1();

	f2();
	f2();
}

fn f1() {
    static LOCAL_COUNT: AtomicI32 = AtomicI32::new(0);
	let mut x = 0;

	x += 1;
	LOCAL_COUNT.fetch_add(1, Ordering::Relaxed);
	GLOBAL_COUNT.fetch_add(1, Ordering::Relaxed); 
	
    println!("In f1, LOCAL_COUNT: {}, x: {}, GLOBAL_COUNT: {:?}", 
              LOCAL_COUNT.load(Ordering::Relaxed), x, GLOBAL_COUNT)
}

fn f2() {
	
	static LOCAL_COUNT: AtomicI32 = AtomicI32::new(0);
	let mut x = 0;

	LOCAL_COUNT.fetch_add(1, Ordering::Relaxed); 
	x += 1;
	GLOBAL_COUNT.fetch_add(1, Ordering::Relaxed); 

    println!("In f2, LOCAL_COUNT: {:?}, x: {}, GLOBAL_COUNT: {:?}", 
              LOCAL_COUNT, x, GLOBAL_COUNT)
}