use clap::Parser;

use crate::timer::DEFAULT_COUNTDOWN_SECOND;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Count down time (in seconds)
    #[arg(short, long, default_value_t = DEFAULT_COUNTDOWN_SECOND)]
    pub time: f64,
}
