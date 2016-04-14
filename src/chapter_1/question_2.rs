fn is_permutation(string_a: &str, string_b: &str, prefix: String) -> bool {
    let len = string_a.len();

    if string_b == string_a || string_b == prefix {
        return true;
    }

    if len == 0 {
        println!("prefix len 0 {}", prefix);
    } else {
        for i in 0..len {
            // Even tough .chars builds a list from the string,
            // .nth on it idiomatic Rust. As of 04/13, char_at
            // https://doc.rust-lang.org/std/string/struct.String.html#method.char_at
            // is unstable
            let char_at = string_a.chars().nth(i).unwrap();

            let rem = string_a[0..i].to_string() + &string_a[i + 1..len];

            let result = is_permutation(&rem, string_b, prefix.to_string() + &char_at.to_string());
            if result == true {
                return result;
            }
        }
    }

    return false;
}

#[test]
fn a_is_permutation_of_a() {
    assert!(is_permutation("a", "a", "".to_string()));
}

#[test]
fn ab_is_permutation_of_ba() {
    assert!(is_permutation("ab", "ba", "".to_string()));
}

#[test]
fn abcd_is_permutation_of_bacd() {
    assert!(is_permutation("abcd", "bacd", "".to_string()));
}

#[test]
fn abcdefg_is_permutation_of_gfedcba() {
    assert!(is_permutation("abcdefg", "gfedcba", "".to_string()));
}

#[test]
fn aabb_is_permutation_of_baba() {
    assert!(is_permutation("aabb", "baba", "".to_string()));
}
