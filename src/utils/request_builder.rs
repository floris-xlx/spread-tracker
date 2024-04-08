//! # Request Builder

#![allow(dead_code)]
use reqwest::{
    Client,
    Response
};

use crate::model::{
    Symbol,
    SymbolSpread,
    HttpsUrl
};
use crate::config::SpreadBrokerUrl;

/// The `RequestBuilder` struct is used to build the request to get the spread data from the broker.
/// This struct is used to build the request to get the spread data from the broker.
///
/// ### Example
///
///
pub struct RequestBuilder {
    client: Client,
    spread_broker_url: SpreadBrokerUrl
}