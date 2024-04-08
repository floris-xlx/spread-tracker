//! # The `config` module
//!
//! This module contains the configuration for the spread tracker.
//! the default configuration is stored in the `config.yaml` file.
//!
//! ### Structuring of the `config.yaml` file
//! ```yaml
//! BrokerSpreadUrls:
//!  Vantage: "https://www.myfxbook.com/forex-broker-quotes/vantage/6052"
//! Pepperstone: "https://www.myfxbook.com/forex-broker-quotes/pepperstone/6053"
//! FusionMarkets: "https://www.myfxbook.com/forex-broker-quotes/fusionmarkets/6054"
//! UltimaMarkets: "https://www.myfxbook.com/forex-broker-quotes/ultimamarkets/6055"
//! Tickmill: "https://www.myfxbook.com/forex-broker-quotes/tickmill/6056"
//! xxxx
//! ```
//!
//! ### Usage
//!
//!

#![allow(clippy::inherent_to_string)]
#![allow(unused_imports)]
#![allow(clippy::new_without_default)]
use crate::model::SymbolSpread;


use serde_yaml::{
    Value,
    to_string
};
use std::{
    fs::File,
    io::BufReader
};



/// The `Brokers` enum is used to store the names of various brokers for spread tracking.
/// This enum is used to store the names of various brokers for spread tracking.
///
///
#[derive(Debug, Clone)]
pub enum Brokers {
    Vantage,
    Pepperstone,
    FusionMarkets,
    UltimaMarkets,
    Tickmill,
    Errante,
    CapitalCom,
    XmGroup,
    Headway,
    ZeroMarkets,
    AcySecurities,
    MonetaMarkets,
    BlueberryMarkets,
    EbcFinancialGroup,
    FpMarkets,
    Fbs,
    DnaMarkets,
    OqTime,
    ExclusiveMarkets,
    GoMarkets,
    FxPro,
    ActiveTrades,
    Octa,
    ForexCom,
    AccentForex,
    Fxtm,
    RoboForex,
    MultiBankGroup,
    LiteForex,
    Just2Trade,
    CmcMarkets,
    Alpari,
    Fxgt,
    Exness,
    JustMarkets,
    FxPig,
    XmTrading,
    StarTrader,
    Tmgm,
    EightCap,
    IcMarkets,
    Afterprime
}


/// The `SpreadBrokerUrl` struct is used to store the URLs of various brokers for spread tracking.
///
/// ### Example
///
/// ```no_run
/// use spread_tracker::config::SpreadBrokerUrl;
///
/// let spread_broker_url = SpreadBrokerUrl::new();
/// println!("{}", spread_broker_url.vantage);
///
/// - Expected output:
/// https://www.myfxbook.com/forex-broker-quotes/vantage/6052
/// ```
/// ### Errors
/// `url_not_found` will be returned if the URL is not found.
impl Brokers {

    pub fn to_string(
        &self
    ) -> String {

        match self {
            Brokers::Vantage => "Vantage".to_string(),
            Brokers::Pepperstone => "Pepperstone".to_string(),
            Brokers::FusionMarkets => "FusionMarkets".to_string(),
            Brokers::UltimaMarkets => "UltimaMarkets".to_string(),
            Brokers::Tickmill => "Tickmill".to_string(),
            Brokers::Errante => "Errante".to_string(),
            Brokers::CapitalCom => "CapitalCom".to_string(),
            Brokers::XmGroup => "XMGroup".to_string(),
            Brokers::Headway => "Headway".to_string(),
            Brokers::ZeroMarkets => "ZeroMarkets".to_string(),
            Brokers::AcySecurities => "AcySecurities".to_string(),
            Brokers::MonetaMarkets => "MonetaMarkets".to_string(),
            Brokers::BlueberryMarkets => "BlueberryMarkets".to_string(),
            Brokers::EbcFinancialGroup => "EbcFinancialGroup".to_string(),
            Brokers::FpMarkets => "FpMarkets".to_string(),
            Brokers::Fbs => "Fbs".to_string(),
            Brokers::DnaMarkets => "DnaMarkets".to_string(),
            Brokers::OqTime => "OqTime".to_string(),
            Brokers::ExclusiveMarkets => "ExclusiveMarkets".to_string(),
            Brokers::GoMarkets => "GoMarkets".to_string(),
            Brokers::FxPro => "FxPro".to_string(),
            Brokers::ActiveTrades => "ActivTrades".to_string(),
            Brokers::Octa => "Octa".to_string(),
            Brokers::ForexCom => "ForexCom".to_string(),
            Brokers::AccentForex => "AccentForex".to_string(),
            Brokers::Fxtm => "Fxtm".to_string(),
            Brokers::RoboForex => "RoboForex".to_string(),
            Brokers::MultiBankGroup => "MultiBankGroup".to_string(),
            Brokers::LiteForex => "LiteForex".to_string(),
            Brokers::Just2Trade => "Just2Trade".to_string(),
            Brokers::CmcMarkets => "CmcMarkets".to_string(),
            Brokers::Alpari => "Alpari".to_string(),
            Brokers::Fxgt => "Fxgt".to_string(),
            Brokers::Exness => "Exness".to_string(),
            Brokers::JustMarkets => "JustMarkets".to_string(),
            Brokers::FxPig => "FxPig".to_string(),
            Brokers::XmTrading => "XmTrading".to_string(),
            Brokers::StarTrader => "StarTrader".to_string(),
            Brokers::Tmgm => "Tmgm".to_string(),
            Brokers::EightCap => "EightCap".to_string(),
            Brokers::IcMarkets => "IcMarkets".to_string(),
            Brokers::Afterprime => "Afterprime".to_string()
        }
    }

    /// # `get_url` Get the URL of the broker.
    pub fn get_url(
        &self
    ) -> String {
        let spread_broker_url: SpreadBrokerUrl = SpreadBrokerUrl::new();

        match self {
            Brokers::Vantage => spread_broker_url.vantage,
            Brokers::Pepperstone => spread_broker_url.pepperstone,
            Brokers::FusionMarkets => spread_broker_url.fusion_markets,
            Brokers::UltimaMarkets => spread_broker_url.ultima_markets,
            Brokers::Tickmill => spread_broker_url.tickmill,
            Brokers::Errante => spread_broker_url.errante,
            Brokers::CapitalCom => spread_broker_url.capital_com,
            Brokers::XmGroup => spread_broker_url.xm_group,
            Brokers::Headway => spread_broker_url.headway,
            Brokers::ZeroMarkets => spread_broker_url.zero_markets,
            Brokers::AcySecurities => spread_broker_url.acy_securities,
            Brokers::MonetaMarkets => spread_broker_url.moneta_markets,
            Brokers::BlueberryMarkets => spread_broker_url.blueberry_markets,
            Brokers::EbcFinancialGroup => spread_broker_url.ebc_financial_group,
            Brokers::FpMarkets => spread_broker_url.fp_markets,
            Brokers::Fbs => spread_broker_url.fbs,
            Brokers::DnaMarkets => spread_broker_url.dna_markets,
            Brokers::OqTime => spread_broker_url.oq_time,
            Brokers::ExclusiveMarkets => spread_broker_url.exclusive_markets,
            Brokers::GoMarkets => spread_broker_url.go_markets,
            Brokers::FxPro => spread_broker_url.fx_pro,
            Brokers::ActiveTrades => spread_broker_url.active_trades,
            Brokers::Octa => spread_broker_url.octa,
            Brokers::ForexCom => spread_broker_url.forex_com,
            Brokers::AccentForex => spread_broker_url.accent_forex,
            Brokers::Fxtm => spread_broker_url.fxtm,
            Brokers::RoboForex => spread_broker_url.robo_forex,
            Brokers::MultiBankGroup => spread_broker_url.multi_bank_group,
            Brokers::LiteForex => spread_broker_url.lite_forex,
            Brokers::Just2Trade => spread_broker_url.just2_trade,
            Brokers::CmcMarkets => spread_broker_url.cmc_markets,
            Brokers::Alpari => spread_broker_url.alpari,
            Brokers::Fxgt => spread_broker_url.fxgt,
            Brokers::Exness => spread_broker_url.exness,
            Brokers::JustMarkets => spread_broker_url.just_markets,
            Brokers::FxPig => spread_broker_url.fx_pig,
            Brokers::XmTrading => spread_broker_url.xm_trading,
            Brokers::StarTrader => spread_broker_url.star_trader,
            Brokers::Tmgm => spread_broker_url.tmgm,
            Brokers::EightCap => spread_broker_url.eight_cap,
            Brokers::IcMarkets => spread_broker_url.ic_markets,
            Brokers::Afterprime => spread_broker_url.afterprime
        }
    }
}


/// The `SpreadBrokerUrl` struct is used to store the URLs of various brokers for spread tracking.
/// This struct is used to store the URLs of various brokers for spread tracking.
///
/// ### Example
///
/// ```
/// use spread_tracker::config::SpreadBrokerUrl;
///
/// let spread_broker_url = SpreadBrokerUrl::new();
/// println!("{}", spread_broker_url.vantage);
///
/// - Expected output:
/// https://www.myfxbook.com/forex-broker-quotes/vantage/6052
/// ```
///
/// ### Errors
/// `url_not_found` will be returned if the URL is not found.
/// `invalid_url` will be returned if the URL is invalid.
#[derive(Debug, Clone)]
pub struct SpreadBrokerUrl {
    pub vantage: String,
    pub pepperstone: String,
    pub fusion_markets: String,
    pub ultima_markets: String,
    pub tickmill: String,
    pub errante: String,
    pub capital_com: String,
    pub xm_group: String,
    pub headway: String,
    pub zero_markets: String,
    pub acy_securities: String,
    pub moneta_markets: String,
    pub blueberry_markets: String,
    pub ebc_financial_group: String,
    pub fp_markets: String,
    pub fbs: String,
    pub dna_markets: String,
    pub oq_time: String,
    pub exclusive_markets: String,
    pub go_markets: String,
    pub fx_pro: String,
    pub active_trades: String,
    pub octa: String,
    pub forex_com: String,
    pub accent_forex: String,
    pub fxtm: String,
    pub robo_forex: String,
    pub multi_bank_group: String,
    pub lite_forex: String,
    pub just2_trade: String,
    pub cmc_markets: String,
    pub alpari: String,
    pub fxgt: String,
    pub exness: String,
    pub just_markets: String,
    pub fx_pig: String,
    pub xm_trading: String,
    pub star_trader: String,
    pub tmgm: String,
    pub eight_cap: String,
    pub ic_markets: String,
    pub afterprime: String
}


// impl get_url for SpreadBrokerUrl
impl SpreadBrokerUrl {
    pub fn get_url(
        &self,
        broker: Brokers
    ) -> String {
        match broker {
            Brokers::Vantage => self.vantage.clone(),
            Brokers::Pepperstone => self.pepperstone.clone(),
            Brokers::FusionMarkets => self.fusion_markets.clone(),
            Brokers::UltimaMarkets => self.ultima_markets.clone(),
            Brokers::Tickmill => self.tickmill.clone(),
            Brokers::Errante => self.errante.clone(),
            Brokers::CapitalCom => self.capital_com.clone(),
            Brokers::XmGroup => self.xm_group.clone(),
            Brokers::Headway => self.headway.clone(),
            Brokers::ZeroMarkets => self.zero_markets.clone(),
            Brokers::AcySecurities => self.acy_securities.clone(),
            Brokers::MonetaMarkets => self.moneta_markets.clone(),
            Brokers::BlueberryMarkets => self.blueberry_markets.clone(),
            Brokers::EbcFinancialGroup => self.ebc_financial_group.clone(),
            Brokers::FpMarkets => self.fp_markets.clone(),
            Brokers::Fbs => self.fbs.clone(),
            Brokers::DnaMarkets => self.dna_markets.clone(),
            Brokers::OqTime => self.oq_time.clone(),
            Brokers::ExclusiveMarkets => self.exclusive_markets.clone(),
            Brokers::GoMarkets => self.go_markets.clone(),
            Brokers::FxPro => self.fx_pro.clone(),
            Brokers::ActiveTrades => self.active_trades.clone(),
            Brokers::Octa => self.octa.clone(),
            Brokers::ForexCom => self.forex_com.clone(),
            Brokers::AccentForex => self.accent_forex.clone(),
            Brokers::Fxtm => self.fxtm.clone(),
            Brokers::RoboForex => self.robo_forex.clone(),
            Brokers::MultiBankGroup => self.multi_bank_group.clone(),
            Brokers::LiteForex => self.lite_forex.clone(),
            Brokers::Just2Trade => self.just2_trade.clone(),
            Brokers::CmcMarkets => self.cmc_markets.clone(),
            Brokers::Alpari => self.alpari.clone(),
            Brokers::Fxgt => self.fxgt.clone(),
            Brokers::Exness => self.exness.clone(),
            Brokers::JustMarkets => self.just_markets.clone(),
            Brokers::FxPig => self.fx_pig.clone(),
            Brokers::XmTrading => self.xm_trading.clone(),
            Brokers::StarTrader => self.star_trader.clone(),
            Brokers::Tmgm => self.tmgm.clone(),
            Brokers::EightCap => self.eight_cap.clone(),
            Brokers::IcMarkets => self.ic_markets.clone(),
            Brokers::Afterprime => self.afterprime.clone()
        }
    }
}

/// ### Implementing a method for the SpreadBrokerUrl struct to load the configuration from the config.yaml file.
/// This method is used to load the configuration from the config.yaml file.
///
/// ### Example
///
/// ```
/// use spread_tracker::config::SpreadBrokerUrl;
///
/// let spread_broker_url = SpreadBrokerUrl::new();
/// ```
///
/// ### Errors
///
/// This method will return an error if the config.yaml file is not found or if the file is not in the correct format.
impl SpreadBrokerUrl {
    /// ### Implementing a method for the SpreadBrokerUrl struct to load the configuration from the config.yaml file.
    ///
    /// ### Example
    ///
    /// ```no_run
    /// use spread_tracker::config::SpreadBrokerUrl;
    ///
    /// let mut spread_broker_url = SpreadBrokerUrl::new();
    /// spread_broker_url.load_config();
    /// ```
    /// !!!! IMPORTANT !!!!
    ///
    /// When it's run in a package and loaded via a `.toml` file it will expect the .yaml file to be in the same directory as the package.
    pub fn load_config(
        &mut self
    ) -> &mut Self {
        let file: File = File::open("spread_config.yaml").expect("Failed to open spread_config.yaml");
        let reader: BufReader<File> = BufReader::new(file);
        let value: Value = serde_yaml::from_reader(reader).expect("Failed to read YAML");
        let prefix_value: Value = value["BrokerSpreadUrls"].clone();


        // when errors occur, the program will panic
        // it most likely means that the config.yaml file is not in the correct format

        self.vantage = prefix_value["Vantage"].as_str().unwrap().to_string();
        self.pepperstone = prefix_value["Pepperstone"].as_str().unwrap().to_string();
        self.fusion_markets = prefix_value["FusionMarkets"].as_str().unwrap().to_string();
        self.ultima_markets = prefix_value["UltimaMarkets"].as_str().unwrap().to_string();
        self.tickmill = prefix_value["Tickmill"].as_str().unwrap().to_string();
        self.errante = prefix_value["Errante"].as_str().unwrap().to_string();
        self.capital_com = prefix_value["CapitalCom"].as_str().unwrap().to_string();
        self.xm_group = prefix_value["XmGroup"].as_str().unwrap().to_string();
        self.headway = prefix_value["Headway"].as_str().unwrap().to_string();
        self.zero_markets = prefix_value["ZeroMarkets"].as_str().unwrap().to_string();
        self.acy_securities = prefix_value["AcySecurities"].as_str().unwrap().to_string();
        self.moneta_markets = prefix_value["MonetaMarkets"].as_str().unwrap().to_string();
        self.blueberry_markets = prefix_value["BlueberryMarkets"].as_str().unwrap().to_string();
        self.ebc_financial_group = prefix_value["EbcFinancialGroup"].as_str().unwrap().to_string();
        self.fp_markets = prefix_value["FpMarkets"].as_str().unwrap().to_string();
        self.fbs = prefix_value["Fbs"].as_str().unwrap().to_string();
        self.dna_markets = prefix_value["DnaMarkets"].as_str().unwrap().to_string();
        self.oq_time = prefix_value["OqTime"].as_str().unwrap().to_string();
        self.exclusive_markets = prefix_value["ExclusiveMarkets"].as_str().unwrap().to_string();
        self.go_markets = prefix_value["GoMarkets"].as_str().unwrap().to_string();
        self.fx_pro = prefix_value["FxPro"].as_str().unwrap().to_string();
        self.active_trades = prefix_value["ActivTrades"].as_str().unwrap().to_string();
        self.octa = prefix_value["Octa"].as_str().unwrap().to_string();
        self.forex_com = prefix_value["ForexCom"].as_str().unwrap().to_string();
        self.accent_forex = prefix_value["AccentForex"].as_str().unwrap().to_string();
        self.fxtm = prefix_value["Fxtm"].as_str().unwrap().to_string();
        self.robo_forex = prefix_value["RoboForex"].as_str().unwrap().to_string();
        self.multi_bank_group = prefix_value["MultiBankGroup"].as_str().unwrap().to_string();
        self.lite_forex = prefix_value["LiteForex"].as_str().unwrap().to_string();
        self.just2_trade = prefix_value["Just2Trade"].as_str().unwrap().to_string();
        self.cmc_markets = prefix_value["CmcMarkets"].as_str().unwrap().to_string();
        self.alpari = prefix_value["Alpari"].as_str().unwrap().to_string();
        self.fxgt = prefix_value["Fxgt"].as_str().unwrap().to_string();
        self.exness = prefix_value["Exness"].as_str().unwrap().to_string();
        self.just_markets = prefix_value["JustMarkets"].as_str().unwrap().to_string();
        self.fx_pig = prefix_value["FxPig"].as_str().unwrap().to_string();
        self.xm_trading = prefix_value["XmTrading"].as_str().unwrap().to_string();
        self.star_trader = prefix_value["StarTrader"].as_str().unwrap().to_string();
        self.tmgm = prefix_value["Tmgm"].as_str().unwrap().to_string();
        self.eight_cap = prefix_value["EightCap"].as_str().unwrap().to_string();
        self.ic_markets = prefix_value["IcMarkets"].as_str().unwrap().to_string();
        self.afterprime = prefix_value["Afterprime"].as_str().unwrap().to_string();



        self
    }

    /// # Implementing a method for the SpreadBrokerUrl struct to create a new instance of the struct.
    ///
    /// ### Example
    ///
    /// ```
    /// use spread_tracker::config::SpreadBrokerUrl;
    ///
    /// let spread_broker_url = SpreadBrokerUrl::new();
    /// println!("{}", spread_broker_url.vantage);
    ///
    /// - Expected output:
    /// https://www.myfxbook.com/forex-broker-quotes/vantage/6052
    ///
    /// ```
    ///
    /// ### Errors
    /// `url_not_found` will be returned if the URL is not found.
    /// `invalid_url` will be returned if the URL is invalid.
    ///
    pub fn new() -> Self {
        let mut spread_broker_url: SpreadBrokerUrl = SpreadBrokerUrl {
            vantage: String::new(),
            pepperstone: String::new(),
            fusion_markets: String::new(),
            ultima_markets: String::new(),
            tickmill: String::new(),
            errante: String::new(),
            capital_com: String::new(),
            xm_group: String::new(),
            headway: String::new(),
            zero_markets: String::new(),
            acy_securities: String::new(),
            moneta_markets: String::new(),
            blueberry_markets: String::new(),
            ebc_financial_group: String::new(),
            fp_markets: String::new(),
            fbs: String::new(),
            dna_markets: String::new(),
            oq_time: String::new(),
            exclusive_markets: String::new(),
            go_markets: String::new(),
            fx_pro: String::new(),
            active_trades: String::new(),
            octa: String::new(),
            forex_com: String::new(),
            accent_forex: String::new(),
            fxtm: String::new(),
            robo_forex: String::new(),
            multi_bank_group: String::new(),
            lite_forex: String::new(),
            just2_trade: String::new(),
            cmc_markets: String::new(),
            alpari: String::new(),
            fxgt: String::new(),
            exness: String::new(),
            just_markets: String::new(),
            fx_pig: String::new(),
            xm_trading: String::new(),
            star_trader: String::new(),
            tmgm: String::new(),
            eight_cap: String::new(),
            ic_markets: String::new(),
            afterprime: String::new()
        };

        spread_broker_url.load_config();

        spread_broker_url
    }
}