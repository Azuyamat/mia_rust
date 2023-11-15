mod cli;
mod dir_manager;
mod error;

use clap::Parser;
use cli::{
    Args,
    Zip
};
use crate::dir_manager::Directory;
use crate::error::Error;

fn main() -> Result<(), Error> {
    let args = Args::parse();

    match args.zip {
        Zip::Create { location, name } => {
            let directory = Directory::new(&location, &name)?;
        }
    }

    Ok(())
}
