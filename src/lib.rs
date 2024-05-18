use std::fs;

use clap::Parser;
use cli::Args;

mod cli;

pub fn run() {
    let args = Args::parse();

    if args.verbose {
        println!("Searching: {}", args.directory);
    }

    let paths = fs::read_dir(&args.directory);
    if let Err(e) = paths {
        eprintln!("Unable to read directory {}", args.directory);
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    for path in paths.unwrap() {
        let path_string = path.unwrap().path().display().to_string();
        let file_infos = extract_file_info(&path_string);
        for info in file_infos.iter() {
            print_info(info, &path_string);
        }
    }
}

struct FileInfo {
    /// Aperture, also known as "f-stop"
    aperture: Option<f64>,
    /// Focal length
    focal_length: Option<f64>,
    /// ISO
    iso: Option<i32>,
    /// Shutter speed, also known as "exposure time"
    shutter_speed: Option<String>,
}

fn extract_file_info(path: &str) -> Option<FileInfo> {
    let meta = rexiv2::Metadata::new_from_path(path);
    meta.ok().map(|m| FileInfo {
        aperture: m.get_fnumber(),
        focal_length: m.get_focal_length(),
        iso: m.get_iso_speed(),
        shutter_speed: m.get_exposure_time().map(|et| et.to_string()),
    })
}

fn print_info(info: &FileInfo, path: &str) {
    println!("File: {}", path);
    match info.aperture {
        Some(a) => {
            println!("Aperture: {}", a);
        }
        None => {
            eprintln!("Unable to find aperature")
        }
    }
    match info.focal_length {
        Some(f) => {
            println!("Focal length: {}mm", f)
        }
        None => {
            eprintln!("Unable to find focal length")
        }
    }
    match info.iso {
        Some(iso) => {
            println!("ISO: {}", iso)
        }
        None => {
            eprintln!("Unable to find ISO")
        }
    }
    match &info.shutter_speed {
        Some(speed) => {
            println!("Shutter speed: {}s", speed)
        }
        None => {
            eprintln!("Unable to find shutter speed")
        }
    }
    println!()
}
