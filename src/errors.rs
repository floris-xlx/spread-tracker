//! # ErrorsSpread
//! The following errors are returned by the functions in this module:

// url is not valid, doesnt start with https
// website is unreachable
// config.yaml file is not found
// config.yaml file is not in the correct format
// no urls found in the config.yaml file
// some other error

// failed to parse the Vec<String> into a Vec<SymbolSpread>
// failed to parse the Symbol from the string
// failed to parse the ask price from the string
// failed to parse the bid price from the string
// failed to parse the spread from the string
// couldn't retrieve spread data from the broker URL
// failed to bind the json object to the key

#![allow(clippy::new_ret_no_self)]

use std::fmt::{
    self,
    Display,
    Formatter,
    Result
};
use std::error::Error as StdError; // we import as StdError to avoid conflicts with our Error enum




#[derive(Debug, Clone, Copy)]
pub enum ErrorsSpread {
    UrlInvalid,
    UrlNotReachable,
    UrlNotFound,
    InvalidBody,
    InternalError,
    EmptyBody,
    CouldNotExtractBrokerName,
    FailedToOpenConfig,
    FailedToReadYaml,
    FailedToParseSymbolSpread,
    FailedToParseSymbol,
    FailedToParseAskPrice,
    FailedToParseBidPrice,
    FailedToParseSpread,
    CouldNotRetrieveSpreadData,
    FailedToBindJsonObjectToKey,
}


impl Display for ErrorsSpread {
    fn fmt(
        &self,
        f: &mut Formatter
    ) -> Result {

        match self {
            ErrorsSpread::UrlInvalid => write!(f, "url is not valid, doesnt start with https"),
            ErrorsSpread::UrlNotReachable => write!(f, "website is unreachable"),
            ErrorsSpread::UrlNotFound => write!(f, "config.yaml file is not found"),
            ErrorsSpread::InvalidBody => write!(f, "config.yaml file is not in the correct format"),
            ErrorsSpread::InternalError => write!(f, "no urls found in the config.yaml file"),
            ErrorsSpread::EmptyBody => write!(f, "some other error"),
            ErrorsSpread::CouldNotExtractBrokerName => write!(f, "failed to parse the Vec<String> into a Vec<SymbolSpread>"),
            ErrorsSpread::FailedToOpenConfig => write!(f, "failed to parse the Symbol from the string"),
            ErrorsSpread::FailedToReadYaml => write!(f, "failed to parse the ask price from the string"),
            ErrorsSpread::FailedToParseSymbolSpread => write!(f, "failed to parse the bid price from the string"),
            ErrorsSpread::FailedToParseSymbol => write!(f, "failed to parse the spread from the string"),
            ErrorsSpread::FailedToParseAskPrice => write!(f, "couldn't retrieve spread data from the broker URL"),
            ErrorsSpread::FailedToParseBidPrice => write!(f, "failed to bind the json object to the key"),
            ErrorsSpread::FailedToParseSpread => write!(f, "failed to bind the json object to the key"),
            ErrorsSpread::CouldNotRetrieveSpreadData => write!(f, "failed to bind the json object to the key"),
            ErrorsSpread::FailedToBindJsonObjectToKey => write!(f, "failed to bind the json object to the key")
        }
    }
}


impl ErrorsSpread {
    pub fn new(
        &self
    ) -> &str {
        match self {
            ErrorsSpread::UrlInvalid => "url is not valid, doesnt start with https",
            ErrorsSpread::UrlNotReachable => "website is unreachable",
            ErrorsSpread::UrlNotFound => "config.yaml file is not found",
            ErrorsSpread::InvalidBody => "config.yaml file is not in the correct format",
            ErrorsSpread::InternalError => "no urls found in the config.yaml file",
            ErrorsSpread::EmptyBody => "some other error",
            ErrorsSpread::CouldNotExtractBrokerName => "failed to parse the Vec<String> into a Vec<SymbolSpread>",
            ErrorsSpread::FailedToOpenConfig => "failed to parse the Symbol from the string",
            ErrorsSpread::FailedToReadYaml => "failed to parse the ask price from the string",
            ErrorsSpread::FailedToParseSymbolSpread => "failed to parse the bid price from the string",
            ErrorsSpread::FailedToParseSymbol => "failed to parse the spread from the string",
            ErrorsSpread::FailedToParseAskPrice => "couldn't retrieve spread data from the broker URL",
            ErrorsSpread::FailedToParseBidPrice => "failed to bind the json object to the key",
            ErrorsSpread::FailedToParseSpread => "failed to bind the json object to the key",
            ErrorsSpread::CouldNotRetrieveSpreadData => "failed to bind the json object to the key",
            ErrorsSpread::FailedToBindJsonObjectToKey => "failed to bind the json object to the key"
        }
    }

}