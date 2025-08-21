use std::{io::{IsTerminal, stdout}, process::{self}};

use clap::Parser;
use cli::Args;
use color_eyre::Result;
use config::get_config;
use database::init_db;

mod app;
mod cli;
mod config;
mod data;
mod database;
mod modes;
mod panic;
mod utils;

use app::App;
use crossterm::{event::EnableMouseCapture, execute};

fn main() -> Result<()> {
    if !stdout().is_terminal() {
        eprintln!("Ensure that the program is running in a terminal");
        process::exit(1)
    }

    // Better panic + error messages
    color_eyre::install().expect("failed to setup color_eyre");
    // Better panic message in release mode
    setup_panic!();

    process::exit(run()?)
}

fn run() -> Result<i32> {
    let args = Args::parse();
    let db = init_db(&args.database)?;
    let config = get_config(&args.config, args.clean)?;
    let mut app = App::new(db, config)?;

    let mut terminal = ratatui::try_init()?;

    execute!(stdout(), EnableMouseCapture).inspect_err(|_| ratatui::restore())?;

    app.run(&mut terminal).inspect_err(|_| ratatui::restore())?;
    ratatui::restore();

    Ok(app.exit_code)
}
