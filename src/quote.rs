#[derive(Debug, Deserialize)]
pub(crate) struct Quote {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "companyName")]
    pub company_name: String,
    #[serde(rename = "latestPrice")]
    pub latest_price: f64,
    #[serde(rename = "latestVolume")]
    pub latest_volume: i64,
    #[serde(rename = "marketCap")]
    pub market_cap: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_quote() {
        let quote_str = 
            "{
              \"symbol\": \"AAPL\",
              \"companyName\": \"Apple Inc\",
              \"latestPrice\": 137.02,
              \"latestVolume\": 109244453,
              \"marketCap\": 2264005852377
            }";
        let quote: Quote = serde_json::from_str(quote_str).unwrap();

        assert_eq!(quote.symbol, String::from("AAPL"));
        assert_eq!(quote.company_name, String::from("Apple Inc"));
        assert_eq!(quote.latest_price, 137.02);
        assert_eq!(quote.latest_volume, 109244453);
        assert_eq!(quote.market_cap, 2264005852377);
    }
}