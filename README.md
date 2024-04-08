# Forex spread tracker
`spread_tracker` is a library for tracking the spread of various symbols in the forex market.

Every forex broker has a different spread for each symbol. This library is used to track the spread of various symbols in the forex market.
Retrieve & scrape the spread of various symbols from different brokers.

## Supported brokers
- Vantage
- Pepperstone
- FusionMarkets
- UltimaMarkets
- Tickmill
- Errante
- CapitalCom
- XmGroup
- Headway
- ZeroMarkets
- AcySecurities
- MonetaMarkets
- BlueberryMarkets
- EbcFinancialGroup
- FpMarkets
- Fbs
- DnaMarkets
- OqTime
- ExclusiveMarkets
- GoMarkets
- FxPro
- ActiveTrades
- Octa
- ForexCom
- AccentForex
- Fxtm
- RoboForex
- MultiBankGroup
- LiteForex
- Just2Trade
- CmcMarkets
- Alpari
- Fxgt
- Exness
- JustMarkets
- FxPig
- XmTrading
- StarTrader
- Tmgm
- EightCap
- IcMarkets
- Afterprime

### Features
- Track the spread of various symbols in the forex market.
- Retrieve & scrape the spread of various symbols from different brokers.
- Store the spread data in a database.
- Cache the spread data to minimize the number of requests to the broker.

### Granularity
- Query the spread of all symbols from all listed brokers.
- Query the spread of a specific symbol from all listed brokers.
- Query the spread of all symbols from a specific broker.
- Query the spread of a specific symbol from a specific broker.

### Usage
Add this to your `Cargo.toml`:
```toml
[dependencies]
spread_tracker = "0.1.0"
```

### How to assign the correct Broker type
```rust
use spread_tracker::config::{
    SpreadBrokerUrl,
    Brokers
};
```

### How to get their spreads
```rust
use spread_tracker::config::SpreadBrokerUrl;
use spread_tracker::model::{
    Symbol,
    SymbolSpread
};

let config: SpreadBrokerUrl = SpreadBrokerUrl::new();

// In this example, we are tracking the spread of the symbols from the Vantage and EightCap brokers.
let brokers: Vec<Brokers> = vec![
    Brokers::Vantage,
    Brokers::EightCap,
];

// Get the spread of the symbols from the brokers
let spread: Value = SpreadTracker::get_spread(
    config,
    brokers
).await.unwrap();

println!("Spread: {:#?}", spread);

// Result:
{
 "spread": {
   "eightcap": [
     {
       "ask": 2197.92,
       "bid": 2209.92,
       "spread": 12.0,
       "symbol": "XAUUSD"
     },
     {
       "ask": 2031.57,
       "bid": 2060.8973,
       "spread": 29.3273,
       "symbol": "XAUEUR"
     },
     {
---------------- cut for brevity ----------------
```
### Return type
The return type is a `serde_json::Value` object, which is a JSON object.

#### Structure
Object[spread]Vector[broker] -> Object[symbol, ask, bid, spread]

* `spread` is the key for the spread data.
* `broker` is the key for the broker name, which will differ based on the broker.

- Notes:
A Vector of objects is sometimes referred to as a list of objects. It is a collection of objects that are stored in no particular order.

### Configuration
The `config.yaml` is used to store the broker URLs for the spread tracking, and the `model.rs` is used to store the data model for the spread tracking. The `config.yaml` file should be in the following format:
```yaml
BrokerSpreadUrls:
    Vantage: "https://www.vantagefx.com/trading-info/forex-market-hours/"
    MyFxBook: "https://www.myfxbook.com/forex-broker-quotes/vantage/6052"
```
Where `Vantage` and `MyFxBook` are the broker names and the URLs are the URLs of the brokers for spread tracking, derived from the MyFxBook website.

