pub fn compress(string: &str) -> String {
    let mut character_count = 0;
    let mut previous_char = string.chars().nth(0).unwrap(); // Starts at first char
    let mut new_string_parts: Vec<String> = vec![];

    for c in string.chars() {
        if previous_char == c {
            character_count = character_count + 1;
        } else {
            new_string_parts.push(previous_char.to_string());
            new_string_parts.push(character_count.to_string());
            character_count = 1;
        }
        previous_char = c;
    }
    new_string_parts.push(previous_char.to_string());
    new_string_parts.push(character_count.to_string());

    let new_string = new_string_parts.join("");

    if string.len() <= new_string.len() {
        return string.to_string();
    } else {
        return new_string_parts.join("");
    }
}

#[test]
fn example_compress() {
    assert_eq!(compress("aabcccccaaa"), "a2b1c5a3");
}

#[test]
fn compress_should_return_original_string_when_not_smaller() {
    assert_eq!(compress("aa"), "aa");
}

#[test]
fn compress_should_return_original_string_when_not_smaller_with_larger_example() {
    assert_eq!(compress("aabbccddeeffgg"), "aabbccddeeffgg");
}

#[test]
fn compress_should_return_original_string_when_compression_generates_larger_string() {
    // if compress() had its way "abcdee" would be "a1b1c1d1e2"
    assert_eq!(compress("abcdee"), "abcdee");
}
