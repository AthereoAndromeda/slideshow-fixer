use std::{fs, path::Path};

use clap::Parser;
use slideshow_fixer::write_files;

/// A simple utility program to fix slideshow sorting order cuz our TV is shit
#[derive(Debug, Parser)]
struct Args {
    /// Path of folder
    path: String,

    /// Optional output folder. Default to <PATH>-1
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();
    // println!("{}", args.path);

    let mut entries = Vec::new();
    let input_dir = fs::read_dir(&args.path);

    // Check if dir exists
    let input_dir = match input_dir {
        Ok(f) => f,
        Err(e) => {
            panic!("{}", e);
            // return;
        }
    };

    for file in input_dir {
        match file {
            Ok(f) => entries.push(f),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    // sort numerically
    entries.sort_by(|a, b| a.path().cmp(&b.path()));

    // Remove trailing /
    let arg_path = match &args.path.strip_suffix("/") {
        Some(p) => *p,
        None => &args.path,
    };

    println!("{:?}", &arg_path);
    let output_path = args.output.unwrap_or(format!("{}-{}", arg_path, "1"));

    let output_path = Path::new(&output_path);
    println!("{:?}", &output_path);

    // If output dir does not exist, create it
    if let Err(_) = fs::read_dir(&output_path) {
        fs::create_dir(&output_path).unwrap();
    }

    write_files(&entries, output_path).unwrap();
    println!("Completed Sucessfully");
}
