use super::MyFile;
use super::MyZipError;
use std::io::{Read, Seek};
use zip::ZipArchive;

pub fn extract<R: Read + Seek>(reader: R) -> Result<Vec<MyFile>, MyZipError> {
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
            #[cfg(debug_assertions)]
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );

            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;

            let my_file = MyFile {
                name: outpath.file_name().unwrap().to_string_lossy().to_string(),
                buf: buf.into_boxed_slice(),
            };

            files.push(my_file);
        }
    }

    Ok(files)
}
