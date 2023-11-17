// Command Line Interface Manager
// Author: Derek Blaney

use clap::{Parser, Subcommand};
use std::ffi::OsString;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub(crate) zip: Zip,
}

#[derive(Subcommand, Debug)]
pub enum Zip {
    Create {
        /// The folder location
        location: OsString,
        /// The name given to the zip file
        name: Option<OsString>,

        // Flags

        #[arg(short, long)]
        verbose: bool,

        /// Exclude certain files, extensions, folders
        #[arg(short, long)]
        exclude: Vec<String>,

        /// Include certain files, extensions, folders
        #[arg(short, long)]
        include: Vec<String>,

        /// Set the output directory
        #[arg(short, long)]
        out: Option<String>,

        /// Uses default output (Same directory as input) instead of config value
        #[arg(short, long)]
        default_out: bool,
    },
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    Set { key: OsString, value: OsString },
    Add { key: OsString, value: OsString },
    Remove { key: OsString, value: OsString },
    List,
}
