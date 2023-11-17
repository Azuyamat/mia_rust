// Directory Manager
// Author: Derek Blaney

use crate::config::Config;
use crate::error;
use chrono::{DateTime, Utc};
use error::Error;
use inline_colorization::*;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::ops::Add;
use std::path::{Path, PathBuf};
use std::string::String;
use std::time::{Instant, SystemTime};
use zip::write::FileOptions;
use zip::ZipWriter;

#[allow(dead_code)]
pub struct Directory {
    pub location: PathBuf,
    name: OsString,
    zip: ZipWriter<File>,
    config: Config,
    verbose: bool,
    exclude: Vec<String>,
    include: Vec<String>,
    count: i32,
}

impl Directory {
    pub fn new(
        location: &OsString,
        name: &Option<OsString>,
        config: Config,
        verbose: bool,
        exclude: Vec<String>,
        include: Vec<String>,
    ) -> Result<Directory, Error> {
        let path = Path::new(&location);
        if !&path.exists() {
            return Err(Error::PathNotFound);
        }
        if !&path.is_dir() {
            return Err(Error::PathNotDir);
        }

        let date: DateTime<Utc> = SystemTime::now().into();
        let mut zip_name: String = match name {
            Some(text) => text.clone().into_string().unwrap_or_default(),
            None => "mia_zip".to_owned(),
        };
        zip_name = config
            .naming
            .replace(":name", &zip_name)
            .replace(":date", &date.format("%Y-%m-%d").to_string());
        zip_name = zip_name.add(".zip");

        println!("Name is {:?}", &zip_name);

        let zip_path = &path.join(&zip_name);
        let zip_file = File::create(zip_path).map_err(Error::IO)?;
        let zip = ZipWriter::new(zip_file);

        Ok(Directory {
            location: path.to_owned(),
            name: zip_name.into(),
            zip,
            config,
            verbose,
            exclude,
            include,
            count: 0,
        })
    }

    // Zip the directory (initial action)
    pub fn zip_it(&mut self) -> Result<(), Error> {
        let start = Instant::now();
        if self.verbose {
            println!("Excluding: {:?}", self.exclude);
            println!("Including: {:?}", self.include);
        }
        self.add_to_zip(&self.location.to_owned())?;
        self.zip.finish().map_err(Error::ZipFileFail)?;
        let elapsed = start.elapsed().as_millis();
        println!(
            "Zipped {color_cyan}{}{color_reset} files in {color_cyan}{}ms{color_reset}",
            self.count, elapsed
        );
        Ok(())
    }

    // Add directory to zip for iteration
    fn add_to_zip(&mut self, location: &Path) -> Result<(), Error> {
        let paths = fs::read_dir(location).expect("TODO: ERROR MESSAGE");
        for path in paths.flatten() {
            let location = &path.path();
            let mut file_name = os_string_to_lower_string(self.location.file_name());
            let file_extension = os_string_to_lower_string(self.location.extension());

            if self.exclude.contains(&file_name) && !self.include.contains(&file_name) {
                continue;
            }

            if location.is_dir() {
                if self.config.blacklisted_folder_names.contains(&file_name)
                    && !self.include.contains(&file_name)
                {
                    continue;
                }
                self.add_to_zip(location)?;
                if self.verbose {
                    println!("Reading dir {color_cyan}{:?}{color_reset}", location);
                }
            } else if location.is_file() {
                file_name = file_name.replace(&".".to_string().add(&file_extension), "");

                let content = fs::read(location).map_err(|_| Error::CantReadFile)?;
                let stripped_path = location
                    .strip_prefix(&self.location)
                    .unwrap()
                    .to_path_buf()
                    .into_os_string()
                    .into_string()
                    .unwrap();

                if self.config.blacklisted_file_names.contains(&file_name)
                    && !self.include.contains(&file_name)
                {
                    continue;
                }
                if self
                    .config
                    .blacklisted_file_extensions
                    .contains(&file_extension)
                    && !self.include.contains(&file_extension)
                {
                    continue;
                }

                self.zip
                    .start_file(&stripped_path, FileOptions::default())?;
                self.zip.write_all(&content)?;

                if self.verbose {
                    println!(
                        "Reading file: {color_cyan}{:?}{color_reset}",
                        &stripped_path
                    );
                }
                self.count += 1;
            }
        }
        Ok(())
    }
}

fn os_string_to_lower_string(os_string: Option<&OsStr>) -> String {
    match os_string {
        Some(text) => text
            .to_os_string()
            .into_string()
            .unwrap()
            .to_ascii_lowercase(),
        None => "".to_string(),
    }
}
