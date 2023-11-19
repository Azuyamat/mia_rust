use std::fs::File;
use serde::{Deserialize};
use crate::error::Error;
use std::io::copy;
use crate::print_pretty_header;

const OWNER: &str = "Azuyamat";
const REPO: &str = "mia_rust";
const ASSET_NAME: &str = "mia.exe";

#[derive(Debug, Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
    download_count: u128
}

#[derive(Debug, Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
    name: String,
    body: String
}

fn get_release_by_version(version: &str) -> Result<Release, Error> {
    let url = format!("https://api.github.com/repos/{OWNER}/{REPO}/releases/tags/{version}");

    let response = reqwest::blocking::Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, "mia_cli")
        .send()?
        .text()?;
    let release: Release = serde_json::from_str(&response)?;

    Ok(release)
}

pub fn get_latest_release() -> Result<String, Error> {
    let url = format!("https://api.github.com/repos/{OWNER}/{REPO}/releases/latest");

    let response = reqwest::blocking::Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, "mia_cli")
        .send()?
        .text()?;
    let release: Release = serde_json::from_str(&response)?;

    Ok(release.tag_name)
}

pub fn get_download_link_for_asset(version: &str) -> Result<String, Error> {
    let release = get_release_by_version(version)?;

    let download_count = release.assets[0].download_count;

    println!("{}", release.name);
    println!("{download_count} downloads");
    let notes = release.body.trim();
    if notes.is_empty() {
        println!("No notes");
    } else {
        print_pretty_header("Release notes", 4);
        println!("{}", release.body);
    }

    if let Some(asset) = release.assets.iter().find(|a| a.name == ASSET_NAME) {
        return Ok(asset.browser_download_url.clone());
    }

    Err("Asset not found or download URL not available".into())
}

pub fn download_asset(url: &str, destination: &mut File) -> Result<(), Error> {
    let response = reqwest::blocking::get(url)?;
    copy(&mut response.bytes().unwrap().as_ref(), destination)?;
    Ok(())
}