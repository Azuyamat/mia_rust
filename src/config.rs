// Config Manager
// Author: Derek Blaney

use crate::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub naming: String, // Follows format `:name` by default
    pub blacklisted_file_names: Vec<String>,
    pub blacklisted_folder_names: Vec<String>,
    pub blacklisted_file_extensions: Vec<String>,

    pub output_dir: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            naming: ":name".to_string(),
            blacklisted_file_names: Vec::new(),
            blacklisted_folder_names: vec![
                ".git".to_string(),
                "bin".to_string(),
                "obj".to_string(),
                ".idea".to_string(),
                ".vs".to_string(),
            ],
            blacklisted_file_extensions: vec!["zip".to_string(), "pdf".to_string()],
            output_dir: None,
        }
    }
}

impl Config {
    pub fn set_key(&mut self, key: &str, value: String) -> Result<(), Error> {
        match key {
            "naming" => {
                self.naming = value;
            }
            "output_dir" => {
                self.output_dir = Some(value);
            }
            _ => {
                return Err(Error::ConfigActionError(
                    "Can't set key to value".to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn add_value(&mut self, key: &str, value: String) -> Result<(), Error> {
        match key {
            "blacklisted_file_names" => {
                self.blacklisted_file_names.push(value);
            }
            "blacklisted_folder_names" => {
                self.blacklisted_folder_names.push(value);
            }
            "blacklisted_file_extensions" => {
                self.blacklisted_file_extensions.push(value);
            }
            _ => {
                return Err(Error::ConfigActionError("Couldn't find key".to_string()));
            }
        }
        Ok(())
    }

    pub fn remove_value(&mut self, key: &str, value: String) -> Result<(), Error> {
        match key {
            "blacklisted_file_names" => {
                let index = find_position_in_vec(&self.blacklisted_file_names, value)?;
                self.blacklisted_file_names.remove(index);
            }
            "blacklisted_folder_names" => {
                let index = find_position_in_vec(&self.blacklisted_folder_names, value)?;
                self.blacklisted_folder_names.remove(index);
            }
            "blacklisted_file_extensions" => {
                let index = find_position_in_vec(&self.blacklisted_file_extensions, value)?;
                self.blacklisted_file_extensions.remove(index);
            }
            _ => {
                return Err(Error::ConfigActionError("Couldn't find key".to_string()));
            }
        }
        Ok(())
    }
}

fn find_position_in_vec(vec: &[String], value: String) -> Result<usize, Error> {
    let index = vec
        .iter()
        .position(|x| *x == value);
    match index {
        Some(index) => Ok(index),
        None => Err(Error::ConfigActionError("Couldn't find value in vec".to_string()))
    }
}
