use std::path::PathBuf;
use std::fs::{File, metadata};
use std::process;
use std::error::Error;
use std::io::Write;
use core::result::Result;
use bytes::Bytes;
use chrono::{DateTime, Local};
use reqwest::blocking::get;
use clap::Arg;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli: clap::ArgMatches = clap::Command::new("tapmusic-cli")

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

    .arg(
        Arg::new("filename")
            .short('f')
            .default_value("")
            .help("Save collage under a custom file name.")
    )
    .get_matches();

    let user: &String = cli.get_one::<String>("user").expect("USER is required");
    let size: &String = cli.get_one::<String>("size").expect("SIZE is required");
    let time: &String = cli.get_one::<String>("time").expect("TIME is required");
    let directory: &PathBuf = cli.get_one::<PathBuf>("directory").expect("DIRECTORY is required");
    let mut caption: Option<&String> = cli.get_one::<String>("caption");
    let mut playcount: Option<&String> = cli.get_one::<String>("playcount");
    let mut filename: Option<&String> = cli.get_one::<String>("filename");

    let new_size: String = format!("{}x{}", size, size);
    let new_time: String = parse_time(time);
    let new_caption: String = str_to_bool(caption.take().unwrap());
    let new_playcount: String = str_to_bool(playcount.take().unwrap());

    let url: String = build_url(user, &new_size, &new_time, &new_caption, &new_playcount);
    let new_filename: String = parse_file_name(user, &new_size, &new_time, filename.take().unwrap());
    let mut directory = directory.clone();
    directory.push(new_filename);

    if file_exists(&directory) == true {
        
        process::exit(1);
    } else {
        _ = get_collage(&url, directory);
    }

    Ok(())
}

//Check if collage download path exists

fn file_exists(path: &PathBuf) -> bool {
    metadata(path).is_ok()
}

fn parse_file_name(user: &str, size: &str, time: &str, filename: &str) -> String {
    if filename == "" {
        let now: DateTime<Local> = Local::now();
        format!("{}_{}_{}_{}.jpg", user, time, size, now.format("%Y-%m-%d_%H%M%S"))
    } else {
        filename.to_string()
    }
}

fn parse_time(time: &str) -> String {
    match time.chars().last().unwrap() {
        'd' => format!("{}day", &time[..time.len() - 1]),
        'm' => format!("{}month", &time[..time.len() - 1]),
        _ => "overall".to_string(),
    }
}

fn str_to_bool(s: &String) -> String {
    match s.chars().last().unwrap() {
        't' => "true".to_string(),
        _ => "false".to_string(),
    }
}

fn build_url(user: &str, size: &str, time: &str, caption: &str, playcount: &str) -> String {
    let mut base_url: String = format!("https://tapmusic.net/collage.php?user={}&type={}&size={}", &user, &time, &size);

    if caption == "true" {
        base_url = format!("{}&caption={}", base_url, caption);
    } else {()}

    if playcount == "true" {
        base_url = format!("{}&playcount={}", base_url, playcount);
    } else {()}

    base_url
}

fn get_collage(url: &str, file_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let res: reqwest::blocking::Response = get(url)?;

    if res.status().is_success() {
        let bytes: Bytes = res.bytes()?;
        write_collage(&bytes, file_path)?;
        Ok(())
    } else {
        Err("HTTP request failed".into())
    }
}

fn write_collage(context: &Bytes, file_path: PathBuf) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file_path)?;
    file.write_all(context)?;
    Ok(())
} 