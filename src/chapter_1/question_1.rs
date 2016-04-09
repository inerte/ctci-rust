use std::collections::HashMap;

fn is_unique_characters(s: &str) -> bool {
    let mut previous_chars = HashMap::new();

    for c in s.chars() {
        if previous_chars.contains_key(&c) {
            return false;
        } else {
            previous_chars.insert(c, 1);
        }
    }

    return true;
}

#[test]
fn a_is_unique() {
    assert!(is_unique_characters("a"));
}

#[test]
#[should_panic]
fn aa_is_not_unique() {
    assert!(is_unique_characters("aa"));
}

#[test]
fn ab_is_unique() {
    assert!(is_unique_characters("ab"));
}

#[test]
fn alphabet_is_unique() {
    assert!(is_unique_characters("abcdefghijklmnopqrstuvwxyz"));
}

#[test]
fn alphabet_plus_a_is_not_unique() {
    assert_eq!(is_unique_characters("abcdefghijklmnopqrstuvwxyza"), false);
}
