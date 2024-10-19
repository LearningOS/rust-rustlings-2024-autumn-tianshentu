//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

use std::time::{SystemTime, UNIX_EPOCH};
use std::env;

fn main() {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    let your_command = format!("rustc-cfg=feature=\"pass\"");
    println!("cargo:{}", your_command);
}
