fn remove_or_add_one_character_equals_strings(largest: &str, smallest: &str) -> bool {
    let largest_string_len = largest.len();
    let mut start;
    let mut end_except_current_character;

    for i in 0..largest_string_len {
        start = &largest[0..i];
        end_except_current_character = &largest[i + 1..largest_string_len];

        if start.to_string() + &end_except_current_character == smallest {
            return true;
        }

        if smallest.starts_with(start) && smallest.ends_with(end_except_current_character) {
            return true;
        }
    }
    return false;
}

fn one_edit_away(string_a: &str, string_b: &str) -> bool {
    if string_a == string_b {
        return true;
    }

    let string_a_len: i32 = string_a.len() as i32;
    let string_b_len: i32 = string_b.len() as i32;

    // If the strings differ by more than one character, can not be one edit away
    if (string_a_len - string_b_len).abs() > 1 {
        return false;
    }

    if string_a_len >= string_b_len {
        return remove_or_add_one_character_equals_strings(string_a, string_b);
    } else {
        return remove_or_add_one_character_equals_strings(string_b, string_b);
    }
}

#[test]
fn example_1_one_edit_away() {
    assert!(one_edit_away("pale", "ple"));
}

#[test]
fn example_2_one_edit_away() {
    assert!(one_edit_away("pales", "pale"));
}

#[test]
fn example_3_one_edit_away() {
    assert!(one_edit_away("pale", "bale"));
}

#[test]
fn example_4_one_edit_away() {
    assert_eq!(one_edit_away("pale", "bake"), false);
}

#[test]
fn zero_edits_is_also_valid() {
    assert!(one_edit_away("pale", "pale"));
}

#[test]
fn strings_two_charaters_smaller_are_not_one_edit_away() {
    assert_eq!(one_edit_away("pa", "pale"), false);
}

#[test]
fn strings_two_charaters_larger_are_not_one_edit_away() {
    assert_eq!(one_edit_away("pale", "pa"), false);
}

#[test]
fn different_end_are_one_edit_away() {
    assert!(one_edit_away("pale", "pall"));
}

#[test]
fn middle_of_word_is_one_edit_away() {
    assert!(one_edit_away("julio", "juxio"));
}

#[test]
fn same_start_and_end_are_one_edit_away() {
    assert!(one_edit_away("aaabaaa", "aaacaaa"));
}
