//! # Regex finder & parser

#![allow(clippy::let_and_return)]

use crate::model::{
    Symbol,
    SymbolSpread,
    IsSymbol
};
use regex::Regex;
use crate::utils::cleaner::remove_banned_chars;
use crate::utils::duplicates::remove_duplicates;

use tracing::{
    info,
    warn,
    error
};

/// # Find the symbol spread in the HTML body.
///
///
/// ### Example
/// ```
/// use spread_tracker::utils::regex_finder::find_symbol_spread;
///
/// let body = "<span></span> 1.1234 1.1233 0.0001Spread";
///
/// let spread = find_symbol_spread(& body);
///
/// assert_eq!(spread, vec!["1.1234 1.1233 0.0001Spread"]);
///
/// ```
///
/// ### Errors
/// `couldnt retrieve data` if the body is empty.
/// `couldnt find spread` if the spread is not found in the body.
/// `couldnt find symbol` if the symbol is not found in the body.
/// `couldnt find ask` if the ask is not found in the body.
/// `couldnt find bid` if the bid is not found in the body.
/// `couldnt find spread` if the spread is not found in the body.
///
///
pub fn find_symbol_spread(
    body: &str
) -> Vec<String> {
    let re: Regex = Regex::new(r"\dSpread").unwrap(); // first finds the line before the spread
    let span_re: Regex = Regex::new(r"<span></span>").unwrap(); // till where it looks up for the spread
    let mut lines_with_spread: Vec<String> = Vec::new();
    let mut capture: bool = false;
    let mut captured_lines: Vec<String> = Vec::new();
    let mut previous_line: String = String::new();

    for line in body.lines().rev() {
        if re.is_match(line) {
            capture = true;
        }
        if capture {
            captured_lines.push(previous_line.clone()); // one line forward (previous line in this reversed iteration)

            captured_lines.push(line.to_string());
            if span_re.is_match(line) {
                capture = false;
                lines_with_spread.push(captured_lines.iter().rev().cloned().collect::<Vec<String>>().join("\n"));
                captured_lines.clear();
            }
        }
        previous_line = line.to_string(); // track of the previous line
    }


    let valid_symbols: Vec<String> = lines_with_spread.into_iter().filter(|line| {
        let words: Vec<&str> = line.split_whitespace().collect();
        words.iter().any(|&word| Symbol::is_valid_symbol(word))
    }).collect();



    let numbers_and_symbols: Vec<String> = valid_symbols.into_iter().map(|line| {
        let words: Vec<&str> = line.split_whitespace().filter(|&word| {
            Symbol::is_valid_symbol(word) || word.parse::<f32>().is_ok()
        }).collect();
        words.join(" ")
    }).collect();


    // clean duplicates
    let cleaned_lines: Vec<String> = remove_duplicates(numbers_and_symbols);

    info!("Found symbol spread in the body");

    cleaned_lines
}