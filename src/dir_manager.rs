// Directory Manager
// Author: Derek Blaney

#![allow(dead_code, unused_imports)]

use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use error::Error;
use std::path::{Path, PathBuf, StripPrefixError};
use std::thread::sleep;
use std::time::Duration;
use zip::write::FileOptions;
use crate::error;
use zip::ZipWriter;
use inline_colorization::*;

pub struct Directory {
    pub location: PathBuf,
    name: OsString,
    zip: ZipWriter<File>
}

impl Directory {
    pub fn new(location: &OsString, name: &Option<OsString>) -> Result<Directory, Error> {
        let path = Path::new(&location);
        if !&path.exists() { return Err(Error::PathNotFound); }
        if !&path.is_dir() { return Err(Error::PathNotDir); }

        let mut zip_name: OsString = match name {
            Some(text) => text.clone(),
            None => OsString::from("mia_zip")
        };
        zip_name.push(".zip");

        let zip_path = &path.join(&zip_name);
        let zip_file = File::create(&zip_path).map_err(|e| Error::ZipFileFail(Some(e)))?;
        let mut zip = ZipWriter::new(zip_file);


        Ok(Directory {
            location: path.to_owned(),
            name: zip_name,
            zip
        })
    }

    pub fn zip_it(&mut self) -> Result<(), Error> {
        let mut directory: &mut Directory = self;
        let location = &*directory.location;

        println!("Zipping dir {:?}", &location);
        directory.add_to_zip(&location.to_path_buf())?;
        directory.zip.finish().map_err(|_| Error::ZipFileFail(None))?;
        Ok(())
    }

    pub fn add_to_zip(&mut self, location: &PathBuf) -> Result<(), Error> {
        let paths = fs::read_dir(&location.as_path()).unwrap();

        for path in paths {
            if let Ok(path) = path {
                let location = &path.path();
                if location.is_dir() {
                    &self.add_to_zip(location);
                    println!("Reading dir {color_cyan}{:?}{color_reset}", location);
                } else if location.is_file() {
                    let content = fs::read(&location).map_err(|_| Error::CantReadFile)?;
                    let stripped_path = location.strip_prefix(&self.location).unwrap().to_path_buf().into_os_string().into_string().unwrap();

                    &self.zip.start_file(&stripped_path, FileOptions::default());
                    &self.zip.write_all(&*content);
                    println!("Reading file: {color_cyan}{:?}{color_reset}", &stripped_path);
                }
            }
        }
        Ok(())
    }
}