use std::collections::HashMap;

fn char_frequency(string: String) -> HashMap<char, usize> {
    let mut frequency = HashMap::new();

    for c in string.chars() {
        if !c.is_whitespace() {
            let counter = frequency.entry(c).or_insert(0);
            *counter += 1;
        }
    }

    return frequency;
}

pub fn is_permutation_of_palindrome(string: &str) -> bool {
    let string = string.to_lowercase();
    let char_frequency = char_frequency(string);

    let mut odd_found = false;

    for frequency in char_frequency.values() {
        if frequency % 2 == 1 {
            if odd_found {
                return false;
            }
            odd_found = true;
        }
    }

    return true;
}

#[test]
fn example_is_permutation_of_palindrome() {
    assert!(is_permutation_of_palindrome("Tact Coa"));
}

#[test]
fn one_character_is_permutation_of_palindrome() {
    assert!(is_permutation_of_palindrome("a"));
}

#[test]
fn a_palindrome_is_a_permutation_of_palindrome() {
    assert!(is_permutation_of_palindrome("abba"));
}

#[test]
fn my_name_is_not_a_permutation_of_palindrome() {
    assert_eq!(is_permutation_of_palindrome("Julio"), false);
}
