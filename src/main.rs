use std::path::PathBuf;
use std::fs;
use std::io::Read;
use std::result::Result;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use reqwest::blocking::get;
use clap::{Arg, builder::PossibleValuesParser, ArgAction};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = clap::Command::new("tapmusic-cli")
    .arg(
        Arg::new("user")
            .help("Your last.fm username.")
            .required(true)

    )

    .arg(
        Arg::new("size")
            .value_parser(["3", "4", "5", "10"])
            .help("Collage size.")
            .required(true)
    )
    .arg(
        Arg::new("time")
            .value_parser(["7d", "1m", "3m", "6m", "12m", "all"])
            .help("Time period of your last.fm history.")
            .required(true)
    )
    
    .arg(
        Arg::new("directory")
            .value_parser(clap::value_parser!(PathBuf))
            .help("directory where the collage will be saved at.")
            .required(true)
    )

    .arg(
        Arg::new("caption")
            .short('c')
            .value_parser(["t", "f"])
            .default_value("t")
            .help("Display album/artist captions in collage.")
    )
    .arg(
        Arg::new("playcount")
            .short('p')
            .value_parser(["t", "f"])
            .default_value("t")
            .help("Display album/artist playcount in collage")
            
    )
    .get_matches();

    // Retrieve and print the values
    let user = cli.get_one::<String>("user").expect("USER is required");
    let size = cli.get_one::<String>("size").expect("SIZE is required");
    let time = cli.get_one::<String>("time").expect("TIME is required");
    let directory = cli.get_one::<PathBuf>("directory").expect("DIRECTORY is required");
    let caption = cli.get_one::<String>("caption");
    let playcount = cli.get_one::<String>("playcount");

    get_collage();
}

fn file_exists(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    fs::metadata(path).is_ok()
}

fn get_collage(url: &str) -> None {
    let url1 = "https://tapmusic.net/collage.php?user=soloroh&type=7day&size=4x4&caption=true";

    let mut res = get(url1)?;

    if res.status().is_success() {
        // Get the response body as bytes
        let mut context = res.bytes()?;

        let path = Path::new("rust_repo.jpg");
        let mut file = File::create(&path)?;

        file.write_all(&context)?;

    } else {
        return Err(reqwest::Error::new(
            reqwest::StatusCode::BAD_REQUEST,
            "Failed to get a successful response".into(),
        ))
    }

    Ok(())
}