use std::io::{Read, Seek};

mod archive;
mod extract;
mod my_error;
mod my_file;

pub use archive::archive;
pub use extract::extract;
pub use my_error::MyZipError;
pub use my_file::MyFile;

pub mod sort {
    use super::MyFile;

    pub fn sort(files: &mut [MyFile]) {
        files.sort_by(|a, b| a.name.cmp(&b.name));
    }
}

pub fn run_process<R: Read + Seek>(reader: R) -> Result<Box<[u8]>, MyZipError> {
    let mut files = extract(reader)?;
    sort::sort(&mut files);
    archive(&files)
}

#[cfg(test)]
mod test {
    use super::sort::sort;
    use super::{archive, extract, MyFile};
    use std::io::Cursor;

    const ZIP_TEST_DATA: &[u8; 547] = include_bytes!("./test.zip");

    #[test]
    #[allow(unused_allocation)]
    fn zip_extract_test() {
        let zip_file = Cursor::new(ZIP_TEST_DATA);
        let files = extract(zip_file).unwrap();

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
        let mut files = extract(zip_file).unwrap();

        sort(&mut files);

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

        let zip_file = archive(&files).unwrap();
        let buf = Cursor::new(zip_file);
        let mut files = extract(buf).unwrap();

        // Check file names (unsorted)
        assert!(files.iter().any(|f| f.name == "04.txt"));
        assert!(files.iter().any(|f| f.name == "07.txt"));
        assert!(files.iter().any(|f| f.name == "05.txt"));

        sort(&mut files);

        assert_eq!(files[0].name, "04.txt");
        assert_eq!(files[1].name, "05.txt");
        assert_eq!(files[2].name, "07.txt");
    }
}
