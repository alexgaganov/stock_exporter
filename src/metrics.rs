use prometheus_exporter::prometheus::{register_gauge_vec, register_int_gauge_vec, IntGaugeVec, GaugeVec, Opts};

static VARIABLE_LABELS: &[&str] = &["symbol", "company_name"];
static METRICS_NAMESPACE: &str = "stocks";

lazy_static! { 
   pub static ref LATEST_PRICE: GaugeVec = register_gauge_vec!(
        Opts::new("latest_price", "Latest stock price").namespace(METRICS_NAMESPACE),
        VARIABLE_LABELS
    )
    .expect("can not create LATEST_PRICE metric. this should never fail");

    pub static ref LATEST_VOLUME: IntGaugeVec = register_int_gauge_vec!(
        Opts::new("latest_volume", "Latest stock volume").namespace(METRICS_NAMESPACE),
        VARIABLE_LABELS
    )
    .expect("can not create LATEST_VOLUME metric. this should never fail");

    pub static ref MARKET_CAP: IntGaugeVec = register_int_gauge_vec!(
        Opts::new("market_cap", "Market capitalization").namespace(METRICS_NAMESPACE),
        VARIABLE_LABELS
    )
    .expect("can not create MARKET_CAP metric. this should never fail");
}