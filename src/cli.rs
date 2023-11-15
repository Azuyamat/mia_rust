// Command Line Interface Manager
// Author: Derek Blaney

use std::ffi::OsString;
use clap::{
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub(crate) zip: Zip
}

#[derive(Subcommand, Debug)]
pub enum Zip {
    Create {
        /// The folder location
        location: OsString,
        /// The name given to the zip file
        name: Option<OsString>
    }
}