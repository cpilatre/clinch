mod cli;
mod config;
mod display;
mod domain;
mod repository;

use console::Term;
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
#[structopt(name = "clinch")]
pub struct Opt {
    /// Count messages for the last 24 hours
    #[structopt(short = "n", long)]
    count: bool,

    /// Load and display messages
    #[structopt(short, long)]
    load_messages: bool,

    /// UTC time zone. Default is system local time zone
    #[structopt(short, long)]
    utc: bool,

    /// Customer plan
    #[structopt(long)]
    plan: Option<String>,

    /// Display SMS
    #[structopt(short, long)]
    show_sms: bool,

    /// Configuration file
    #[structopt(short, long, parse(from_os_str), default_value = ".sinch/config.json")]
    config: PathBuf,

    /// load specific page
    #[structopt(short, long, default_value = "0")]
    page: usize,

    /// Number of messages loaded
    #[structopt(short = "z", long, default_value = "10")]
    page_size: usize,

    /// Only list messages received at or after this date time (default now - 24h)
    #[structopt(long)]
    start_date: Option<String>,

    /// Only list messages received before this date time
    #[structopt(long)]
    end_date: Option<String>,

    /// Find a number
    #[structopt(short, long)]
    find: Option<String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let config = config::Config::from(&opt.config)?;

    let services_plan = config.get_plan(&opt.plan)?;
    let repository = repository::SinchRepository::new(services_plan);
    let sinch = domain::Sinch::new(repository);

    let term = Term::stdout();
    let services_map = config.get_map();
    let display = display::Display::init(term, services_map);

    if let Err(err) = cli::Cli::new(sinch, opt, display).run() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{}", err),
        )));
    }

    Ok(())
}
