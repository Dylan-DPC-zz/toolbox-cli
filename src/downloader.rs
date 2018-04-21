use std::process::{Command, Output};
use std::path::PathBuf;
use std::fs;

pub struct Downloader {
    directory: PathBuf
}

impl Downloader {
    pub fn from(platform: Platform, directory: PathBuf) -> Downloader {
        Downloader { directory }
    }

    pub fn download(&self, url: &str) -> Output {

        if !self.directory.is_dir() {
            println!("{:?}", fs::create_dir(&self.directory));
        }

        Command::new("wget")
            .args(&[url])
            .current_dir(&self.directory)
            .output()
            .expect("unable to fetch. try again!")
    }

}

pub enum Platform {
    Linux,
}