fn is_rotation(s1: &str, s2: &str) -> bool {
    let len = s1.len();

    if len == s2.len() && len > 0 {
        let s1s1 = s1.to_string() + s1;
        return s1s1.contains(s2);
    }

    return false;
}

#[test]
fn example_is_rotation() {
    assert!(is_rotation("erbottlewat", "waterbottle"));
}
