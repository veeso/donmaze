//! # Args
//!
//! Cli args

use argh::FromArgs;

#[derive(FromArgs)]
#[argh(
    description = "Please, report issues to <https://github.com/veeso/donmaze>
Please, consider supporting the author <https://ko-fi.com/veeso>"
)]
pub struct Args {
    #[argh(switch, short = 'D', description = "enable TRACE log level")]
    pub debug: bool,
    #[argh(switch, short = 'm', description = "play donmaze without audio")]
    pub muted: bool,
    #[argh(option, short = 'T', description = "UI refresh rate (ms)")]
    pub ticks: Option<u64>,
    #[argh(switch, short = 'v', description = "verbose mode")]
    pub verbose: bool,
    #[argh(switch, short = 'V', description = "print version")]
    pub version: bool,
}
