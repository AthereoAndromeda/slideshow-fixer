use std::io::Write;
use std::{fs, io, path::Path};

use clap::Parser;
use slideshow_fixer::zip_main;
use slideshow_fixer::{write_files, MyZipError};
use zip::result::ZipError;

/// A simple utility program to fix slideshow sorting order cuz our TV is shit
#[derive(Debug, Parser)]
struct Args {
    /// Path of folder
    path: String,

    /// Optional output folder. Defaults to <PATH>-1
    #[arg(short, long)]
    output: Option<String>,
}

fn process_zip_file(args: &Args) {
    let file_res = fs::File::open(&args.path);

    let file = match file_res {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };

    let zip_file_res = zip_main(file);

    // Error handling
    let zip_file = match zip_file_res {
        Ok(res) => res,

        Err(err) => match err {
            // Errors from \`zip\` lib
            MyZipError::ZipError(err) => {
                // Check if Invalid or Unsupported. Other errors (I/O) panics
                match err {
                    ZipError::InvalidArchive(s) => {
                        eprintln!(
                            "Not an actual ZIP file! Check for corruption or if it actually is a ZipFile"
                        );
                        eprintln!("{s}");
                        std::process::exit(1);
                    }
                    ZipError::UnsupportedArchive(s) => {
                        eprintln!("ZIP File not supported. Deflated algorithm is only supported");
                        eprintln!("{s}");
                        std::process::exit(1);
                    }
                    _ => panic!("ZIP Error: {err}"),
                }
            }

            // Errors from I/O like reading or writing from files
            MyZipError::IoError(err) => {
                eprintln!("I/O Error");
                eprintln!("{err}");
                std::process::exit(1);
            }
        },
    };

    let outpath = match &args.output {
        Some(output) => output.to_owned(),

        None => {
            let default_name = Path::new(&args.path);

            // turn `filename` into `filename-1.zip`
            let file_stem = default_name.file_stem().unwrap();
            let new_file_stem = file_stem.to_str().unwrap().to_string() + "-1.zip";

            // Attach parent to `file-1`
            let parent = default_name.parent().unwrap();
            let default_path = parent.join(new_file_stem);
            let default_path_string = default_path.display().to_string();

            // println!("{}", &default_path.display());

            default_path_string
        }
    };

    let mut outfile = fs::File::create(&outpath).unwrap();
    outfile.write_all(&zip_file).unwrap();

    println!("File written at {}", &outpath);
}

fn main() {
    let args = Args::parse();
    let mut entries = Vec::new();

    if args.path.ends_with(".zip") {
        process_zip_file(&args);
        return;
    }

    // Check if dir exists
    let input_dir = match fs::read_dir(&args.path) {
        Ok(f) => f,
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                eprintln!("Cannot Find Directory!\n{e}");
                std::process::exit(1);
            }

            _ => panic!("{}", e),
        },
    };

    for file in input_dir {
        match file {
            Ok(f) => entries.push(f),
            Err(e) => eprintln!("Error opening file: {e}"),
        }
    }

    // sort numerically
    entries.sort_by_key(|a| a.path());

    // Remove trailing /
    let arg_path = args.path.strip_suffix('/').unwrap_or(&args.path);

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
