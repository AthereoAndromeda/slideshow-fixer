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
    use std::{sync::Arc, thread};
    let mut handles = Vec::new();
    let output_path_arc = Arc::new(output_path);

    for entry in entries {
        let output_path_counter = Arc::clone(&output_path_arc);

        let handle = thread::spawn(move || {
            let new_path = output_path_counter.join(entry.file_name());

            let content = fs::read(entry.path()).unwrap();
            fs::write(new_path, content).unwrap();
        });

        handles.push(handle);
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    Ok(())
}
