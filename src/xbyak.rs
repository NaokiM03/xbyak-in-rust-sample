extern "C" {
    fn _return_same_value(x: i32) -> i32;
}

pub fn return_same_value(x: i32) -> i32 {
    unsafe { _return_same_value(x) }
}

#[test]
fn test_return_same_value() {
    assert_eq!(return_same_value(42), 42);
}
