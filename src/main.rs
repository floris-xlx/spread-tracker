#![allow(unused_imports)]
#![allow(clippy::single_component_path_imports)]

pub mod utils;
pub mod config;
pub mod model;

use spread_tracker::SpreadTracker;
use spread_tracker::model::SymbolSpread;
use spread_tracker::config::{ SpreadBrokerUrl, Brokers };

use serde_json::Value;
use std::fs::File;
use std::io::Write;

use tracing_subscriber::EnvFilter;
use tracing::{ info, warn, error };

#[tokio::main]
async fn main() {
    let config: SpreadBrokerUrl = SpreadBrokerUrl::new();

    let brokers: Vec<Brokers> = vec![
        Brokers::FusionMarkets,
        Brokers::IcMarkets,
        Brokers::Octa,
        Brokers::OqTime,
        Brokers::BlueberryMarkets,
        Brokers::EbcFinancialGroup,
        Brokers::EightCap,
        Brokers::Errante,
        Brokers::ExclusiveMarkets,
        Brokers::Exness,
        Brokers::RoboForex,
        Brokers::Tickmill,
        Brokers::Tmgm,
        Brokers::XmTrading,
        Brokers::StarTrader,
        Brokers::Just2Trade,
        Brokers::ActiveTrades,
        Brokers::UltimaMarkets,
        Brokers::Pepperstone,
        Brokers::FxPig,
        Brokers::FxPro
    ];

    let result: Value = SpreadTracker::get_spread(config, brokers).await.unwrap();

    init_tracing();

    info!("Spread yield: {:#?}", result);

    // // save to file json with indent 4
    // let mut file: File = File::create("output.json").unwrap();
    // file.write_all(serde_json::to_string_pretty(&result).unwrap().as_bytes()).unwrap();
}

fn init_tracing() {
    let filter: EnvFilter = EnvFilter::try_from_default_env().unwrap_or_else(|_|
        EnvFilter::new("error")
            .add_directive("warn".parse().unwrap())
            .add_directive("info".parse().unwrap())
    );

    tracing_subscriber::fmt().with_env_filter(filter).init();
}
