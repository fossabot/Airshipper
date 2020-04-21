//! This lib parses command line arguments and returns a parsed struct on which the GUI/CLI can act upon.
use clap::{crate_authors, crate_version, Clap};

/// Provides automatic updates for the voxel RPG Veloren. ( https://veloren.net )
#[derive(Clap, Debug)]
#[clap(name = "Airshipper", bin_name = "airshipper", version = crate_version!(), author = crate_authors!())]
pub struct CmdLine {
    #[clap(subcommand)]
    pub action: Option<Action>,
    /// Set the logging verbosity for Veloren (v = INFO, vv = DEBUG, vvv = TRACE)
    #[clap(short, long, parse(from_occurrences), global = true)]
    pub verbose: i32,
    /// Set the logging verbosity for Airshipper (v = DEBUG, vv = TRACE)
    #[clap(short, long, parse(from_occurrences), global = true)]
    pub debug: i32,
}

#[derive(Clap, Debug)]
pub enum Action {
    /// Starts the game without updating
    Start,
    /// Only updates the game
    Update,
}

pub fn parse() -> CmdLine {
    CmdLine::parse()
}
