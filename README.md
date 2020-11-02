# Clinch
A command line app in RUST to retrieve Sinch inbound messages

## Usage
```bash
$ clinch -h
clinch 0.1.8

USAGE:
    clinch [FLAGS] [OPTIONS]

FLAGS:
    -n, --count            Count messages for the last 24 hours
    -h, --help             Prints help information
    -l, --load-messages    Load and display messages
    -s, --show-sms         Display SMS
    -u, --utc              UTC time zone. Default is system local time zone
    -V, --version          Prints version information

OPTIONS:
    -c, --config <config>            Configuration file [default: .sinch/config.json]
        --end-date <end-date>        Only list messages received before this date time
    -f, --find <find>                Find a number
    -p, --page <page>                load specific page [default: 0]
    -z, --page-size <page-size>      Number of messages loaded [default: 10]
        --plan <plan>                Customer plan
        --start-date <start-date>    Only list messages received at or after this date time (default now - 24h)
```