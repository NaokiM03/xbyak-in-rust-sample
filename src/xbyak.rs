extern "C" {
    fn _return_same_value(x: i32) -> i32;
    fn _add(m: i32, n: i32) -> i32;
}

pub fn return_same_value(x: i32) -> i32 {
    unsafe { _return_same_value(x) }
}

pub fn add(m: i32, n: i32) -> i32 {
    unsafe { _add(m, n) }
}

#[test]
fn test_return_same_value() {
    assert_eq!(return_same_value(42), 42);
}

#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
}
