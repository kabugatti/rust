use std::sync::atomic::{AtomicI32, Ordering};

static GLOBAL_COUNT: AtomicI32 = AtomicI32::new(0);

pub fn do_it() {

    println!("\nIn static_mutable::do_it()");

    unsafe {
		f1();
		f1();
	}

	f2();
	f2();
}

unsafe fn f1() {
	
    static mut LOCAL_COUNT: i32 = 0;
	let mut x = 0;

	LOCAL_COUNT += 1;
	x += 1;
	GLOBAL_COUNT.fetch_add(1, Ordering::Relaxed); 
	
    println!("In f1, LOCAL_COUNT: {}, x: {}, GLOBAL_COUNT: {:?}", 
              LOCAL_COUNT, x, GLOBAL_COUNT)
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