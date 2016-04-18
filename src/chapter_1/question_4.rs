// I confess I "cheated" this problem. I couldn't figure out what
// was being asked, so I went to the answer to understand the problem,
// read it twice, and of course couldn't get the solution out of my
// mind.
// After solving with the first solution, I realized it's one of those
// problems that greatly benefit from random knowledge about a shortcut.
// If you didn't know a palindrome couldn't have more than one odd
// character frequency, you would try to brute-force by computing all
// permutations. Still, there you go:
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
#[should_panic]
fn my_name_is_not_a_permutation_of_palindrome() {
    assert!(is_permutation_of_palindrome("Julio"));
}
