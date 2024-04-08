//! # Cleaner Module

use std::collections::HashSet;



/// # Asynchronously removes banned characters from a given string.
///
/// ### Arguments
///
/// * `input` - A string slice that holds the input string.
/// * `banned_chars` - A slice of characters that are to be removed from the input string.
///
/// ### Returns
///
/// A new `String` with the banned characters removed.
///
/// ### Examples
///
/// ```
/// let cleaned_string = remove_banned_chars_async("Hello, World!", &[' ', ',']).await;
/// assert_eq!(cleaned_string, "HelloWorld!");
/// ```
pub fn remove_banned_chars(
    inputs: Vec<String>
) -> Vec<String> {
    let banned_chars: &[char] = &['\n'];
    let banned_patterns: &[&str] = &[ "class=", "\n"];

    inputs.into_iter().map(|input| {
        // remove banned characters
        let banned_set: HashSet<char> = banned_chars.iter().cloned().collect();
        let removed_chars: String = input.chars().filter(|c| !banned_set.contains(c)).collect();

        // remove banned patterns
        let mut result: String = removed_chars.to_string();
        for pattern in banned_patterns {
            result = result.replace(pattern, "");
        }

        result
    }).collect()
}
