pub fn replace_spaces(string: &str, size: usize) -> String {
    // First pass
    // let words: Vec<&str> = string.trim().split(" ").collect();
    // return words.join("%20");

    let mut replaced = "".to_string();

    for (i, c) in string.chars().enumerate() {
        if c == ' ' {
            replaced.push_str("%20");
        } else {
            replaced.push(c);
        }

        if i + 1 == size {
            break;
        }
    }
    return replaced;
}

#[test]
fn example_replace_spaces() {
    assert_eq!(replace_spaces("Mr John Smith    ", 13), "Mr%20John%20Smith");
}

#[test]
fn multiple_spaces_are_replaced() {
    assert_eq!(replace_spaces("Julio  Nobrega    ", 14), "Julio%20%20Nobrega");
}
