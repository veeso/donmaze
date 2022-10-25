#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;

mod args;
mod audio;
mod game;
mod gfx;
mod ui;
mod utils;

use args::Args;
use game::{Options, Runtime};

use log::LevelFilter;

const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() -> anyhow::Result<()> {
    // parse arguments
    let args: Args = argh::from_env();
    // print version
    if args.version {
        anyhow::bail!("donmaze {} - developed by {}", APP_VERSION, APP_AUTHORS)
    }
    // setup config dir
    let config_dir =
        utils::dirs::init_config_dir()?.expect("your system doesn't support config directory");
    let game_saves_dir = utils::dirs::get_saves_path(&config_dir)?;
    // setup logging
    let log_level = if args.debug {
        LevelFilter::Debug
    } else if args.verbose {
        LevelFilter::Info
    } else {
        LevelFilter::Off
    };
    if log_level != LevelFilter::Off {
        utils::setup_logger(log_level, &utils::dirs::get_log_path(&config_dir))?;
    }
    info!("starting donmaze {}", APP_VERSION);
    // run Game
    Runtime::setup(
        Options::default()
            .music(!args.no_music && !args.muted)
            .sound(!args.muted)
            .saved_games_dir(game_saves_dir)
            .ticks(args.ticks.unwrap_or(10)),
    )?
    .run()?;
    Ok(())
}
