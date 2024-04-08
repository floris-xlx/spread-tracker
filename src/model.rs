//! Models, structs, enums and traits for the SpreadTracker library.
//!
//! ### Overview
//! - Symbol
//! - SymbolSpread
//! - HttpsUrl
//! - FromStr
//! - IsSymbol
//!
//! ### Structs
//! - Symbol
//! - SymbolSpread
//! - HttpsUrl
//!
//! ### Traits
//! - FromStr
//! - IsSymbol
//!
//!
//!
#![allow(dead_code)]


/// Struct that represents a currency pair.
/// Every major FX pair and few indices are included.
///
/// ### Example
///
/// ```
/// use forex_model::Symbol;
///
/// let symbol = Symbol::EuroUsd;
///
/// assert_eq!(symbol.get_symbol(), "EURUSD");
/// ```
///
/// ### Errors
/// provided symbol is not supported in this library
/// `
#[derive(Debug, Clone)]
pub enum Symbol {
    EuroUsd,
    AudCad,
    AudChf,
    AudNzd,
    AudUsd,
    CadChf,
    CadJpy,
    ChfJpy,
    EurAud,
    EurCad,
    EurChf,
    EurGbp,
    EurJpy,
    EurNzd,
    GbpAud,
    GbpCad,
    GbpChf,
    GbpJpy,
    GbpNzd,
    GbpUsd,
    NzdCad,
    NzdChf,
    NzdJpy,
    NzdUsd,
    UsdCad,
    UsdChf,
    XagUsd,
    XauAud,
    XauEur,
    XauUsd
}


/// Struct that represents a currency pair with the spread.
/// The spread is the difference between the ask and bid price.
///
/// ### Example
///
/// ```
/// use forex_model::{Symbol, SymbolSpread};
///
/// let symbol = Symbol::EuroUsd;
/// let spread = SymbolSpread::new(symbol, 0.0001, 1.1234, 1.1233);
///
/// assert_eq!(spread.get_symbol(), "EURUSD");
/// ```
///
#[derive(Debug, Clone)]
pub struct SymbolSpread {
    pub symbol: Symbol,
    pub spread: f64,
    pub ask: f64,
    pub bid: f64
}


/// Implementing a method for the SymbolSpread struct to get the symbol as a string.
///
/// ### Example
///
/// ```
/// use spread_tracker::model::{Symbol, SymbolSpread};
///
/// let symbol = Symbol::EuroUsd;
/// let spread = SymbolSpread::new(symbol, 0.0001, 1.1234, 1.1233);
///
/// assert_eq!(spread.get_symbol(), "EURUSD");
/// ```
impl SymbolSpread {
    pub fn new(symbol: Symbol, spread: f64, ask: f64, bid: f64) -> Self {
        Self {
            symbol,
            spread,
            ask,
            bid
        }
    }
}


/// Implementing a method for the Symbol enum to get the symbol as a string.
/// This method is used to get the symbol as a string to be used in the API request.
///
/// ### Example
///
/// ```
/// use spread_tracker::model::Symbol;
///
/// let symbol = Symbol::EuroUsd;
///
/// assert_eq!(symbol.get_symbol(), "EURUSD");
/// ```
pub trait FromStr {
    fn from_str(symbol: &str) -> Result<Self, String> where Self: Sized;
}


pub trait IsSymbol {
    fn is_valid_symbol(symbol: &str) -> bool;
}

impl IsSymbol for Symbol {
    fn is_valid_symbol(
        symbol: &str
    ) -> bool {

        matches!(symbol, "EURUSD" | "AUDCAD" | "AUDCHF" | "AUDNZD" | "AUDUSD" | "CADCHF" | "CADJPY" | "CHFJPY" | "EURAUD" | "EURCAD" | "EURCHF" | "EURGBP" | "EURJPY" | "EURNZD" | "GBPAUD" | "GBPCAD" | "GBPCHF" | "GBPJPY" | "GBPNZD" | "GBPUSD" | "NZDCAD" | "NZDCHF" | "NZDJPY" | "NZDUSD" | "USDCAD" | "USDCHF" | "XAGUSD" | "XAUAUD" | "XAUEUR" | "XAUUSD")
    }
}



impl FromStr for Symbol {
    /// Implementing a method for the Symbol enum to convert a string to a Symbol.
    /// This method is used to convert a string to a Symbol.
    ///
    /// ### Example
    ///
    /// ```
    /// use spread_tracker::model::Symbol;
    /// use spread_tracker::model::FromStr;
    ///
    /// let symbol = Symbol::from_str("EURUSD").unwrap();
    ///
    /// assert_eq!(symbol, Symbol::EuroUsd);
    ///
    /// ```
    ///
    /// ### Errors
    /// This method will return an error if the symbol is not supported in this library.
    ///
    fn from_str(
        symbol: &str
    ) -> Result<Self, String> {

        match symbol {
            "EURUSD" => Ok(Self::EuroUsd),
            "AUDCAD" => Ok(Self::AudCad),
            "AUDCHF" => Ok(Self::AudChf),
            "AUDNZD" => Ok(Self::AudNzd),
            "AUDUSD" => Ok(Self::AudUsd),
            "CADCHF" => Ok(Self::CadChf),
            "CADJPY" => Ok(Self::CadJpy),
            "CHFJPY" => Ok(Self::ChfJpy),
            "EURAUD" => Ok(Self::EurAud),
            "EURCAD" => Ok(Self::EurCad),
            "EURCHF" => Ok(Self::EurChf),
            "EURGBP" => Ok(Self::EurGbp),
            "EURJPY" => Ok(Self::EurJpy),
            "EURNZD" => Ok(Self::EurNzd),
            "GBPAUD" => Ok(Self::GbpAud),
            "GBPCAD" => Ok(Self::GbpCad),
            "GBPCHF" => Ok(Self::GbpChf),
            "GBPJPY" => Ok(Self::GbpJpy),
            "GBPNZD" => Ok(Self::GbpNzd),
            "GBPUSD" => Ok(Self::GbpUsd),
            "NZDCAD" => Ok(Self::NzdCad),
            "NZDCHF" => Ok(Self::NzdChf),
            "NZDJPY" => Ok(Self::NzdJpy),
            "NZDUSD" => Ok(Self::NzdUsd),
            "USDCAD" => Ok(Self::UsdCad),
            "USDCHF" => Ok(Self::UsdChf),
            "XAGUSD" => Ok(Self::XagUsd),
            "XAUAUD" => Ok(Self::XauAud),
            "XAUEUR" => Ok(Self::XauEur),
            "XAUUSD" => Ok(Self::XauUsd),
            _ => Err("Invalid symbol".to_string())
        }
    }
}

use std::fmt;

impl fmt::Display for Symbol {
    /// Implementing a method for the Symbol enum to format the symbol as a string.
    /// This method is used to format the symbol as a string.
    ///
    /// ### Example
    ///
    /// ```
    /// use spread_tracker::model::Symbol;
    ///
    /// let symbol = Symbol::EuroUsd;
    ///
    /// assert_eq!(format!("{}", symbol), "EURUSD");
    ///
    /// ```
    ///
    /// ### Errors
    /// `Invalid symbol` will be returned if the symbol is not supported in this library.
    ///
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol_str = match self {
            Self::EuroUsd => "EURUSD",
            Self::AudCad => "AUDCAD",
            Self::AudChf => "AUDCHF",
            Self::AudNzd => "AUDNZD",
            Self::AudUsd => "AUDUSD",
            Self::CadChf => "CADCHF",
            Self::CadJpy => "CADJPY",
            Self::ChfJpy => "CHFJPY",
            Self::EurAud => "EURAUD",
            Self::EurCad => "EURCAD",
            Self::EurChf => "EURCHF",
            Self::EurGbp => "EURGBP",
            Self::EurJpy => "EURJPY",
            Self::EurNzd => "EURNZD",
            Self::GbpAud => "GBPAUD",
            Self::GbpCad => "GBPCAD",
            Self::GbpChf => "GBPCHF",
            Self::GbpJpy => "GBPJPY",
            Self::GbpNzd => "GBPNZD",
            Self::GbpUsd => "GBPUSD",
            Self::NzdCad => "NZDCAD",
            Self::NzdChf => "NZDCHF",
            Self::NzdJpy => "NZDJPY",
            Self::NzdUsd => "NZDUSD",
            Self::UsdCad => "USDCAD",
            Self::UsdChf => "USDCHF",
            Self::XagUsd => "XAGUSD",
            Self::XauAud => "XAUAUD",
            Self::XauEur => "XAUEUR",
            Self::XauUsd => "XAUUSD",
        };
        write!(f, "{}", symbol_str)
    }
}





/// Struct that represents a https url.
///
/// ### Example
///
/// ```
/// use spread_tracker::model::HttpsUrl;
///
/// let https_url = HttpsUrl {
///    url: "https://www.alphavantage.co/".to_string()
/// };
///
/// assert_eq!(https_url.verify_url(), true);
/// ```
///
pub struct HttpsUrl {
    pub url: String
}


/// Implementing a method that will look at a https url and verify if the url is a valid url.
/// This method is used to verify if the url is a valid url.
///
/// ### Example
///
/// ```
/// use spread_tracker::model::SpreadBrokerUrl;
///
/// let spread_broker_url = SpreadBrokerUrl::new();
///
/// assert_eq!(spread_broker_url.vantage, "https://www.alphavantage.co/ ");
/// ```
///
/// ### Errors
///
/// This method will return an error if the url is not a valid url.
///
impl HttpsUrl {
    fn verify_url(&self) -> bool {
        self.url.starts_with("https://")
    }
}

