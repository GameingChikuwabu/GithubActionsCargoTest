
#[no_mangle]
extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}


#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3);
}