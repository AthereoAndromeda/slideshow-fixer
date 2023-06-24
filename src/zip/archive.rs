use chrono::{Datelike, Timelike, Utc};
use zip::{write::FileOptions, CompressionMethod, ZipWriter};

use super::MyFile;
use super::MyZipError;
use std::io::{Cursor, Write};

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
/// Helper function to both convert `chrono::DateTime` to `zip::DateTime`
/// and adds `i` seconds to `DateTime`.
///
/// This helps sort files by `Date Modified`.
fn get_date_time(date_time: chrono::DateTime<Utc>, i: usize) -> zip::DateTime {
    // Increment `Date Modified` by 1 second for each successive file
    let date_time = date_time + chrono::Duration::seconds(i as i64);

    // Convert chrono::DateTime to zip::DateTime since the zip lib
    // im using does not support chrono
    zip::DateTime::from_date_and_time(
        date_time.year() as u16,
        date_time.month() as u8,
        date_time.day() as u8,
        date_time.hour() as u8,
        date_time.minute() as u8,
        date_time.second() as u8,
    )
    .unwrap()
}

pub fn archive(files: &[MyFile]) -> Result<Box<[u8]>, MyZipError> {
    let buffer = Cursor::new(Vec::new());
    let mut zip_writer = ZipWriter::new(buffer);

    let date_time = chrono::offset::Utc::now();
    let base_options = FileOptions::default().compression_method(CompressionMethod::Deflated);

    for (i, file) in files.iter().enumerate() {
        let zip_date_time = get_date_time(date_time, i);
        let options = base_options.last_modified_time(zip_date_time);

        zip_writer.start_file(&file.name, options)?;
        zip_writer.write_all(&file.buf)?;

        #[cfg(debug_assertions)]
        println!("File {i} written");
    }

    let zip_buffer = zip_writer.finish()?;
    Ok(zip_buffer.into_inner().into_boxed_slice())
}
