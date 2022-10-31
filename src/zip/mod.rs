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
