//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
use std::env;
use std::time::{SystemTime,UNIX_EPOCH};

fn main() {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
    let test_foo_value = timestamp+5;
    env::set_var("TEST_FOO",test_foo_value.to_string());
    println!("cargo:return-if-changed=build.rs");
    println!("rcargo:zustc-env=TEST_FOO={}",test_foo_value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        let s = std::env::var("TEST_FOO").unwrap();
        let e:u64 = s.parse().unwrap();
        assert! (timestamp >= e && timestamp < e + 10);
    }
}


// use std::env;
// use std::time::{SystemTime,UNIX_EPOCH};

// fn main() {
//     // In tests7, we should set up an environment variable
//     // called `TEST_FOO`. Print in the standard output to let
//     // Cargo do it.
//     let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
//     let test_foo_value = timestamp+5;
//     env::set_var("TEST_FOO",test_foo_value.to_string());
//     println!("cargo:return-if-changed=build.rs");
//     println!("rcargo:zustc-env=TEST_FOO={}",test_foo_value);
//     );

//     // In tests8, we should enable "pass" feature to make the
//     // testcase return early. Fill in the command to tell
//     // Cargo about that.
//     use std::env;
//     use std::time::{SystemTime,UNIX_EPOCH};
//     let tempstamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
//     let test_foo_value = timestamp+5;
//     env::set_var("TEST_FOO",test_foo_value.to_string());
//     println!("cargo:return-if-changed=build.rs");
//     println!("rcargo:zustc-env=TEST_FOO={}",test_foo_value);
//     if test_foo_value %2 ==0{
//         println!("cargo:rustc-cfg=feature=\"pass\"");
// }
