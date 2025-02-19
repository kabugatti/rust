use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;

use std::thread::sleep;
use std::time::Duration;

fn main() {
    run_time();
    compile_time();
}

fn run_time() {
    static MESSAGE: &str = "Yellooow!";
    println!("Message: {}", MESSAGE);
}

fn compile_time() {
    static TIMESTAMP: Lazy<DateTime<Utc>> = Lazy::new(|| {
        println!("The current time is: {}", Utc::now().format("%T"));
        sleep(Duration::new(5, 0));
        Utc::now()
    });

    println!("The current time is: {}", TIMESTAMP.format("%T"));
}
