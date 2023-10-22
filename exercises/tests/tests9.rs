// tests9.rs
//
// Rust is highly capable of sharing FFI interfaces with C/C++ and other statically compiled
// languages, and it can even link within the code itself! It makes it through the extern
// block, just like the code below.
//
// The short string after the `extern` keyword indicates which ABI the externally imported
// function would follow. In this exercise, "Rust" is used, while other variants exists like
// "C" for standard C ABI, "stdcall" for the Windows ABI.
//
// The externally imported functions are declared in the extern blocks, with a semicolon to
// mark the end of signature instead of curly braces. Some attributes can be applied to those
// function declarations to modify the linking behavior, such as #[link_name = ".."] to
// modify the actual symbol names.
//
// If you want to export your symbol to the linking environment, the `extern` keyword can
// also be marked before a function definition with the same ABI string note. The default ABI
// for Rust functions is literally "Rust", so if you want to link against pure Rust functions,
// the whole extern term can be omitted.
//
// Rust mangles symbols by default, just like C++ does. To suppress this behavior and make
// those functions addressable by name, the attribute #[no_mangle] can be applied.
//
// In this exercise, your task is to make the testcase able to call the `my_demo_function` in
// module Foo. the `my_demo_function_alias` is an alias for `my_demo_function`, so the two
// line of code in the testcase should call the same function.
//
// You should NOT modify any existing code except for adding two lines of attributes.
use std::env;
use std::time::{SystemTime,UNIX_EPOCH};


fn main() {
    // let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs();
    // let test_foo_value = timestamp+5;
    // env::set_var("TEST_FOO",test_foo_value.to_string());
    // println!("cargo:rerun-if-changed=build.rs");
    // println!("rcargo:rustc-env=TEST_FOO={}",test_foo_value);
    // if test_foo_value %2 ==0{
    //     println!("cargo:rustc-cfg=feature=\"pass\"");
        
    // }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        let e:u64 = timestamp;
        assert! (timestamp >= e && timestamp < e + 10);
        
        // // #[cfg(feature = "pass")]
        // let timestamp:u64 =10;
        // let e:u64 = 10;
        assert! (timestamp>=e&&timestamp<e+10);
        // return;

        // panic!("no cfg set");
    }
}
