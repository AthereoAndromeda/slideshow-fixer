use chrono::{Datelike, Timelike};
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

use super::ZipMainResult;
use crate::MyFile;
use std::io::{Cursor, Write};

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
