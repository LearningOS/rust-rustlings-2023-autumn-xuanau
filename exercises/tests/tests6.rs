// tests6.rs
//
// In this example we take a shallow dive into the Rust standard library's
// unsafe functions. Fix all the question marks and todos to make the test
// pass.
//
// Execute `rustlings hint tests6` or use the `hint` watch subcommand for a
// hint.



struct Foo {
    a: u128,
    b: Option<String>,
}

fn raw_pointer_to_box(address: usize) -> Box<Foo> {
    // address is a pointer that points to heap.
    let ptr = address as *mut Foo;
    // construct Box from this address, and modify Foo's b field to 
    // the string "hello"
    let mut boxed_foo = unsafe{
        Box::from_raw(ptr)
    };
    boxed_foo.b = Some("hello".to_owned());
    boxed_foo
    
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn test_success() {
        let nanos = Instant::now().duration_since(Instant::now().checked_sub(std::time::Duration::new(0, 1)).unwrap()).as_nanos();

        let data = Box::new(Foo{
            a: nanos,
            b: None
        });

        let ptr_1 = &data.a as *const u128 as usize;
        let ret = raw_pointer_to_box(Box::into_raw(data) as usize);

        let ptr_2 = &ret.a as *const u128 as usize;

        assert!(ptr_1 == ptr_2);
        assert!(ret.b == Some("hello".to_owned()));

    }
}
