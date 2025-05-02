use std::{fs, io, path::PathBuf};

use bytes::Bytes;
use clap::{CommandFactory, Parser, error::ErrorKind};
use net::{download_file, download_file_bytes};
use serde::Deserialize;
// use tokio_stream::{Stream, StreamExt};

pub mod net;

extern crate reqwest;

/// Simple program to download maps from .bplist file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Output dir
    #[arg(short, long)]
    output: Option<String>,

    /// Overwrite directory
    #[arg(long, default_value = "false")]
    overwrite: bool,

    /// Path to .bplist file
    #[arg()]
    file: PathBuf,
}

#[derive(Debug, Deserialize)]
struct Song {
    key: String,
    hash: String,
    songName: String,
}

#[derive(Debug, Deserialize)]
struct Maps {
    songs: Vec<Song>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // handle some human errors
    if !args.file.exists() {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::Io,
            format!("file at {:?} does not exist!", &args.file.as_os_str(),),
        )
        .exit();
    }

    if args.file.extension().unwrap() != "bplist" {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::Io,
            format!("file at {:?} is not a .bplist", &args.file.as_os_str(),),
        )
        .exit();
    }

    let file = fs::read(&args.file)?;

    let json: Maps = serde_json::from_slice(file.as_slice()).expect("JSON was not well-formatted");

    let current_dir = std::env::current_dir()?;

    let mut out_dir = current_dir.join(&args.file.file_stem().unwrap());
    if args.output.is_some() {
        out_dir = PathBuf::from(&args.output.clone().unwrap());
    }

    if out_dir.exists() {
        if !args.overwrite {
            let mut cmd = Args::command();
            cmd.error(
                ErrorKind::Io,
                format!("Output directory {out_dir:?} already exists, use --overwrite to continue anyway"),
            )
            .exit();
        }
    }

    println!("Collecting zip files to buffer");

    let mut zip_archives: Vec<Bytes> = Vec::new();

    // https://api.beatsaver.com/download/key/
    for song in &json.songs {
        println!("Downloading {}", song.songName);
        zip_archives.push(
            download_file_bytes(
                format!("https://api.beatsaver.com/download/key/{}", song.key).as_str(),
                // out_dir.join("zip").join(format!("{}.zip", song.key)),
            )
            .await?,
        )
    }

    println!("Unzipping files.");

    let mut i: usize = 0;
    while !zip_archives.is_empty() {
        let dir = out_dir.join(&json.songs[i].songName);
        fs::create_dir_all(&dir)?;
        // dir name is simply song's name
        let archive = io::Cursor::new(zip_archives.pop().unwrap());
        // creates a pointer to the last zip archive in buffer and removes it

        zip_extract::extract(archive, &dir, true)?;

        i += 1;
    }

    println!("\nEverything done! You can find your files in {out_dir:?}");
    Ok(())
}
