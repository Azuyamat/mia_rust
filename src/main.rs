// Mia Command Line Interface
// Author: Derek Blaney

mod cli;
mod config;
mod dir_manager;
mod error;

use crate::cli::ConfigAction;
use crate::dir_manager::Directory;
use crate::error::Error;
use clap::Parser;
use cli::{Args, Zip};
use inline_colorization::*; // This IS used. IDE doesn't detect it.

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let mut config: config::Config = confy::load("mia", None).map_err(Error::Config)?;

    match args.zip {
        Zip::Create {
            location,
            name,
            verbose,
            exclude,
            include,
            out
        } => {
            let output_dir = if out.is_none() {
                config.output_dir.clone()
            } else {
                out
            };
            let mut directory =
                Directory::new(&location, &name, config, verbose, exclude, include, output_dir)?;
            directory.zip_it()?;
        }
        Zip::Config { action } => {
            match action {
                ConfigAction::Set { key, value } => {
                    let string_key = key.clone().into_string().unwrap().to_ascii_lowercase();
                    let string_value = value.into_string().unwrap();
                    config.set_key(&string_key, string_value.clone())?;
                    println!("Successfully changed key {color_cyan}`{string_key}`{color_reset} to \
                    {color_cyan}`{string_value}`{color_reset}")
                }
                ConfigAction::Add { key, value } => {
                    let string_key = key.clone().into_string().unwrap().to_ascii_lowercase();
                    let string_value = value.into_string().unwrap();
                    config.add_value(&string_key, string_value.clone())?;
                    println!("Successfully added value {color_cyan}`{string_value}`{color_reset} \
                    to {color_cyan}`{string_key}`{color_reset}");
                }
                ConfigAction::Remove { key, value } => {
                    let string_key = key.clone().into_string().unwrap().to_ascii_lowercase();
                    let string_value = value.into_string().unwrap();
                    config.remove_value(&string_key, string_value.clone())?;
                    println!("Successfully removed value {color_cyan}`{string_value}`{color_reset} \
                    from {color_cyan}`{string_key}`{color_reset}");
                }
                ConfigAction::List => {
                    print_pretty_header("Config List", 4);
                    println!(" - Naming: {}", config.naming);
                    println!(" ↳ {color_cyan}mia config set naming <format>{color_reset}");
                    println!(" - Output Dir: {}", config.output_dir.clone().unwrap_or("Not set"
                        .to_string
                    ()));
                    println!(" ↳ {color_cyan}mia config set output_dir <format>{color_reset}");
                    println!(
                        " - Blacklisted file names: {:?}",
                        config.blacklisted_file_names
                    );
                    println!(" ↳ {color_cyan}mia config add/remove blacklisted_file_names <key> <value>{color_reset}");
                    println!(
                        " - Blacklisted folder names: {:?}",
                        config.blacklisted_folder_names
                    );
                    println!(" ↳ {color_cyan}mia config add/remove blacklisted_folder_names <key> <value>{color_reset}");
                    println!(
                        " - Blacklisted file extensions: {:?}",
                        config.blacklisted_file_extensions
                    );
                    println!(" ↳ {color_cyan}mia config add/remove blacklisted_file_extensions <key> <value>{color_reset}");
                }
            }

            // Save config
            confy::store("mia", None, config).map_err(Error::Config)?;
        }
    }

    Ok(())
}

pub fn print_pretty_header(text: &str, padding: usize) {
    let padding_text = " ".repeat(padding);
    println!("{}", "=".repeat(text.len() + (padding * 2)));
    println!("{padding_text}{color_cyan}{text}{color_reset}{padding_text}");
    println!("{}", "=".repeat(text.len() + (padding * 2)));
}
