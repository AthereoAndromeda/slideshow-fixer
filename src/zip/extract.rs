use crate::MyFile;
use std::io::{self, Read, Seek};
use zip::ZipArchive;

pub fn zip_extract<R: Read + Seek>(reader: R) -> Result<Vec<MyFile>, Box<dyn std::error::Error>> {
    let mut archive = ZipArchive::new(reader)?;
    let mut files = Vec::with_capacity(archive.file_names().count());

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if (*file.name()).ends_with('/') {
            println!("Ignoring subdirectory {}", outpath.display());
            // println!("File {} extracted to \"{}\"", i, outpath.display());
            // fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );

            let mut buf_writer = Vec::new();
            io::copy(&mut file, &mut buf_writer)?;

            let my_file = MyFile {
                name: outpath.file_name().unwrap().to_string_lossy().to_string(),
                buf: buf_writer,
            };

            files.push(my_file);
        }
    }

    Ok(files)
}