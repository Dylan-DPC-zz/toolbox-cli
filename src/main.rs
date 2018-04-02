#[macro_use] extern crate quicli;
use quicli::prelude::*;
use scraper::Downloader;
mod scraper;
use std::process::Output;

#[derive(Debug, StructOpt)]
struct Toolbox {
    tool: String,
    #[structopt(long = "version", short = "v", default_value="latest")]
    version: String

}

main!(|args: Toolbox| {
    let version = match args.version {
        x => Some(x),
        _ => None
    };

    match args.tool.as_str() {
        "phpstorm" => PhpStorm::new(version).download(),
        _ => panic!("invalid tool entered or not supported yet")
    };

});

pub struct PhpStorm {
    version: Option<String>
}

impl PhpStorm {
    fn new(version: Option<String>) -> PhpStorm {
        PhpStorm { version }
    }
}

pub trait Tool {
    fn download(&self) -> Output;

}

impl Tool for PhpStorm {
    fn download(&self) -> Output {
        Downloader::from(scraper::Platform::Linux)
            .download("https://download.jetbrains.com/webide/PhpStorm-2018.1.tar.gz")

    }
}





