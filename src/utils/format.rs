//! # Formatting utilities for JSON and strings
//!
//! All intertwined conversion like datetime, Json, Vec<String> to JSON, etc.
//!
//!

use serde_json::Value;
use std::error::Error;
use regex::Regex;

use crate::model::{
    SymbolSpread,
    Symbol,
    FromStr
};

/// # Converts a vector of strings into a JSON string, where each string represents a `SymbolSpread`.
///
/// ### Arguments
///
/// * `data` - A vector of strings, each containing symbol, ask, bid, and spread separated by spaces.
///
/// ### Returns
///
/// A JSON string representation of the vector, or an error if the conversion fails.
///
/// ### Examples
///
/// ```
/// let input = vec!["AUDCHF 0.593 0.4821 1.4495".to_string()];
/// let json_output = vec_to_json(input).unwrap();
/// println!("{}", json_output);
/// ```
pub fn vec_to_json(
    data: Vec<String>
) -> Result<Value, Box<dyn Error>> {
    let mut symbol_spreads: Vec<SymbolSpread> = Vec::new();


    for entry in data.iter() {
        let parts: Vec<&str> = entry.split_whitespace().collect();

        if parts.len() != 4 {
            continue;
        }

        let symbol = match Symbol::from_str(parts[0]) {
            Ok(sym) => sym,
            Err(_) => return Err("Failed to parse symbol".into()),
        };

        let ask = match parts[1].parse::<f64>() {
            Ok(num) => num,
            Err(_) => return Err("Failed to parse ask price".into()),
        };

        let bid = match parts[2].parse::<f64>() {
            Ok(num) => num,
            Err(_) => return Err("Failed to parse bid price".into()),
        };

        let spread = match parts[3].parse::<f64>() {
            Ok(num) => num,
            Err(_) => return Err("Failed to parse spread".into()),
        };

        let symbol_spread = SymbolSpread {
            symbol,
            ask,
            bid,
            spread,
        };


        symbol_spreads.push(symbol_spread);
    }


    // serialize `symbol_spreads` manually since `SymbolSpread` does not implement `serde::ser::Serialize`
    let mut json_array: Vec<Value> = Vec::new();

    for spread in symbol_spreads {


        let obj: Value = serde_json::json!({
            "symbol": spread.symbol.to_string(),
            "ask": spread.ask,
            "bid": spread.bid,
            "spread": spread.spread,
        });

        json_array.push(obj);
    }
    let json_output: Value = serde_json::value::Value::Array(json_array);

    Ok(json_output)
}


/// # Wraps a JSON object under a specified key.
///
/// ### Arguments
///
/// * `json_object` - The JSON object to be wrapped.
/// * `key` - The key under which the JSON object will be wrapped.
///
/// ### Returns
///
/// A `serde_json::Value` representing the wrapped JSON object.
///
/// ### Examples
///
/// ```
/// let data = serde_json::json!({"name": "John Doe", "age": 30});
/// let wrapped_data = wrap_json_under_key(data, "person".to_string()).unwrap();
/// println!("{}", wrapped_data);
/// // Output: {"person": {"name": "John Doe", "age": 30}}
/// ```
pub fn wrap_json_under_key(
    json_object: Value,
    key: String
) -> Result<Value, Box<dyn Error>> {

    let wrapped_object: Value = serde_json::json!(
        { key: json_object }
    );

    Ok(wrapped_object)
}


/// # Extracts the broker name from a given URL.
///
/// ### Arguments
///
/// * `url` - The URL string from which the broker name is to be extracted.
///
/// ### Returns
///
/// A `String` representing the broker name extracted from the URL.
///
/// ### Examples
///
/// ```
/// let broker_url = "https://www.myfxbook.com/forex-broker-quotes/vantage/6052";
/// let broker_name = extract_broker_name(broker_url).unwrap();
/// println!("{}", broker_name);
/// // Output: vantage
/// ```
///
/// ### Errors
/// `Could not extract broker name from URL` will be returned if the broker name could not be extracted from the URL.
///
pub fn extract_broker_name(
    url: &str
) -> Result<String, &'static str> {
    let re: Regex = regex::Regex::new(r"/([^/]+)/\d+$").unwrap();

    if let Some(caps) = re.captures(url) {
        if let Some(matched) = caps.get(1) {
            return Ok(matched.as_str().to_string());
        }
    }
    Err("Could not extract broker name from URL")
}
