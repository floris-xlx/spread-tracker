//! # Lightweight helper functions for removing duplicates from strings.
//!
use std::collections::HashSet;

/// # Removes duplicate values from each string in a vector. Values within strings are separated by spaces.
///
/// ### Arguments
///
/// * `inputs` - A vector of strings to process.
///
/// ### Returns
///
/// A new `Vec<String>` where each string has had duplicate values removed.
///
/// ### Examples
///
/// ```
/// let inputs = vec!["a b c a b".to_string(), "1 2 3 2 1".to_string()];
/// let outputs = remove_duplicates(inputs);
/// assert_eq!(outputs, vec!["a b c".to_string(), "1 2 3".to_string()]);
/// ```
///
pub fn remove_duplicates(
    inputs: Vec<String>
) -> Vec<String> {

    inputs.into_iter().map(|input| {
        let mut seen: HashSet<&str> = HashSet::new();

        let result: Vec<String> = input.split_whitespace()
            .filter_map(|word| {
                if seen.insert(word) {
                    Some(word.to_string())
                } else {
                    None
                }
            })
            .collect();
        result.join(" ")

    }).collect()
}
