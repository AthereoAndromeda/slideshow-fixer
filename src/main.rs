use std::{fs, io, path::Path};

use clap::Parser;
use slideshow_fixer::write_files;

/// A simple utility program to fix slideshow sorting order cuz our TV is shit
#[derive(Debug, Parser)]
struct Args {
    /// Path of folder
    path: String,

    /// Optional output folder. Defaults to <PATH>-1
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();
    let mut entries = Vec::new();

    // Check if dir exists
    let input_dir = match fs::read_dir(&args.path) {
        Ok(f) => f,
        Err(e) => {
            panic!("{}", e);
        }
    };

    for file in input_dir {
        match file {
            Ok(f) => entries.push(f),
            Err(e) => eprintln!("Error opening file: {}", e),
        }
    }

    // sort numerically
    entries.sort_by(|a, b| a.path().cmp(&b.path()));

    // Remove trailing /
    let arg_path = args.path.strip_suffix("/").unwrap_or(&args.path);

    // Create output path name
    let output_path_name = args.output.unwrap_or(format!("{}-{}", arg_path, "1"));
    let output_path = Path::new(&output_path_name);

    match fs::read_dir(output_path) {
        // If output dir already exists, prompt overwrite
        Ok(_) => prompt_overwrite(output_path),

        // If output dir does not exist, create it. If it's any other error, panic
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                fs::create_dir(output_path).unwrap();
            } else {
                panic!("{}", e);
            }
        }
    }

    write_files(&entries, output_path).unwrap();
    println!("Completed Sucessfully");
}

pub fn prompt_overwrite(output_path: &Path) {
    println!(
        "{} already exists. Overwrite directory? (Will erase ALL content of directory) [Y/n]",
        output_path.display()
    );

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let buf = buf.trim();

    if buf != "Y" {
        println!("Aborting...");
        std::process::exit(1)
    } else {
        println!("Overwriting...");
        fs::remove_dir_all(output_path).unwrap();
        fs::create_dir(output_path).unwrap();
    }
}
