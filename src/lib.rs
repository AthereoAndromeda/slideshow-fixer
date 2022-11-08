mod zip;
pub use crate::zip::*;

#[cfg(feature = "wasm")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

#[cfg(not(target_arch = "wasm32"))]
pub fn write_files(entries: Vec<DirEntry>, output_path: PathBuf) -> Result<(), std::io::Error> {
    use filetime::{set_file_mtime, FileTime};
    use std::{sync::Arc, thread};

    let mut handles = Vec::new();
    let output_path_arc = Arc::new(output_path);

    let now = FileTime::now();
    let now = now.unix_seconds();

    for (i, entry) in entries.into_iter().enumerate() {
        let output_path_counter = Arc::clone(&output_path_arc);

        let handle = thread::spawn(move || {
            let new_path = output_path_counter.join(entry.file_name());

            let content = fs::read(entry.path()).unwrap();
            fs::write(&new_path, content).unwrap();

            let new_time = FileTime::from_unix_time(now + i as i64, 0);
            set_file_mtime(new_path, new_time).unwrap();
        });

        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    Ok(())
}
