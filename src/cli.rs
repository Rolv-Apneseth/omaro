use std::path::PathBuf;

use clap::Parser;

use crate::{config::DEFAULT_CONFIG_PATH, database::DEFAULT_DB_PATH};

#[derive(Parser, Debug)]
#[command(version, about="TUI for the fantastic lobste.rs", long_about = None)]
pub struct Args {
    #[arg(
        short = 'c',
        long,
        default_value = DEFAULT_CONFIG_PATH.to_str(),
        help = "Path to a config file - overrides the default",
        env = "OMARO_CONFIG",
    )]
    pub config: PathBuf,

    #[arg(
        short = 'C',
        long,
        help = "Run without a config file - overrides any provided config file",
        env = "OMARO_CLEAN"
    )]
    pub clean: bool,

    #[arg(
        short,
        long,
        default_value = DEFAULT_DB_PATH.to_str(),
        help = "Path to the SQlite database to be used for marking posts as read - overrides the default",
        env = "OMARO_DB"
    )]
    pub database: PathBuf,
}
