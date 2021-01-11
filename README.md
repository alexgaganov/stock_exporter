# stock_exporter
Retrieve stock quotes from IEX Cloud API and export them as Prometheus metrics.
Just learning some Rust. Probably won't be useful for anyone.

```
USAGE:
    stock_exporter [OPTIONS] -s <SYMBOL>... -t <TOKEN>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a <LISTEN_ADDR>        Listen IPV4 address
    -p <LISTEN_PORT>        Listen port
    -l <LOG_LEVEL>          Log level
    -s <SYMBOL>...          Stock Symbol
    -t <TOKEN>              API token
```
