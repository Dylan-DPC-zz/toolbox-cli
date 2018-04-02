use std::process::{Command, Output};

pub struct Downloader {

}

impl Downloader {
    pub fn from(platform: Platform) -> Downloader {
        Downloader {}
    }

    pub fn download(&self, url: &str) -> Output {
        Command::new("wget")
            .args(&[url])
            .output()
            .expect("unable to fetch. try again!")
    }

}

pub enum Platform {
    Linux,
}