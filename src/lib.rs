use std::fs;

use clap::Parser;
use cli::{Cli, Command};

mod cli;

pub fn run() {
    let args = Cli::parse();
    match args.command {
        Some(Command::Average(avg_args)) => run_average(&avg_args.directory, avg_args.verbose),
        None => run_default(&args.common.directory, args.common.verbose),
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

fn run_default(directory: &str, verbose: bool) {
    if verbose {
        println!("Searching: {}\n", directory);
    }

    let paths = fs::read_dir(&directory);
    if let Err(e) = paths {
        eprintln!("Unable to read directory {}", directory);
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

fn run_average(directory: &str, verbose: bool) {
    if verbose {
        println!("Searching: {}\n", directory);
    }

    let paths = fs::read_dir(&directory);
    if let Err(e) = paths {
        eprintln!("Unable to read directory {}", directory);
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }

    let mut apertures: Vec<f64> = Vec::new();
    let mut focal_lens: Vec<f64> = Vec::new();
    let mut isos: Vec<i32> = Vec::new();
    // let mut shutter_speeds: Vec<String> = Vec::new();

    for path in paths.unwrap() {
        let path_string = path.unwrap().path().display().to_string();
        let file_infos = extract_file_info(&path_string);
        for info in file_infos.iter() {
            if let Some(a) = info.aperture {
                if verbose {
                    println!("Aperture for {} is {}", path_string, a)
                }
                apertures.push(a)
            }
            if let Some(fl) = info.focal_length {
                if verbose {
                    println!("Focal length for {} is {}", path_string, fl)
                }
                focal_lens.push(fl)
            }
            if let Some(iso) = info.iso {
                if verbose {
                    println!("ISO for {} is {}", path_string, iso)
                }
                isos.push(iso)
            }
            // if let Some(ss) = &info.shutter_speed {
            //     if verbose {
            //         println!("Shutter speed for {} is {}", path_string, ss)
            //     }
            //     shutter_speeds.push(ss.to_string())
            // }
            if verbose {
                println!()
            }
        }
    }

    println!(
        "Average aperture: {}",
        apertures.iter().sum::<f64>() / apertures.len() as f64
    );
    println!(
        "Average focal length: {}",
        focal_lens.iter().sum::<f64>() / focal_lens.len() as f64
    );
    println!(
        "Average ISO: {}",
        isos.iter().sum::<i32>() / isos.len() as i32
    );
    // println!(
    //     "Average shutter speed: {}",
    //     shutter_speeds.iter().sum() / shutter_speeds.len()
    // );
}
