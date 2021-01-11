use anyhow::{Context, Result};
use clap::{crate_authors, crate_name, crate_version, Arg};

#[derive(Debug)]
pub struct Options {
    pub listen_addr: std::net::IpAddr,
    pub listen_port: u16,
    pub log_level: log::Level,
    pub token: String,
    pub symbols: Vec<String>,
}

impl Options {
    pub fn get_args() -> Result<Options> {
        let matches = Options::parse_cli().get_matches();
        Options::from_claps(&matches)
    }
    fn parse_cli() -> clap::App<'static, 'static> {
        clap::App::new(crate_name!())
            .version(crate_version!())
            .author(crate_authors!())
            .arg(
                Arg::with_name("LISTEN_ADDR")
                    .short("a")
                    .help("Listen IPV4 address")
                    .takes_value(true)
                    .required(false),
            )
            .arg(
                Arg::with_name("LISTEN_PORT")
                    .short("p")
                    .help("Listen port")
                    .takes_value(true)
                    .required(false),
            )
            .arg(
                Arg::with_name("LOG_LEVEL")
                    .short("l")
                    .help("Log level")
                    .takes_value(true)
                    .required(false),
            )
            .arg(
                Arg::with_name("TOKEN")
                    .short("t")
                    .help("API token")
                    .takes_value(true)
                    .required(true),
            )
            .arg(
                Arg::with_name("SYMBOL")
                    .short("s")
                    .help("Stock Symbol")
                    .takes_value(true)
                    .multiple(true)
                    .number_of_values(1)
                    .required(true),
            )
    }

    fn from_claps(matches: &clap::ArgMatches<'_>) -> Result<Options> {
        let listen_addr = matches
            .value_of("LISTEN_ADDR")
            .unwrap_or("0.0.0.0")
            .parse()
            .context("Invalid listen address")?;

        let listen_port = matches
            .value_of("LISTEN_PORT")
            .unwrap_or("8080")
            .parse()
            .context("Invalid listen port")?;

        let log_level = matches
            .value_of("LOG_LEVEL")
            .unwrap_or("info")
            .parse()
            .context("Invalid log level")?;

        let token = matches.value_of("TOKEN").unwrap().to_string();

        let symbols: Vec<_> = matches
            .values_of("SYMBOL")
            .unwrap()
            .map(String::from)
            .collect();

        Ok(Options {
            listen_addr,
            listen_port,
            log_level,
            token,
            symbols,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cli_args() {
        let good_res = Options::parse_cli().get_matches_from_safe(vec![
            crate_name!(),
            "-l",
            "debug",
            "-t",
            "pk_test",
            "-s",
            "AAPL",
            "-s",
            "GOOG",
            "-a",
            "127.0.0.1",
            "-p",
            "8081",
        ]);

        let bad_res_no_symbol = Options::parse_cli().get_matches_from_safe(vec![
            crate_name!(),
            "-l",
            "debug",
            "-t",
            "pk_test",
        ]);

        let bad_res_no_token = Options::parse_cli().get_matches_from_safe(vec![
            crate_name!(),
            "-l",
            "debug",
            "-s",
            "AAPL",
        ]);

        let bad_res_unknown_arg =
            Options::parse_cli().get_matches_from_safe(vec![crate_name!(), "-b", "debug"]);

        assert!(good_res.is_ok());
        assert!(bad_res_no_symbol.is_err());
        assert!(bad_res_no_token.is_err());
        assert!(bad_res_unknown_arg.is_err());
    }

    #[test]
    fn build_opts_from_valid_claps() {
        let good_matches = Options::parse_cli().get_matches_from(vec![
            crate_name!(),
            "-l",
            "debug",
            "-t",
            "pk_test",
            "-s",
            "AAPL",
            "-s",
            "GOOG",
            "-a",
            "127.0.0.1",
            "-p",
            "8081",
        ]);

        let opts = Options::from_claps(&good_matches);
        assert!(opts.is_ok());

        let opts = opts.unwrap();
        assert_eq!(opts.log_level, log::Level::Debug);
        assert_eq!(opts.token, String::from("pk_test"));
        assert_eq!(
            opts.symbols,
            vec![String::from("AAPL"), String::from("GOOG")]
        );
        assert_eq!(opts.listen_addr, std::net::Ipv4Addr::new(127, 0, 0, 1));
        assert_eq!(opts.listen_port, 8081);
    }

    #[test]
    fn build_opts_from_invalid_claps() {
        let bad_matches_log_level = Options::parse_cli().get_matches_from(vec![
            crate_name!(),
            "-l",
            "bad_level",
            "-t",
            "pk_test",
            "-s",
            "AAPL",
        ]);

        let bad_matches_listen_addr = Options::parse_cli().get_matches_from(vec![
            crate_name!(),
            "-a",
            "127.0.0.0.1",
            "-t",
            "pk_test",
            "-s",
            "AAPL",
        ]);

        let bad_matches_listen_port = Options::parse_cli().get_matches_from(vec![
            crate_name!(),
            "-p",
            "999999",
            "-t",
            "pk_test",
            "-s",
            "AAPL",
        ]);

        let opts = Options::from_claps(&bad_matches_log_level);
        assert!(opts.is_err());

        let opts = Options::from_claps(&bad_matches_listen_addr);
        assert!(opts.is_err());

        let opts = Options::from_claps(&bad_matches_listen_port);
        assert!(opts.is_err());
    }
}
