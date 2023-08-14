use super::*;

#[test]
fn test_reverse_string() {
    assert_eq!(reverse_string("tgs"), "sgt");
    assert_eq!(reverse_string("hello"), "olleh");
    assert_eq!(reverse_string(""), "");
}
