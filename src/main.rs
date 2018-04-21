#[macro_use] extern crate quicli;
use quicli::prelude::*;
use downloader::Downloader;
mod downloader;
use std::process::Output;
use std::path::PathBuf;

#[derive(Debug, StructOpt)]
struct Toolbox {
    tool: String,
    #[structopt(long = "version", short = "v", default_value="latest")]
    version: String,
    #[structopt(long = "directory", short = "d", default_value=".")]
    directory: PathBuf

}

main!(|args: Toolbox| {
    let version = match args.version {
        x => Some(x),
        _ => None
    };



    match args.tool.as_str() {
        "phpstorm" => PhpStorm::new(version, args.directory).download(),
        _ => panic!("invalid tool entered or not supported yet")
    };

});

pub struct PhpStorm {
    version: Option<String>,
    directory: PathBuf
}

impl PhpStorm {
    fn new(version: Option<String>, directory: PathBuf) -> PhpStorm {
        PhpStorm { version, directory }
    }
}

pub trait Tool {
    fn download(&self) -> Output;

}

impl Tool for PhpStorm {
    fn download(&self) -> Output {
        Downloader::from(downloader::Platform::Linux, self.directory.to_owned())
            .download("https://download.jetbrains.com/webide/PhpStorm-2018.1.tar.gz")

    }
}





