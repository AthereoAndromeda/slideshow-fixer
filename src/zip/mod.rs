use chrono::{Datelike, Timelike};
use std::io::{self, Cursor, Read, Seek, Write};
use zip::{result::ZipResult, write::FileOptions, CompressionMethod, ZipArchive, ZipWriter};

#[derive(Debug)]
pub struct MyFile {
    name: String,
    buf: Vec<u8>,
}

impl std::fmt::Display for MyFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({} bytes)", self.name, self.buf.len())
    }
}

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

pub fn zip_sort(files: &mut Vec<MyFile>) {
    files.sort_by(|a, b| a.name.cmp(&b.name));
}

pub fn zip_archive(files: &Vec<MyFile>) -> ZipMainResult {
    let buf_writer = Cursor::new(Vec::new());
    let mut zip_writer = ZipWriter::new(buf_writer);

    let date_time = chrono::offset::Utc::now();
    let base_options = FileOptions::default().compression_method(CompressionMethod::Deflated);

    for i in 0..files.len() {
        let file = &files[i];

        // Increment `Date Modified` by 1 second for each successive file
        let date_time = date_time + chrono::Duration::seconds(i as i64);

        let zip_date_time = zip::DateTime::from_date_and_time(
            date_time.year() as u16,
            date_time.month() as u8,
            date_time.day() as u8,
            date_time.hour() as u8,
            date_time.minute() as u8,
            date_time.second() as u8,
        )
        .unwrap();

        let options = base_options.last_modified_time(zip_date_time);
        zip_writer.start_file(&file.name, options)?;
        zip_writer.write_all(&file.buf)?;
    }

    zip_writer.finish()
}

type ZipMainResult = ZipResult<Cursor<Vec<u8>>>;
pub fn zip_main<R: Read + Seek>(reader: R) -> ZipMainResult {
    let mut files = zip_extract(reader).unwrap();
    zip_sort(&mut files);
    zip_archive(&files)
}

#[cfg(test)]
mod test {
    use crate::{zip_archive, zip_extract, zip_sort, MyFile};
    use std::io::Cursor;

    const ZIP_TEST_DATA: [u8; 547] = [
        80, 75, 3, 4, 20, 0, 8, 0, 8, 0, 104, 189, 74, 85, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 6,
        0, 32, 0, 48, 49, 46, 116, 120, 116, 85, 84, 13, 0, 7, 148, 61, 68, 99, 148, 61, 68, 99,
        148, 61, 68, 99, 117, 120, 11, 0, 1, 4, 232, 3, 0, 0, 4, 232, 3, 0, 0, 75, 76, 228, 2, 0,
        80, 75, 7, 8, 253, 203, 103, 42, 5, 0, 0, 0, 3, 0, 0, 0, 80, 75, 3, 4, 20, 0, 8, 0, 8, 0,
        106, 189, 74, 85, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 6, 0, 32, 0, 48, 50, 46, 116, 120,
        116, 85, 84, 13, 0, 7, 152, 61, 68, 99, 152, 61, 68, 99, 157, 61, 68, 99, 117, 120, 11, 0,
        1, 4, 232, 3, 0, 0, 4, 232, 3, 0, 0, 75, 76, 75, 44, 78, 227, 2, 0, 80, 75, 7, 8, 225, 156,
        71, 98, 8, 0, 0, 0, 6, 0, 0, 0, 80, 75, 3, 4, 20, 0, 8, 0, 8, 0, 114, 189, 74, 85, 0, 0, 0,
        0, 0, 0, 0, 0, 6, 0, 0, 0, 6, 0, 32, 0, 48, 51, 46, 116, 120, 116, 85, 84, 13, 0, 7, 169,
        61, 68, 99, 169, 61, 68, 99, 169, 61, 68, 99, 117, 120, 11, 0, 1, 4, 232, 3, 0, 0, 4, 232,
        3, 0, 0, 75, 76, 75, 44, 78, 227, 2, 0, 80, 75, 7, 8, 225, 156, 71, 98, 8, 0, 0, 0, 6, 0,
        0, 0, 80, 75, 1, 2, 20, 3, 20, 0, 8, 0, 8, 0, 104, 189, 74, 85, 253, 203, 103, 42, 5, 0, 0,
        0, 3, 0, 0, 0, 6, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 180, 129, 0, 0, 0, 0, 48, 49, 46, 116,
        120, 116, 85, 84, 13, 0, 7, 148, 61, 68, 99, 148, 61, 68, 99, 148, 61, 68, 99, 117, 120,
        11, 0, 1, 4, 232, 3, 0, 0, 4, 232, 3, 0, 0, 80, 75, 1, 2, 20, 3, 20, 0, 8, 0, 8, 0, 106,
        189, 74, 85, 225, 156, 71, 98, 8, 0, 0, 0, 6, 0, 0, 0, 6, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        180, 129, 89, 0, 0, 0, 48, 50, 46, 116, 120, 116, 85, 84, 13, 0, 7, 152, 61, 68, 99, 152,
        61, 68, 99, 157, 61, 68, 99, 117, 120, 11, 0, 1, 4, 232, 3, 0, 0, 4, 232, 3, 0, 0, 80, 75,
        1, 2, 20, 3, 20, 0, 8, 0, 8, 0, 114, 189, 74, 85, 225, 156, 71, 98, 8, 0, 0, 0, 6, 0, 0, 0,
        6, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 180, 129, 181, 0, 0, 0, 48, 51, 46, 116, 120, 116, 85,
        84, 13, 0, 7, 169, 61, 68, 99, 169, 61, 68, 99, 169, 61, 68, 99, 117, 120, 11, 0, 1, 4,
        232, 3, 0, 0, 4, 232, 3, 0, 0, 80, 75, 5, 6, 0, 0, 0, 0, 3, 0, 3, 0, 252, 0, 0, 0, 17, 1,
        0, 0, 0, 0,
    ];

    #[test]
    fn zip_extract_test() {
        let zip_file = Cursor::new(ZIP_TEST_DATA);
        let files = zip_extract(zip_file).unwrap();

        assert_eq!(files.len(), 3);

        // Check file names (unsorted)
        assert!(files.iter().any(|f| f.name == "01.txt"));
        assert!(files.iter().any(|f| f.name == "02.txt"));
        assert!(files.iter().any(|f| f.name == "03.txt"));

        // Check file content/buffer (unsorted)
        assert!(files.iter().any(|f| f.buf == [0x61, 0x61, 0xA]));
        assert!(files
            .iter()
            .any(|f| f.buf == [0x61, 0x66, 0x61, 0x73, 0x66, 0xA]));
        assert!(files
            .iter()
            .any(|f| f.buf == [0x61, 0x66, 0x61, 0x73, 0x66, 0xA]));
    }

    #[test]
    fn zip_sort_test() {
        let zip_file = Cursor::new(ZIP_TEST_DATA);
        let mut files = zip_extract(zip_file).unwrap();

        zip_sort(&mut files);

        assert_eq!(files[0].name, "01.txt");
        assert_eq!(files[1].name, "02.txt");
        assert_eq!(files[2].name, "03.txt");
    }

    #[test]
    fn write_test() {
        let files = vec![
            MyFile {
                name: "04.txt".to_string(),
                buf: vec![0x61, 0x61, 0xA],
            },
            MyFile {
                name: "07.txt".to_string(),
                buf: vec![0x61, 0x66, 0x61, 0x23, 0x66, 0xA],
            },
            MyFile {
                name: "05.txt".to_string(),
                buf: vec![0x61, 0x66, 0x60, 0x73, 0x66, 0xA],
            },
        ];

        let zip_file = zip_archive(&files).unwrap();
        let mut files = zip_extract(zip_file).unwrap();

        // Check file names (unsorted)
        assert!(files.iter().any(|f| f.name == "04.txt"));
        assert!(files.iter().any(|f| f.name == "07.txt"));
        assert!(files.iter().any(|f| f.name == "05.txt"));

        zip_sort(&mut files);

        assert_eq!(files[0].name, "04.txt");
        assert_eq!(files[1].name, "05.txt");
        assert_eq!(files[2].name, "07.txt");
    }
}
