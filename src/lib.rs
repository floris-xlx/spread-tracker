//! # Xylex spread tracker
//!
//! `spread_tracker` is a library for tracking the spread of various symbols in the forex market.
//!
//! Every forex broker has a different spread for each symbol. This library is used to track the spread of various symbols in the forex market.
//! Retrieve & scrape the spread of various symbols from different brokers.
//!
//!
//!
//! ### Features
//! - Track the spread of various symbols in the forex market.
//! - Retrieve & scrape the spread of various symbols from different brokers.
//! - Store the spread data in a database.
//! - Cache the spread data to minimize the number of requests to the broker.
//!
//! ### Granularity
//! - Query the spread of all symbols from all listed brokers.
//! - Query the spread of a specific symbol from all listed brokers.
//! - Query the spread of all symbols from a specific broker.
//! - Query the spread of a specific symbol from a specific broker.
//!
//!
//! ### Usage
//! Add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! spread_tracker = "0.1.0"
//! ```
//!
//! ### How to assign the correct Broker type
//! ```rust
//! use spread_tracker::config::{
//!     SpreadBrokerUrl,
//!     Brokers
//! };
//! ```
//!
//! ### How to get their spreads
//!
//!
//! <div class="warning">
//! This is currently strictly available for asynchronous context!
//! </div>
//!
//!
//! ```rust
//! use spread_tracker::config::SpreadBrokerUrl;
//! use spread_tracker::model::{
//!     Symbol,
//!     SymbolSpread
//! };
//!
//! let config: SpreadBrokerUrl = SpreadBrokerUrl::new();
//!
//! // In this example, we are tracking the spread of the symbols from the Vantage and EightCap brokers.
//! let brokers: Vec<Brokers> = vec![
//!     Brokers::Vantage,
//!     Brokers::EightCap,
//! ];
//!
//! // Get the spread of the symbols from the brokers
//! let spread: Value = SpreadTracker::get_spread(
//!     config,
//!     brokers
//! ).await.unwrap();
//!
//! println!("Spread: {:#?}", spread);
//!
//! Result:
//!
//! {
//!  "spread": {
//!    "eightcap": [
//!      {
//!        "ask": 2197.92,
//!        "bid": 2209.92,
//!        "spread": 12.0,
//!        "symbol": "XAUUSD"
//!      },
//!      {
//!        "ask": 2031.57,
//!        "bid": 2060.8973,
//!        "spread": 29.3273,
//!        "symbol": "XAUEUR"
//!      },
//!      {
//! ---------------- cut for brevity ----------------
//! ```
//! ### Return type
//! The return type is a `serde_json::Value` object, which is a JSON object.
//!
//! #### Structure
//! Object[spread]Vector[broker] -> Object[symbol, ask, bid, spread]
//!
//! * `spread` is the key for the spread data.
//! * `broker` is the key for the broker name, which will differ based on the broker.
//!
//! - Notes:
//! A Vector of objects is sometimes referred to as a list of objects. It is a collection of objects that are stored in no particular order.
//!
//! ### Configuration
//! The `config.yaml` is used to store the broker URLs for the spread tracking, and the `model.rs` is used to store the data model for the spread tracking. The `config.yaml` file should be in the following format:
//! ```yaml
//! BrokerSpreadUrls:
//!     Vantage: "https://www.vantagefx.com/trading-info/forex-market-hours/"
//!     MyFxBook: "https://www.myfxbook.com/forex-broker-quotes/vantage/6052"
//! ```
//!
//! Where `Vantage` and `MyFxBook` are the broker names and the URLs are the URLs of the brokers for spread tracking, derived from the MyFxBook website.
//!
//!
//!
//! ### Caching
//! wip
//! You can use the `db` module to store the spread data in a database.
//! Currently, the library relies on `supabase_rs` to store the data in a Supabase database.
//!
//! If you want to use a different database, you can implement the `Db` trait for your database.
//! Or simply call the `store_spread` method in the `model.rs` file to store the spread data in your database via a manual file like `.json`
//!
//!
//! ### Asynchronous vs Synchronous API
//! The main library is already asynchronous, so you can use it in an asynchronous context.
//! Sync API is **not** due to more testing being needed
//!
//!
//!
//! ### Database
//! wip
//! You can use the `db` module to store the spread data in a database.
//! Currently, the library relies on `supabase_rs` to store the data in a Supabase database.
//!
//! If you want to use a different database, you can implement the `Db` trait for your database.
//! Or simply call the `store_spread` method in the `model.rs` file to store the spread data in your database via a manual file like `.json`
//!
//! ### Errors
//!
//! <div class="warning">
//! Custom type errors are not yet implemented!
//! </div>
//!
//! * `url_invalid` will be returned if the URL is invalid.
//! * `url_not_reachable` will be returned if the URL is not reachable.
//! * `url_not_found` will be returned if the URL is not found.
//! * `invalid_body` will be returned if the body is invalid.
//! * `internal_error` will be returned if there is an internal error.
//! * `empty_body` will be returned if the body is empty.
//! * `could_not_extract_broker_name` will be returned if the broker name could not be extracted from the URL.
//! * `failed_to_open_config` will be returned if the config.yaml file is not found.
//! * `failed_to_read_yaml` will be returned if the file is not in the correct format.
//! * `failed_to_parse_symbol_spread` will be returned if the Vec<String> could not be parsed into a Vec<SymbolSpread>.
//! * `failed_to_parse_symbol` will be returned if the Symbol could not be parsed from the string.
//! * `failed_to_parse_ask_price` will be returned if the ask price could not be parsed from the string.
//! * `failed_to_parse_bid_price` will be returned if the bid price could not be parsed from the string.
//! * `failed_to_parse_spread` will be returned if the spread could not be parsed from the string.
//! * `could_not_retrieve_spread_data` will be returned if the spread data could not be retrieved from the broker URL.
//! * `failed_to_bind_json_object_to_key` will be returned if the JSON object could not be bound to the key.
//!
//!
//!
//! ### Modules
//! - `db`: This module is used to store the spread data in a database.
//! - `caching`: This module is used to cache the spread data to minimize the number of requests to the broker.
//! - `utils`: This module is used to save the spread data to a `.json` file.
//! - `config`: This module is used to load the configuration from the `config.yaml` file.
//! - `model`: This module is used to store the data model for the spread tracking.
//! - `errors`: This module is used to handle the errors in the library.
//!
#![doc(
    html_logo_url = "https://storage.googleapis.com/xylex_images/Svg/svgviewer-output%20(1).svg",
    html_favicon_url = "https://storage.googleapis.com/xylex_images/Svg/svgviewer-output%20(1).svg"
)]


#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(rustdoc::invalid_rust_codeblocks)]
#![allow(rustdoc::invalid_html_tags)]

// import the necessary modules into the hierarchy
pub mod caching;
pub mod db;
pub mod utils;
pub mod config;
pub mod model;
pub mod errors;




// import the necessary external crates into the hierarchy
use std::fs::File;
use std::io::Write;
use std::error::Error as StdError;
use reqwest::get;
use serde_json::{
    Value,
    Map
};


// import the necessary modules into the hierarchy
use crate::utils::regex_finder::find_symbol_spread;
use crate::model::{
    Symbol,
    SymbolSpread
};
use crate::config::{
    SpreadBrokerUrl,
    Brokers
};
use crate::utils::format::{
    wrap_json_under_key,
    vec_to_json,
    extract_broker_name
};




/// ### The `SpreadTracker` struct is used to track the spread of various symbols in the forex market.
///
/// This struct is used to track the spread of various symbols in the forex market.
///
pub struct SpreadTracker {
    spread_broker_url: SpreadBrokerUrl,
    spread: SymbolSpread
}


impl SpreadTracker {
    /// The `get_spreads` function is used to get the spread of various symbols from the broker URL.
    ///
    /// This function is used to get the spread of various symbols from the broker URL.
    ///
    /// ### Example
    ///
    /// ```
    /// use spread_tracker::SpreadTracker;
    /// use spread_tracker::config::SpreadBrokerUrl;
    ///
    /// let result = SpreadTracker::get_spreads(SpreadBrokerUrl::new());
    ///
    /// println!("Result: {:#?}", result);
    /// Output:
    /// {
    ///    "ask": Number(0.90451),
    ///    "bid": Number(1.37551),
    ///    "spread": Number(0.471),
    ///    "symbol": String("USDCHF"),
    /// });
    /// ```
    ///
    /// ### Errors
    /// `url_not_reachable` will be returned if the URL is not reachable.
    /// `url_not_found` will be returned if the URL is not found.
    /// `invalid_url` will be returned if the URL is invalid.
    /// `invalid_body` will be returned if the body is invalid.
    /// `internal_error` will be returned if there is an internal error.
    ///
    pub async fn get_spread(
        config: SpreadBrokerUrl,
        brokers: Vec<Brokers>
    ) -> Result<Value, Box<dyn StdError + Send + Sync + 'static>> {

        let mut all_broker_spreads: Map<String, Value> = serde_json::Map::new();

        for broker in brokers {
            let url: String = config.get_url(broker.clone());
            let url: &str = url.as_str();

            let name: String = extract_broker_name(url).unwrap_or_else(|_| broker.to_string());

            let spread_tracker: Result<String, Box<dyn StdError + Send + Sync>> = SpreadTracker::download_html_body(url).await;


            if let Ok(body) = spread_tracker {
                if let Ok(results) = SpreadTracker::regex_find_symbol_spread(&body).await {
                    if let Ok(json_output) = vec_to_json(results) {

                        // Wrap the JSON output under the broker's name
                        all_broker_spreads.insert(name, json_output);
                    }
                }
            }
        }

        // Wrap all broker spreads under the key "spread"
        let wrapped_all_broker_spreads: Value = serde_json::json!({ "spread": all_broker_spreads });

        Ok(wrapped_all_broker_spreads)
    }

    /// The `download_html_body` function is used to download the HTML body from the URL.
    /// This function is used to download the HTML body from the URL.
    ///
    /// ### Example
    ///
    /// ```
    /// use spread_tracker::SpreadTracker;
    ///
    /// let url = "https://www.vantagefx.com/trading-info/forex-market-hours/";
    ///
    /// let body = SpreadTracker::download_html_body(url);
    ///
    /// assert_eq!(body, "EURUSD 0.0001 1.1234 1.1233");
    /// ```
    pub async fn download_html_body(
        url: &str
    ) -> Result<String, Box<dyn StdError + Send + Sync + 'static>> {

        // Download the HTML body
        let response = get(url).await;
        if response.is_err() {
            return Err(Box::new(response.err().unwrap()));
        }
        let text = response.unwrap().text().await;
        if text.is_err() {
            return Err(Box::new(text.err().unwrap()));
        }
        let body: String = text.unwrap();

        let file_result = File::create("body.txt");
        if file_result.is_err() {
            return Err(Box::new(file_result.err().unwrap()));
        }
        let mut file: File = file_result.unwrap();
        let write_result = write!(file, "{}", body);
        if write_result.is_err() {
            return Err(Box::new(write_result.err().unwrap()));
        }

        Ok(body)
    }

    /// The `regex_find_symbol_spread` function is used to find the symbol spread in the HTML body.
    /// This function is used to find the symbol spread in the HTML body.
    ///
    /// ### Example
    ///
    /// ```
    /// use spread_tracker::SpreadTracker;
    ///
    /// let body = "<span></span> 1.1234 1.1233 0.0001Spread";
    ///
    /// let spread = SpreadTracker::regex_find_symbol_spread(& body);
    ///
    /// assert_eq!(spread, vec!["1.1234 1.1233 0.0001Spread"]);
    ///
    /// ```
    ///
    /// ### Errors
    /// `empty_body` will be returned if the body is empty.
    ///
    pub async fn regex_find_symbol_spread(
        body: &str
    ) -> Result<Vec<String>, Box<dyn StdError + Send + Sync + 'static>>{
        let results: Vec<String> = find_symbol_spread(body);


        Ok(results)
    }


}