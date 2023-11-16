// Directory Manager
// Author: Derek Blaney

#![allow(dead_code, unused_imports)]

use std::ffi::OsString;
use std::fmt::format;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::ops::{Add, Deref};
use error::Error;
use std::path::{Path, PathBuf, StripPrefixError};
use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime};
use zip::write::FileOptions;
use crate::error;
use zip::{ZipWriter};
use inline_colorization::*;
use crate::config::Config;
use std::string::String;
use chrono::{DateTime, Utc};

pub struct Directory {
    pub location: PathBuf,
    name: OsString,
    zip: ZipWriter<File>,
    config: Config,
    verbose: bool,
    exclude: Vec<String>,
    include: Vec<String>,
    count: i32
}

impl Directory {
    pub fn new(location: &OsString, name: &Option<OsString>, config: Config, verbose: bool, exclude: Vec<String>, include: Vec<String>) -> Result<Directory, Error> {

        let path = Path::new(&location);
        if !&path.exists() { return Err(Error::PathNotFound); }
        if !&path.is_dir() { return Err(Error::PathNotDir); }

        let date: DateTime<Utc> = SystemTime::now().into();
        let mut zip_name: String = match name {
            Some(text) => text.clone().into_string().unwrap_or_default(),
            None => "mia_zip".to_owned()
        };
        zip_name = config.naming
            .replace(":name", &zip_name)
            .replace(":date", &date.format("%Y-%m-%d").to_string());
        zip_name = zip_name.add(".zip");

        println!("Name is {:?}", &zip_name);

        let zip_path = &path.join(&zip_name);
        let zip_file = File::create(zip_path).map_err(|e| Error::ZipFileFail(Some(e)))?;
        let zip = ZipWriter::new(zip_file);


        Ok(Directory {
            location: path.to_owned(),
            name: zip_name.into(),
            zip,
            config,
            verbose,
            exclude,
            include,
            count: 0
        })
    }

    pub fn zip_it(&mut self) -> Result<(), Error> {
        let mut directory: &mut Directory = self;
        let location = &*directory.location;

        let start = Instant::now();
        println!("Zipping dir {color_cyan}{:?}{color_reset}", &location);
        if directory.verbose {
            println!("Excluding: {:?}", directory.exclude);
            println!("Including: {:?}", directory.include);
        }
        directory.add_to_zip(&location.to_path_buf())?;
        directory.zip.finish().map_err(|_| Error::ZipFileFail(None))?;
        let elapsed = start.elapsed().as_millis();
        println!("Zipped {color_cyan}{}{color_reset} files in {color_cyan}{}ms{color_reset}", directory.count, elapsed);
        Ok(())
    }

    pub fn add_to_zip(&mut self, location: &Path) -> Result<(), Error> {
        let paths = fs::read_dir(location).expect("TODO: ERROR MESSAGE");
        for path in paths.flatten() {
            let location = &path.path();
            let mut file_name = location.file_name().unwrap_or_default().to_os_string().into_string().unwrap_or_default().to_ascii_lowercase();
            let file_extension = location.extension().unwrap_or_default().to_os_string().into_string().unwrap_or_default().to_ascii_lowercase();

            if self.exclude.contains(&file_name) && !self.include.contains(&file_name) { continue; }

            if location.is_dir() {
                if self.config.blacklisted_folder_names.contains(&file_name) && !self.include.contains(&file_name) { continue; }
                self.add_to_zip(location)?;
                if self.verbose { println!("Reading dir {color_cyan}{:?}{color_reset}", location); }
            } else if location.is_file() {
                file_name = file_name.replace(&".".to_string().add(&file_extension), "");

                let content = fs::read(location).map_err(|_| Error::CantReadFile)?;
                let stripped_path = location.strip_prefix(&self.location).unwrap().to_path_buf().into_os_string().into_string().unwrap();

                if self.config.blacklisted_file_names.contains(&file_name) && !self.include.contains(&file_name) { continue; }
                if self.config.blacklisted_file_extensions.contains(&file_extension) && !self.include.contains(&file_extension) { continue; }

                self.zip.start_file(&stripped_path, FileOptions::default());
                self.zip.write_all(&content);
                if self.verbose { println!("Reading file: {color_cyan}{:?}{color_reset}", &stripped_path); }
                self.count += 1;
            }
        }
        Ok(())
    }
}