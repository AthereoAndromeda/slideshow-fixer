use std::{
    fs::{self, DirEntry, File},
    io,
    path::Path,
};

use zip::ZipArchive;

pub fn write_files(entries: &Vec<DirEntry>, output_path: &Path) -> Result<(), std::io::Error> {
    for entry in entries {
        let new_path = output_path.join(entry.file_name());

        // This copies the entire file to memory. Use caution with big files
        let content = fs::read(entry.path())?;
        fs::write(new_path, content)?;
    }

    Ok(())
}

pub fn extract_zip<'a>(zip_file: File) -> Result<Vec<Vec<u8>>, Box<dyn std::error::Error>> {
    let mut archive = ZipArchive::new(zip_file).unwrap();
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

            let mut w_buf = Vec::new();
            io::copy(&mut file, &mut w_buf)?;

            files.push(w_buf);
        }
    }

    let a = files.iter().map(|f| f.as_slice()).collect::<Vec<_>>();

    Ok(files)
}

#[cfg(test)]
mod test {
    use super::extract_zip;
    use std::{fs::File, path::Path};

    #[test]
    pub fn zip_extract() {
        let path = Path::new("./test/e.zip");
        let zip_file = File::open(path).unwrap();
        let a = extract_zip(zip_file).unwrap();

        println!("{:?}", a);
    }
}
