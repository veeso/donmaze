#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

use log::LevelFilter;

mod args;
mod audio;
mod game;
mod gfx;
mod ui;
mod utils;

use args::Args;
use game::Runtime;

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() -> anyhow::Result<()> {
    // parse arguments
    let args: Args = argh::from_env();
    // print version
    if args.version {
        anyhow::bail!("donmaze {} - developed by {}", APP_VERSION, APP_AUTHORS)
    }
    // setup logging
    let log_level = if args.debug {
        LevelFilter::Debug
    } else if args.verbose {
        LevelFilter::Info
    } else {
        LevelFilter::Off
    };
    todo!("setup logger");

    // run Game
    Runtime::setup()?.run()?;
    Ok(())
}
