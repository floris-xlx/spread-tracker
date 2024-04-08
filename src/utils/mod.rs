//! This module contains all the utility functions like RegexFinder, Cleaner, Duplicates, etc.
//!
//! ### Functions, Structs, and Traits
//! - `remove_banned_chars` - Removes banned characters from a string.
//! - `remove_duplicates` - Removes duplicate values from a vector of strings.
//! - `vec_to_json` - Converts a vector of strings into a JSON string.
//! - `wrap_json_under_key` - Wraps a JSON object under a key.
//! - `extract_broker_name` - A function that extracts the broker name from a URL.
//! - `find_symbol_spread` - A function that finds the symbol spread from a given URL.
//!


pub mod save_to_json;
pub mod request_builder;
pub mod regex_finder;
pub mod cleaner;
pub mod duplicates;
pub mod format;