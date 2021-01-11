#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

use anyhow::{Context, Result};
use env_logger::{Builder, Env};
use futures::{stream, StreamExt};
use log::{debug, error, info};
use reqwest::Client;

mod options;
use crate::options::Options;
mod quote;
use crate::quote::Quote;
mod metrics;
use crate::metrics::{LATEST_PRICE, LATEST_VOLUME, MARKET_CAP};

const IEXAPIS_URL: &str = "https://sandbox.iexapis.com/stable/stock";
const QUERY_INTERVAL: std::time::Duration = std::time::Duration::from_secs(30);
const CONCURRENT_REQUESTS: usize = 2;

fn publish_metrics(quote: Quote) {
    let label_values = vec![quote.symbol.as_str(), quote.company_name.as_str()];

    LATEST_PRICE
        .with_label_values(&label_values)
        .set(quote.latest_price);
    LATEST_VOLUME
        .with_label_values(&label_values)
        .set(quote.latest_volume);
    MARKET_CAP
        .with_label_values(&label_values)
        .set(quote.market_cap);
}

#[tokio::main]
async fn main() -> Result<()> {
    let options = Options::get_args()?;

    Builder::from_env(Env::default().default_filter_or(options.log_level.to_string())).init();

    let exporter = prometheus_exporter::start(std::net::SocketAddr::new(
        options.listen_addr,
        options.listen_port,
    ))
    .context(format!(
        "Failed to start listening on {}:{}",
        options.listen_addr, options.listen_port
    ))?;

    let client = Client::new();

    loop {
        info!("Updating metrics");
        stream::iter(&options.symbols)
            .map(|symbol| {
                let client = &client;
                let url = format!("{}/{}/quote?token=T{}", IEXAPIS_URL, symbol, options.token);
                async move { client.get(&url).send().await?.json::<Quote>().await }
            })
            .buffer_unordered(CONCURRENT_REQUESTS)
            .for_each(|q| async {
                match q {
                    Ok(q) => {
                        debug!("Retrieved quote: {:?}", q);
                        publish_metrics(q);
                    }
                    Err(e) => error!("Failed to retrieve quote {}", e),
                }
            })
            .await;

        let _guard = exporter.wait_duration(QUERY_INTERVAL);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn publish_metrics_from_quote() {
        let q = Quote {
            symbol: String::from("AAPL"),
            company_name: String::from("Apple Inc"),
            latest_price: 240.0, 
            latest_volume: 1060862, 
            market_cap: 7629978361,
        };

        publish_metrics(q);
        
        let latest_price = LATEST_PRICE.get_metric_with_label_values(&["AAPL", "Apple Inc"]);
        assert!(latest_price.is_ok());
        assert_eq!(latest_price.unwrap().get(), 240.0);

        let latest_volume = LATEST_VOLUME.get_metric_with_label_values(&["AAPL", "Apple Inc"]);
        assert!(latest_volume.is_ok());
        assert_eq!(latest_volume.unwrap().get(), 1060862);

        let market_cap = MARKET_CAP.get_metric_with_label_values(&["AAPL", "Apple Inc"]);
        assert!(market_cap.is_ok());
        assert_eq!(market_cap.unwrap().get(), 7629978361);
    }
}
