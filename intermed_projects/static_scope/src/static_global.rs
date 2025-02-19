use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;

// use std::thread::sleep;
// use std::time::Duration;

pub static GLOBAL_MESSAGE: &str = "Thomas Shelby";

pub static GLOBAL_TIMESTAMP: Lazy<DateTime<Utc>> = Lazy::new(|| {
    let now = Utc::now();
    println!("Current time: {}", now.format("%T"));
    now
});
