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
use std::io::{Write};
use std::ops::{Add};
use std::path::{Path, PathBuf};
use std::string::String;
use std::time::{Instant, SystemTime};
use zip::write::FileOptions;
use zip::ZipWriter;
use crate::languages::{detect_language, Language};

pub struct Directory {
    pub location: PathBuf,
    name: OsString,
    zip: ZipWriter<File>,
    config: Config,
    verbose: bool,
    exclude: Vec<String>,
    include: Vec<String>,
    count: i32,
    out: Option<String>,
    lines: HashMap<Language, i128>
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
            fs::create_dir_all(&p)?;
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
            out: zip_path.clone().into_os_string().into_string().ok(),
            lines: HashMap::new()
        })
    }

    // Zip the directory (initial action)
    pub fn zip_it(&mut self) -> Result<(), Error> {
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
        let start = Instant::now();
        self.add_to_zip(&self.location.to_owned())?;
        self.zip.finish().map_err(Error::ZipFileFail)?;
        let elapsed = start.elapsed().as_millis();
        let line_count = &self.lines
            .iter()
            .filter(|(language, _)| *language != &Language::None)
            .map(|(_, &count)| count)
            .sum::<i128>();
        println!(
            "Zipped {color_cyan}{}{color_reset} files in {color_cyan}{}ms{color_reset} ({color_cyan}{line_count}{color_reset} lines)",
            self.count, elapsed
        );
        if self.verbose {
            println!("--------------------------------------");
            for (&lang, &count) in &self.lines {
                if lang == Language::None {
                    println!("Other: {color_cyan}{count}{color_reset} lines");
                    continue;
                }
                let percentage: f64 = ((count as f64/(*line_count) as f64)*10000.0).round()/100.0;
                println!("{lang:?}: {color_cyan}{count}{color_reset} lines ({percentage}%)")
            }
            println!("--------------------------------------");
        }
        Ok(())
    }

    // Add directory to zip for iteration
    fn add_to_zip(&mut self, location: &Path) -> Result<(), Error> {
        let paths = fs::read_dir(location)?;
        for path in paths.flatten() {
            let location = &path.path();
            let file_name = os_string_to_lower_string(location.file_name());
            let file_extension = os_string_to_lower_string(location.extension());

            if self.exclude.contains(&file_name) && !self.include.contains(&file_name) { continue; }

            if location.is_dir() {
                if self.config.blacklisted_folder_names.contains(&file_name) && !self.include.contains(&file_name) {
                    if self.verbose {
                        println!("[DIR] {color_yellow}/{color_reset} {color_cyan}{:?}{color_reset}",
                                 location);
                    }
                    continue;
                }
                self.add_to_zip(location)?;
                if self.verbose {
                    println!("[DIR] {color_green}+{color_reset} {color_cyan}{:?}{color_reset}",
                             location);
                }
            } else if location.is_file() {
                let mia_file: MiaFile = MiaFile::new (
                    file_name,
                    file_extension,
                    location
                );

                if self.config.blacklisted_file_names.contains(&mia_file.name) && !self.include.contains(&mia_file.name) { continue; }
                if self.config.blacklisted_file_extensions.contains(&mia_file.extension) && !self.include.contains(&mia_file.extension) { continue; }

                let stripped_path = location
                    .strip_prefix(&self.location)
                    .unwrap()
                    .to_path_buf()
                    .into_os_string()
                    .into_string()
                    .unwrap();

                let content = mia_file.get_content()?;

                self.zip.start_file(&stripped_path, FileOptions::default())?;
                self.zip.write_all(&content)?;

                let lines = mia_file.count_lines();
                let entry = &mut self.lines.entry(lines.0);

                match entry {
                    std::collections::hash_map::Entry::Occupied(ref mut occupied) => {
                        // Increment the value if the key exists
                        *occupied.get_mut() += lines.1;
                    }
                    std::collections::hash_map::Entry::Vacant(_vacant) => {
                        // Add the key with value 1 if the key doesn't exist
                        self.lines.insert(lines.0, lines.1);
                    }
                }

                if self.verbose {
                    let lines_text = if lines.1 > 0 { format!("({} lines)", lines.1) } else { String::new() };
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

struct MiaFile {
    name: String,
    extension: String,
    location: PathBuf
}

impl MiaFile {
    fn new(name: String, extension: String, location: &PathBuf) -> Self {
        let name = name.replace(&".".to_string().add(&extension), "");
        MiaFile {
            name,
            extension,
            location: location.to_owned()
        }
    }

    fn get_content(&self) -> Result<Vec<u8>, Error> {
        fs::read(&self.location).map_err(|_| Error::CantReadFile)
    }

    fn count_lines(&self) -> (Language, i128) {
        let language = detect_language(&self.extension);
        let content = self.get_content().unwrap();
        let text_content = std::str::from_utf8(&content);
        if let Ok(text) = text_content {
            (language, text.lines().filter(|line| !line.trim().is_empty()).count() as i128)
        } else { (language, 0) }
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