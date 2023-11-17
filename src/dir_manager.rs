// Directory Manager
// Author: Derek Blaney

use std::collections::HashMap;
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

const LANGUAGES: [&str; 2] = ["py", "cs"];

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
    out: Option<String>
}

impl Directory {
    pub fn new(
        location: &OsString,
        name: &Option<OsString>,
        config: Config,
        verbose: bool,
        exclude: Vec<String>,
        include: Vec<String>,
        out: Option<String>
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

        let map = HashMap::from([
            (":name", zip_name.clone()),
            (":date", date.format("%Y-%m-%d").to_string()),
        ]);


        zip_name = config.naming.clone();
        for (key, value) in map {
            zip_name = zip_name.replace(key, &value);
        }
        zip_name = zip_name.add(".zip");

        let save_path = if let Some(out) = out {
            let p = Path::new(&out).to_path_buf();
            fs::create_dir_all(&p).map_err(Error::IO)?;
            if !p.exists() { path.to_owned() }  else { p }
        } else {
            path.to_owned()
        };

        let zip_path = &save_path.join(&zip_name);
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
            out: zip_path.clone().into_os_string().into_string().ok()
        })
    }

    // Zip the directory (initial action)
    pub fn zip_it(&mut self) -> Result<(), Error> {
        let start = Instant::now();
        if self.verbose {
            println!("--------------------------------------");
            println!("Zipping: {color_cyan}{:?}{color_reset}", self.name);
            println!("Output: {color_cyan}{:?}{color_reset}", self.out.clone().unwrap_or_default());
            let excluding = [&self.config.blacklisted_file_extensions, &self.config
                .blacklisted_file_names, &self.config.blacklisted_folder_names, &self.exclude];
            println!("Excluding: {:?} (Use --exclude or -e)", excluding);
            println!("Including: {:?} (Use --include or -i)", self.include);
            println!("--------------------------------------");
        }
        self.add_to_zip(&self.location.to_owned())?;
        self.zip.finish().map_err(Error::ZipFileFail)?;
        let elapsed = start.elapsed().as_millis();
        println!(
            "Zipped {color_cyan}{}{color_reset} files in {color_cyan}{}ms{color_reset}",
            self.count, elapsed
        );
        if self.verbose { println!("--------------------------------------"); }
        Ok(())
    }

    // Add directory to zip for iteration
    fn add_to_zip(&mut self, location: &Path) -> Result<(), Error> {
        let paths = fs::read_dir(location).expect("TODO: ERROR MESSAGE");
        for path in paths.flatten() {
            let location = &path.path();
            let mut file_name = os_string_to_lower_string(location.file_name());
            let file_extension = os_string_to_lower_string(location.extension());

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
                    println!("[DIR] {color_green}+{color_reset} {color_cyan}{:?}{color_reset}",
                             location);
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
                    let mut lines = 0;
                    if LANGUAGES.contains(&&*file_extension) {
                        let text_content = std::str::from_utf8(&content);
                        if let Ok(text) = text_content {
                            lines = text.lines().count();
                        }
                    }
                    let lines_text = if lines > 0 { format!("({lines} lines)") } else { ""
                        .to_string() };
                    println!(
                        "[FILE] {color_green}+{color_reset} {color_cyan}{:?}{color_reset} \
                        {color_yellow}{lines_text}{color_reset}",
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
