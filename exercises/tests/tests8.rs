// tests8.rs
//
// This execrise shares `build.rs` with the previous exercise.
// You need to add some code to `build.rs` to make both this exercise and
// the previous one work.
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.
use std::env;
use std::time::{SystemTime,UNIX_EPOCH};

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        let e:u64 = timestamp;
        assert! (timestamp >= e && timestamp < e + 10);      
        assert! (timestamp>=e&&timestamp<e+10);

    }
}
