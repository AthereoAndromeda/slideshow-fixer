use std::io::{Read, Seek};

mod archive;
mod extract;
mod my_error;
mod my_file;

pub use archive::zip_archive;
pub use extract::zip_extract;
pub use my_error::MyZipError;
pub use my_file::MyFile;

pub fn zip_sort(files: &mut Vec<MyFile>) {
    files.sort_by(|a, b| a.name.cmp(&b.name));
}

pub fn zip_main<R: Read + Seek>(reader: R) -> Result<Box<[u8]>, MyZipError> {
    let mut files = zip_extract(reader)?;
    zip_sort(&mut files);
    zip_archive(&files)
}

#[cfg(test)]
mod test {
    use crate::{zip_archive, zip_extract, zip_sort, MyFile};
    use std::io::Cursor;

    const ZIP_TEST_DATA: &[u8; 547] = include_bytes!("./test.zip");

    #[test]
    #[allow(unused_allocation)]
    fn zip_extract_test() {
        let zip_file = Cursor::new(ZIP_TEST_DATA);
        let files = zip_extract(zip_file).unwrap();

        assert_eq!(files.len(), 3);

        // Check file names (unsorted)
        assert!(files.iter().any(|f| f.name == "01.txt"));
        assert!(files.iter().any(|f| f.name == "02.txt"));
        assert!(files.iter().any(|f| f.name == "03.txt"));

        // Check file content/buffer (unsorted)
        assert!(files.iter().any(|f| f.buf == Box::new([0x61, 0x61, 0xA])));
        assert!(files
            .iter()
            .any(|f| f.buf == Box::new([0x61, 0x66, 0x61, 0x73, 0x66, 0xA])));
        assert!(files
            .iter()
            .any(|f| f.buf == Box::new([0x61, 0x66, 0x61, 0x73, 0x66, 0xA])));
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
                buf: vec![0x61, 0x61, 0xA].into_boxed_slice(),
            },
            MyFile {
                name: "07.txt".to_string(),
                buf: vec![0x61, 0x66, 0x61, 0x23, 0x66, 0xA].into_boxed_slice(),
            },
            MyFile {
                name: "05.txt".to_string(),
                buf: vec![0x61, 0x66, 0x60, 0x73, 0x66, 0xA].into_boxed_slice(),
            },
        ];

        let zip_file = zip_archive(&files).unwrap();
        let buf = Cursor::new(zip_file);
        let mut files = zip_extract(buf).unwrap();

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
