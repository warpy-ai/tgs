pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
fn test_reverse_string() {
    assert_eq!(reverse_string("tgs"), "sgt");
    assert_eq!(reverse_string("hello"), "olleh");
    assert_eq!(reverse_string(""), "");
}
