use std::{
    fs::{self, DirEntry},
    io::{self, Cursor, Read, Seek, Write},
    path::Path,
};

use zip::{result::ZipResult, write::FileOptions, CompressionMethod, ZipArchive, ZipWriter};

pub fn write_files(entries: &Vec<DirEntry>, output_path: &Path) -> Result<(), std::io::Error> {
    for entry in entries {
        let new_path = output_path.join(entry.file_name());

        // This copies the entire file to memory. Use caution with big files
        let content = fs::read(entry.path())?;
        fs::write(new_path, content)?;
    }

    Ok(())
}

#[derive(Debug)]
pub struct MyFile {
    name: String,
    buffer: Vec<u8>,
}

impl std::fmt::Display for MyFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({} bytes)", self.name, self.buffer.len())
    }
}

pub fn extract_zip<R: Read + Seek>(reader: R) -> Result<Vec<MyFile>, Box<dyn std::error::Error>> {
    let mut archive = ZipArchive::new(reader).unwrap();
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

            let my_file = MyFile {
                name: outpath.file_name().unwrap().to_string_lossy().to_string(),
                buffer: w_buf,
            };

            files.push(my_file);
        }
    }

    Ok(files)
}

pub fn sort_files(files: &mut Vec<MyFile>) {
    files.sort_by(|a, b| a.name.cmp(&b.name));
}

pub fn zip_files(files: &Vec<MyFile>) -> ZipResult<Cursor<Vec<u8>>> {
    let buf_writer = Cursor::new(Vec::new());
    let mut zip_writer = ZipWriter::new(buf_writer);

    let options = FileOptions::default().compression_method(CompressionMethod::Deflated);

    for file in files {
        zip_writer.start_file(&file.name, options)?;
        zip_writer.write_all(&file.buffer)?;
    }

    zip_writer.finish()
}

#[cfg(test)]
mod test {
    use crate::{extract_zip, zip_files, MyFile};
    use std::{fs::File, path::Path};

    #[test]
    pub fn zip_extract() {
        let path = Path::new("./test/e.zip");
        let zip_file = File::open(path).unwrap();
        let files = extract_zip(zip_file).unwrap();

        assert_eq!(files.len(), 4);
        println!("{}", files[0]);
    }

    #[test]
    pub fn write_test() {
        let files = vec![
            MyFile {
                name: "01.jpg".to_string(),
                buffer: vec![0, 32, 155, 13],
            },
            MyFile {
                name: "02.jpg".to_string(),
                buffer: vec![64, 22, 155, 13],
            },
            MyFile {
                name: "03.jpg".to_string(),
                buffer: vec![100, 52, 55, 123],
            },
        ];

        let a = zip_files(&files).unwrap();
    }
}
